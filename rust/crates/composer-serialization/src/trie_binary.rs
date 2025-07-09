//! Binary trie serialization for chord progression data

use crate::error::{SerializationError, SerializationResult};
use std::collections::HashMap;

/// Trie node structure for serialization
#[derive(Debug, Clone)]
pub struct TrieNode {
    pub node_count: u32,
    pub id_list: Vec<u32>,
    pub children: HashMap<Vec<u8>, TrieNode>, // key is 5-byte chord binary
}

/// Child node entry for binary format
#[derive(Debug, Clone)]
pub struct ChildNodeEntry {
    pub data_length: u32,
    pub key: Vec<u8>, // 5-byte serialized chord key
    pub node_data: Vec<u8>,
}

/// Progress callback type for serialization
pub type ProgressCallback = Box<dyn Fn(f64) + Send + Sync>;

impl TrieNode {
    /// Create a new empty trie node
    pub fn new() -> Self {
        Self {
            node_count: 0,
            id_list: Vec::new(),
            children: HashMap::new(),
        }
    }

    /// Add a chord pattern to the trie
    pub fn add_pattern(&mut self, pattern: &[Vec<u8>], id: u32) {
        if pattern.is_empty() {
            self.id_list.push(id);
            self.node_count += 1;
            return;
        }

        let key = pattern[0].clone();
        let remaining = &pattern[1..];

        let child = self.children.entry(key).or_default();
        child.add_pattern(remaining, id);
    }

    /// Search for patterns in the trie
    pub fn search_patterns(&self, pattern: &[Vec<u8>]) -> Vec<u32> {
        if pattern.is_empty() {
            return self.id_list.clone();
        }

        if let Some(child) = self.children.get(&pattern[0]) {
            child.search_patterns(&pattern[1..])
        } else {
            Vec::new()
        }
    }

