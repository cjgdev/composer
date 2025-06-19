//! Trie-based pattern storage and matching for chord progressions
//!
//! Implements high-performance trie data structure for storing and querying
//! chord progression patterns with statistical analysis capabilities.

use crate::error::{AiError, AiResult};
use ahash::AHashMap;
use composer_core::Chord;
use composer_serialization::{serialize_chord, ChordBinary};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;

/// Reference to source data (song/progression)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    /// Source identifier (song ID, file hash, etc.)
    pub id: String,
    /// Original key tonic (optional)
    pub key_tonic: Option<String>,
}

/// Trie node for chord progression storage
#[derive(Debug, Clone)]
pub struct TrieNode {
    /// Occurrence frequency at this node
    pub count: u32,

    /// Child nodes indexed by serialized chord keys
    pub children: AHashMap<ChordBinary, TrieNode>,

    /// References to source data
    pub id_list: SmallVec<[Reference; 4]>,

    /// Popularity ranking (1-based, calculated)
    pub rank: u32,

    /// Whether to include key tonic in references
    pub include_key_tonic: bool,
}

/// Chord progression trie for pattern storage
#[derive(Debug)]
pub struct ChordProgressionTrie {
    /// Root node
    root: Arc<RwLock<TrieNode>>,

    /// Scale-specific branches
    scale_branches: Arc<RwLock<AHashMap<String, TrieNode>>>,

    /// Total patterns stored
    total_patterns: Arc<RwLock<u64>>,

    /// Memory usage tracking
    memory_usage_bytes: Arc<RwLock<u64>>,
}

/// Pattern search result
#[derive(Debug, Clone)]
pub struct PatternResult {
    /// Serialized chord that matches
    pub serialized_chord: ChordBinary,

    /// Frequency count
    pub count: u32,

    /// Popularity rank
    pub rank: u32,

    /// Relative frequency (0.0-1.0)
    pub relative_count: f64,

    /// Source references
    pub id_list: Vec<Reference>,

    /// Computed relevance weight
    pub weight: f64,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            count: 0,
            children: AHashMap::new(),
            id_list: SmallVec::new(),
            rank: 0,
            include_key_tonic: false,
        }
    }
}

impl TrieNode {
    /// Create a new trie node
    pub fn new(include_key_tonic: bool) -> Self {
        Self {
            include_key_tonic,
            ..Default::default()
        }
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> u64 {
        let base_size = std::mem::size_of::<Self>() as u64;
        let children_size = self.children.capacity() as u64
            * (std::mem::size_of::<ChordBinary>() + std::mem::size_of::<TrieNode>()) as u64;
        let id_list_size = self.id_list.capacity() as u64 * std::mem::size_of::<Reference>() as u64;

        base_size
            + children_size
            + id_list_size
            + self
                .children
                .values()
                .map(|child| child.memory_usage())
                .sum::<u64>()
    }

    /// Calculate ranks for all children based on count
    pub fn calculate_ranks(&mut self) {
        // Collect keys and counts separately to avoid borrowing conflicts
        let mut key_count_pairs: Vec<(ChordBinary, u32)> = self
            .children
            .iter()
            .map(|(key, node)| (*key, node.count))
            .collect();

        // Sort by count descending
        key_count_pairs.sort_by(|a, b| b.1.cmp(&a.1));

        // Assign ranks
        for (rank, (key, _)) in key_count_pairs.iter().enumerate() {
            if let Some(child) = self.children.get_mut(key) {
                child.rank = (rank + 1) as u32;
                child.calculate_ranks(); // Recursively calculate for children
            }
        }
    }
}

impl ChordProgressionTrie {
    /// Create a new chord progression trie
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(TrieNode::new(false))),
            scale_branches: Arc::new(RwLock::new(AHashMap::new())),
            total_patterns: Arc::new(RwLock::new(0)),
            memory_usage_bytes: Arc::new(RwLock::new(0)),
        }
    }

    /// Add a chord progression pattern to the trie
    pub fn add_pattern(
        &self,
        pattern: &[Chord],
        source_id: String,
        key_tonic: Option<String>,
    ) -> AiResult<()> {
        if pattern.is_empty() {
            return Err(AiError::InvalidPattern {
                reason: "Pattern cannot be empty".to_string(),
            });
        }

        if pattern.len() > 20 {
            return Err(AiError::InvalidPattern {
                reason: format!("Pattern too long: {} (max: {})", pattern.len(), 20),
            });
        }

        // Serialize the pattern
        let mut serialized_pattern = Vec::new();
        for chord in pattern {
            let binary = serialize_chord(chord).map_err(|e| AiError::InvalidPattern {
                reason: format!("Chord serialization failed: {}", e),
            })?;
            serialized_pattern.push(binary);
        }

        // Add to the trie
        let reference = Reference {
            id: source_id,
            key_tonic,
        };

        self.add_serialized_pattern(&serialized_pattern, reference)?;

        // Update counters
        *self.total_patterns.write() += 1;

        Ok(())
    }

    /// Add a serialized pattern to the trie
    fn add_serialized_pattern(
        &self,
        pattern: &[ChordBinary],
        reference: Reference,
    ) -> AiResult<()> {
        // Simple recursive implementation
        self.add_pattern_recursive(&mut self.root.write(), pattern, 0, &reference)
    }

    /// Recursively add pattern to trie
    fn add_pattern_recursive(
        &self,
        node: &mut TrieNode,
        pattern: &[ChordBinary],
        depth: usize,
        reference: &Reference,
    ) -> AiResult<()> {
        // Update current node
        node.count += 1;
        if !node.id_list.iter().any(|r| r.id == reference.id) {
            node.id_list.push(reference.clone());
        }

        // If we've processed the entire pattern, we're done
        if depth >= pattern.len() {
            return Ok(());
        }

        // Get the next chord in the pattern
        let chord_binary = pattern[depth];

        // Create child if it doesn't exist
        if !node.children.contains_key(&chord_binary) {
            node.children
                .insert(chord_binary, TrieNode::new(node.include_key_tonic));
        }

        // Recursively add to child
        let child = node.children.get_mut(&chord_binary).unwrap();
        self.add_pattern_recursive(child, pattern, depth + 1, reference)
    }

    /// Search for patterns with wildcard support
    pub fn search_with_wildcard(
        &self,
        pattern: &[Option<Chord>],
        limit: usize,
    ) -> AiResult<Vec<PatternResult>> {
        let mut results = Vec::new();

        // Convert pattern to binary with wildcards
        let mut binary_pattern = Vec::new();
        for chord_opt in pattern {
            match chord_opt {
                Some(chord) => {
                    let binary = serialize_chord(chord).map_err(|e| AiError::SuggestionFailed {
                        reason: format!("Chord serialization failed: {}", e),
                    })?;
                    binary_pattern.push(Some(binary));
                },
                None => {
                    binary_pattern.push(None);
                },
            }
        }

        // Search the trie
        self.search_trie_recursive(&self.root.read(), &binary_pattern, 0, &mut results, limit)?;

        // Sort by weight descending
        results.sort_by(|a, b| {
            b.weight
                .partial_cmp(&a.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit results
        results.truncate(limit);

        Ok(results)
    }

    /// Recursive trie search helper
    fn search_trie_recursive(
        &self,
        node: &TrieNode,
        pattern: &[Option<ChordBinary>],
        pattern_index: usize,
        results: &mut Vec<PatternResult>,
        limit: usize,
    ) -> AiResult<()> {
        if results.len() >= limit {
            return Ok(());
        }

        if pattern_index >= pattern.len() {
            // We've matched the entire pattern
            return Ok(());
        }

        match &pattern[pattern_index] {
            Some(chord_binary) => {
                // Look for exact match
                if let Some(child) = node.children.get(chord_binary) {
                    if pattern_index == pattern.len() - 1 {
                        // End of pattern, add result
                        results.push(PatternResult {
                            serialized_chord: *chord_binary,
                            count: child.count,
                            rank: child.rank,
                            relative_count: child.count as f64 / node.count.max(1) as f64,
                            id_list: child.id_list.to_vec(),
                            weight: self.calculate_weight(child.count, child.rank),
                        });
                    } else {
                        // Continue searching
                        self.search_trie_recursive(
                            child,
                            pattern,
                            pattern_index + 1,
                            results,
                            limit,
                        )?;
                    }
                }
            },
            None => {
                // Wildcard - try all children
                for (chord_binary, child) in &node.children {
                    if pattern_index == pattern.len() - 1 {
                        // End of pattern, add result
                        results.push(PatternResult {
                            serialized_chord: *chord_binary,
                            count: child.count,
                            rank: child.rank,
                            relative_count: child.count as f64 / node.count.max(1) as f64,
                            id_list: child.id_list.to_vec(),
                            weight: self.calculate_weight(child.count, child.rank),
                        });
                    } else {
                        // Continue searching
                        self.search_trie_recursive(
                            child,
                            pattern,
                            pattern_index + 1,
                            results,
                            limit,
                        )?;
                    }

                    if results.len() >= limit {
                        break;
                    }
                }
            },
        }

        Ok(())
    }

    /// Calculate weight for a pattern result
    fn calculate_weight(&self, count: u32, rank: u32) -> f64 {
        let frequency_score = count as f64;
        let rank_penalty = if rank > 0 { 1.0 / rank as f64 } else { 1.0 };

        frequency_score * rank_penalty
    }

    /// Get scale-specific branch
    pub fn get_scale_branch(&self, scale_name: &str) -> Option<TrieNode> {
        self.scale_branches.read().get(scale_name).cloned()
    }

    /// Add scale-specific branch
    pub fn add_scale_branch(&self, scale_name: String, branch: TrieNode) {
        self.scale_branches.write().insert(scale_name, branch);
    }

    /// Get total number of patterns
    pub fn total_patterns(&self) -> u64 {
        *self.total_patterns.read()
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> u64 {
        let root_usage = self.root.read().memory_usage();
        let branches_usage: u64 = self
            .scale_branches
            .read()
            .values()
            .map(|branch| branch.memory_usage())
            .sum();

        root_usage + branches_usage
    }

    /// Calculate ranks for all nodes
    pub fn calculate_all_ranks(&self) {
        self.root.write().calculate_ranks();

        for branch in self.scale_branches.write().values_mut() {
            branch.calculate_ranks();
        }
    }

    /// Get statistics about the trie
    pub fn statistics(&self) -> TrieStatistics {
        let root = self.root.read();
        let total_patterns = self.total_patterns();
        let memory_usage = self.memory_usage();
        let scale_branches = self.scale_branches.read().len();

        TrieStatistics {
            total_patterns,
            total_nodes: self.count_nodes(&root),
            memory_usage_bytes: memory_usage,
            scale_branches,
            max_depth: self.calculate_max_depth(&root, 0),
            avg_branching_factor: self.calculate_avg_branching_factor(&root),
        }
    }

    /// Count total nodes in trie
    fn count_nodes(&self, node: &TrieNode) -> u64 {
        1 + node
            .children
            .values()
            .map(|child| self.count_nodes(child))
            .sum::<u64>()
    }

    /// Calculate maximum depth
    fn calculate_max_depth(&self, node: &TrieNode, current_depth: u32) -> u32 {
        if node.children.is_empty() {
            current_depth
        } else {
            node.children
                .values()
                .map(|child| self.calculate_max_depth(child, current_depth + 1))
                .max()
                .unwrap_or(current_depth)
        }
    }

    /// Calculate average branching factor
    fn calculate_avg_branching_factor(&self, node: &TrieNode) -> f64 {
        let mut total_branches = 0u64;
        let mut total_nodes = 0u64;

        self.collect_branching_stats(node, &mut total_branches, &mut total_nodes);

        if total_nodes > 0 {
            total_branches as f64 / total_nodes as f64
        } else {
            0.0
        }
    }

    /// Collect branching statistics recursively
    fn collect_branching_stats(
        &self,
        node: &TrieNode,
        total_branches: &mut u64,
        total_nodes: &mut u64,
    ) {
        *total_nodes += 1;
        *total_branches += node.children.len() as u64;

        for child in node.children.values() {
            self.collect_branching_stats(child, total_branches, total_nodes);
        }
    }
}

/// Trie statistics for monitoring and optimization
#[derive(Debug, Clone)]
pub struct TrieStatistics {
    pub total_patterns: u64,
    pub total_nodes: u64,
    pub memory_usage_bytes: u64,
    pub scale_branches: usize,
    pub max_depth: u32,
    pub avg_branching_factor: f64,
}

impl Default for ChordProgressionTrie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use composer_core::Chord;

    #[test]
    fn test_trie_creation() {
        let trie = ChordProgressionTrie::new();
        assert_eq!(trie.total_patterns(), 0);
        assert!(trie.memory_usage() > 0);
    }

    #[test]
    fn test_add_pattern() {
        let trie = ChordProgressionTrie::new();

        let pattern = vec![
            Chord::new(1, 5).unwrap(), // I
            Chord::new(5, 7).unwrap(), // V7
            Chord::new(6, 5).unwrap(), // vi
        ];

        let result = trie.add_pattern(&pattern, "test_song_1".to_string(), Some("C".to_string()));
        assert!(result.is_ok());
        assert_eq!(trie.total_patterns(), 1);
    }

    #[test]
    fn test_pattern_search() {
        let trie = ChordProgressionTrie::new();

        // Add some patterns
        let pattern1 = vec![
            Chord::new(1, 5).unwrap(), // I
            Chord::new(5, 7).unwrap(), // V7
        ];

        let pattern2 = vec![
            Chord::new(1, 5).unwrap(), // I
            Chord::new(4, 5).unwrap(), // IV
        ];

        trie.add_pattern(&pattern1, "song1".to_string(), None)
            .unwrap();
        trie.add_pattern(&pattern2, "song2".to_string(), None)
            .unwrap();

        // Search with wildcard
        let search_pattern = vec![
            Some(Chord::new(1, 5).unwrap()), // I
            None,                            // Wildcard
        ];

        let results = trie.search_with_wildcard(&search_pattern, 10).unwrap();
        assert_eq!(results.len(), 2); // Should find both V7 and IV
    }

    #[test]
    fn test_invalid_pattern() {
        let trie = ChordProgressionTrie::new();

        // Empty pattern should fail
        let result = trie.add_pattern(&[], "test".to_string(), None);
        assert!(result.is_err());

        // Pattern too long should fail
        let long_pattern: Vec<Chord> = (0..50).map(|_| Chord::new(1, 5).unwrap()).collect();
        let result = trie.add_pattern(&long_pattern, "test".to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_trie_statistics() {
        let trie = ChordProgressionTrie::new();

        let pattern = vec![
            Chord::new(1, 5).unwrap(),
            Chord::new(5, 7).unwrap(),
            Chord::new(6, 5).unwrap(),
        ];

        trie.add_pattern(&pattern, "song1".to_string(), None)
            .unwrap();

        let stats = trie.statistics();
        assert_eq!(stats.total_patterns, 1);
        assert!(stats.total_nodes > 1);
        assert!(stats.memory_usage_bytes > 0);
        assert!(stats.max_depth > 0);
    }

    #[test]
    fn test_memory_tracking() {
        let trie = ChordProgressionTrie::new();
        let initial_memory = trie.memory_usage();

        // Add a pattern
        let pattern = vec![Chord::new(1, 5).unwrap(), Chord::new(5, 7).unwrap()];

        trie.add_pattern(&pattern, "song1".to_string(), None)
            .unwrap();
        let after_memory = trie.memory_usage();

        assert!(after_memory > initial_memory);
    }

    #[test]
    fn test_rank_calculation() {
        let trie = ChordProgressionTrie::new();

        // Add patterns with different frequencies
        let common_pattern = vec![Chord::new(1, 5).unwrap(), Chord::new(5, 7).unwrap()];
        let rare_pattern = vec![Chord::new(1, 5).unwrap(), Chord::new(7, 5).unwrap()];

        // Add common pattern multiple times
        for i in 0..5 {
            trie.add_pattern(&common_pattern, format!("song{}", i), None)
                .unwrap();
        }

        // Add rare pattern once
        trie.add_pattern(&rare_pattern, "rare_song".to_string(), None)
            .unwrap();

        trie.calculate_all_ranks();

        // Search should rank common pattern higher
        let search_pattern = vec![Some(Chord::new(1, 5).unwrap()), None];
        let results = trie.search_with_wildcard(&search_pattern, 10).unwrap();

        assert!(results.len() >= 2);
        // First result should have higher count (common pattern)
        assert!(results[0].count >= results[1].count);
    }
}