    /// Calculate the rank of this node based on count
    pub fn calculate_rank(&self, total_nodes: u32) -> u32 {
        if total_nodes == 0 {
            return 0;
        }
        // Simplified rank calculation - higher count = lower rank number
        std::cmp::max(1, total_nodes.saturating_sub(self.node_count))
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

/// Serialize a trie to binary format
pub fn serialize_trie(trie: &TrieNode) -> SerializationResult<Vec<u8>> {
    serialize_trie_with_progress(trie, None)
}

/// Serialize a trie with progress callback
pub fn serialize_trie_with_progress(
    trie: &TrieNode,
    progress_callback: Option<&ProgressCallback>,
) -> SerializationResult<Vec<u8>> {
    let mut buffer = Vec::new();

    // Calculate total nodes for progress reporting
    let total_nodes = count_total_nodes(trie);
    let mut processed_nodes = 0;

    serialize_node_recursive(
        trie,
        &mut buffer,
        &mut processed_nodes,
        total_nodes,
        progress_callback,
    )?;

    Ok(buffer)
}

/// Deserialize a trie from binary format
pub fn deserialize_trie(data: &[u8], include_key_tonic: bool) -> SerializationResult<TrieNode> {
    deserialize_trie_with_progress(data, include_key_tonic, None)
}

/// Deserialize a trie with progress callback
pub fn deserialize_trie_with_progress(
    data: &[u8],
    include_key_tonic: bool,
    progress_callback: Option<&ProgressCallback>,
) -> SerializationResult<TrieNode> {
    let mut offset = 0;
    let total_bytes = data.len();

    deserialize_node_recursive(
        data,
        &mut offset,
        include_key_tonic,
        total_bytes,
        progress_callback,
    )
}

/// Validate binary format
pub fn validate_binary_format(data: &[u8]) -> bool {
    // Basic validation checks
    if data.len() < 12 {
        return false; // Minimum size for header
    }

    // Check for basic structure integrity
    let mut offset = 0;
    if read_u32_be(data, &mut offset).is_err() {
        return false;
    }

    // Additional checks could be added here
    true
}

// Internal serialization functions

fn serialize_node_recursive(
    node: &TrieNode,
    buffer: &mut Vec<u8>,
    processed_nodes: &mut u32,
    total_nodes: u32,
    progress_callback: Option<&ProgressCallback>,
) -> SerializationResult<()> {
    // Serialize node count (4 bytes, big-endian)
    write_u32_be(buffer, node.node_count);

    // Serialize ID list
    write_u32_be(buffer, node.id_list.len() as u32);
    for &id in &node.id_list {
        write_u32_be(buffer, id);
    }

    // Serialize children
    write_u32_be(buffer, node.children.len() as u32);

    for (key, child) in &node.children {
        // Validate key length
        if key.len() != 5 {
            return Err(SerializationError::InvalidFormat {
                message: "Chord key must be exactly 5 bytes".to_string(),
            });
        }

        // Serialize child data to temporary buffer
        let mut child_buffer = Vec::new();
        serialize_node_recursive(
            child,
            &mut child_buffer,
            processed_nodes,
            total_nodes,
            progress_callback,
        )?;

        // Write data length, key, and child data
        write_u32_be(buffer, child_buffer.len() as u32);
        buffer.extend_from_slice(key);
        buffer.extend_from_slice(&child_buffer);
    }

    // Update progress
    *processed_nodes += 1;
    if let Some(callback) = progress_callback {
        let progress = *processed_nodes as f64 / total_nodes as f64;
        callback(progress);
    }

    Ok(())
}

fn deserialize_node_recursive(
    data: &[u8],
    offset: &mut usize,
    include_key_tonic: bool,
    total_bytes: usize,
    progress_callback: Option<&ProgressCallback>,
) -> SerializationResult<TrieNode> {
    // Read node count
    let node_count = read_u32_be(data, offset)?;

    // Read ID list
    let id_list_length = read_u32_be(data, offset)?;
    let mut id_list = Vec::with_capacity(id_list_length as usize);

    for _ in 0..id_list_length {
        let id = if include_key_tonic {
            // Read 6 bytes for key tonic support
            let mut id_bytes = [0u8; 6];
            read_bytes(data, offset, &mut id_bytes)?;
            u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]])
        } else {
            read_u32_be(data, offset)?
        };
        id_list.push(id);
    }

    // Read children
    let children_count = read_u32_be(data, offset)?;
    let mut children = HashMap::with_capacity(children_count as usize);

    for _ in 0..children_count {
        // Read child data length and key
        let _data_length = read_u32_be(data, offset)?;

        let mut key = vec![0u8; 5];
        read_bytes(data, offset, &mut key)?;

        // Recursively deserialize child
        let child = deserialize_node_recursive(
            data,
            offset,
            include_key_tonic,
            total_bytes,
            progress_callback,
        )?;
        children.insert(key, child);
    }

    // Update progress
    if let Some(callback) = progress_callback {
        let progress = *offset as f64 / total_bytes as f64;
        callback(progress);
    }

    Ok(TrieNode {
        node_count,
        id_list,
        children,
    })
}

// Utility functions

fn count_total_nodes(node: &TrieNode) -> u32 {
    let mut count = 1; // Count this node
    for child in node.children.values() {
        count += count_total_nodes(child);
    }
    count
}

fn write_u32_be(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_be_bytes());
}

fn read_u32_be(data: &[u8], offset: &mut usize) -> SerializationResult<u32> {
    if *offset + 4 > data.len() {
        return Err(SerializationError::UnexpectedEof);
    }

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[*offset..*offset + 4]);
    *offset += 4;

    Ok(u32::from_be_bytes(bytes))
}

fn read_bytes(data: &[u8], offset: &mut usize, buffer: &mut [u8]) -> SerializationResult<()> {
    if *offset + buffer.len() > data.len() {
        return Err(SerializationError::UnexpectedEof);
    }

    buffer.copy_from_slice(&data[*offset..*offset + buffer.len()]);
    *offset += buffer.len();

    Ok(())
}

/// Token vocabulary management
pub fn reduce_chord_vocab(
    chords: &[Vec<u8>], // Vec of 5-byte chord binaries
    max_vocab: usize,
) -> SerializationResult<Vec<Vec<u8>>> {
    // Count frequency of each unique chord
    let mut chord_counts: HashMap<Vec<u8>, u32> = HashMap::new();

    for chord in chords {
        *chord_counts.entry(chord.clone()).or_insert(0) += 1;
    }

    // Sort by frequency (descending)
    let mut sorted_chords: Vec<(Vec<u8>, u32)> = chord_counts.into_iter().collect();
    sorted_chords.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top maxVocab chords
    let top_chords: Vec<Vec<u8>> = sorted_chords
        .into_iter()
        .take(max_vocab)
        .map(|(chord, _)| chord)
        .collect();

    // Replace remaining chords with closest matches
    let result: Vec<Vec<u8>> = chords
        .iter()
        .map(|chord| {
            if top_chords.contains(chord) {
                chord.clone()
            } else {
                // Find closest match (simplified - could use more sophisticated matching)
                find_closest_chord(chord, &top_chords)
            }
        })
        .collect();

    Ok(result)
}

fn find_closest_chord(target: &[u8], candidates: &[Vec<u8>]) -> Vec<u8> {
    // Simplified closest match - just return first candidate
    // In practice, would implement sophisticated chord similarity matching
    candidates
        .first()
        .cloned()
        .unwrap_or_else(|| target.to_vec())
}

/// Token augmentation with repetition
pub fn augment_with_repeated<T: Clone>(sequence: Vec<T>, min_tokens: usize) -> Vec<T> {
    if sequence.is_empty() {
        return sequence;
    }

    let mut result = sequence.clone();

    // Repeat until we reach minimum token count
    while result.len() < min_tokens {
        // Add variation to avoid exact repetition
        let mut repeated = sequence.clone();

        // Simple variation: reverse every other repetition
        if result.len() / sequence.len() % 2 == 1 {
            repeated.reverse();
        }

        result.extend(repeated);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_node_creation() {
        let node = TrieNode::new();
        assert_eq!(node.node_count, 0);
        assert!(node.id_list.is_empty());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_add_pattern() {
        let mut root = TrieNode::new();
        let pattern = vec![vec![1, 2, 3, 4, 5], vec![2, 3, 4, 5, 6]];

        root.add_pattern(&pattern, 123);

        assert_eq!(root.children.len(), 1);
        assert!(root.children.contains_key(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut root = TrieNode::new();
        root.node_count = 5;
        root.id_list = vec![1, 2, 3];

        let serialized = serialize_trie(&root).unwrap();
        let deserialized = deserialize_trie(&serialized, false).unwrap();

        assert_eq!(root.node_count, deserialized.node_count);
        assert_eq!(root.id_list, deserialized.id_list);
    }

    #[test]
    fn test_validate_binary_format() {
        let mut root = TrieNode::new();
        root.node_count = 1;

        let valid_data = serialize_trie(&root).unwrap();
        assert!(validate_binary_format(&valid_data));

        let invalid_data = vec![1, 2, 3]; // Too short
        assert!(!validate_binary_format(&invalid_data));
    }

    #[test]
    fn test_reduce_chord_vocab() {
        let chords = vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5], // Duplicate
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
        ];

        let reduced = reduce_chord_vocab(&chords, 2).unwrap();
        assert_eq!(reduced.len(), 4); // Same length, but limited vocabulary
    }

    #[test]
    fn test_augment_with_repeated() {
        let sequence = vec![1, 2, 3];
        let augmented = augment_with_repeated(sequence, 10);

        assert!(augmented.len() >= 10);
        assert_eq!(augmented[0], 1);
        assert_eq!(augmented[1], 2);
        assert_eq!(augmented[2], 3);
    }
}
