//! Hash and compression functions for serialization

use crate::error::SerializationResult;
use std::collections::HashMap;

/// Fast hash function for 32-bit hash generation
pub fn fast_hash(data: &str) -> u32 {
    let mut hash: u32 = 0;

    for byte in data.bytes() {
        hash = hash
            .wrapping_shl(5)
            .wrapping_sub(hash)
            .wrapping_add(byte as u32);
        // Ensure 32-bit overflow behavior is handled by wrapping operations above
    }

    // Return absolute value by clearing sign bit
    hash & 0x7FFFFFFF
}

/// Fold hash function for combining hashes
pub fn fold_hash(existing_hash: u32, data: &str) -> u32 {
    let new_hash = fast_hash(data);

    // Combine with bit rotation and XOR
    let rotated = existing_hash.rotate_left(5);
    let combined = rotated ^ new_hash;

    // Ensure deterministic output
    combined & 0x7FFFFFFF
}

/// Scale fingerprint encoding using run-length compression and base64-like encoding
pub fn scale40_encode(fingerprint: &[bool; 12]) -> SerializationResult<String> {
    // Step 1: Convert boolean fingerprint to bit pattern
    let mut bit_pattern = 0u16;
    for (i, &active) in fingerprint.iter().enumerate() {
        if active {
            bit_pattern |= 1 << i;
        }
    }

    // Step 2: Compress using run-length encoding
    let compressed = compress_bit_pattern(bit_pattern);

    // Step 3: Apply base64-like encoding for compact representation
    let encoded = base64_like_encode(&compressed);

    // Step 4: Add checksum for integrity verification
    let checksum = calculate_checksum(&encoded);

    Ok(format!("{}{:02x}", encoded, checksum))
}

/// Scale fingerprint decoding - reverses the compression process
pub fn scale40_decode(encoded: &str) -> SerializationResult<[bool; 12]> {
    if encoded.len() < 3 {
        return Err(crate::error::SerializationError::InvalidFormat {
            message: "Scale encoding too short".to_string(),
        });
    }

    // Extract checksum (last 2 characters)
    let (data_part, checksum_part) = encoded.split_at(encoded.len() - 2);
    let expected_checksum = u8::from_str_radix(checksum_part, 16).map_err(|_| {
        crate::error::SerializationError::InvalidFormat {
            message: "Invalid checksum format".to_string(),
        }
    })?;

    // Verify checksum
    let actual_checksum = calculate_checksum(data_part);
    if actual_checksum != expected_checksum {
        return Err(crate::error::SerializationError::InvalidFormat {
            message: "Checksum mismatch".to_string(),
        });
    }

    // Step 1: Decode base64-like encoding
    let compressed = base64_like_decode(data_part)?;

    // Step 2: Decompress using run-length encoding
    let bit_pattern = decompress_bit_pattern(&compressed)?;

    // Step 3: Convert bit pattern to boolean array
    let mut fingerprint = [false; 12];
    for (i, item) in fingerprint.iter_mut().enumerate() {
        *item = (bit_pattern & (1 << i)) != 0;
    }

    Ok(fingerprint)
}

// Helper functions

fn compress_bit_pattern(pattern: u16) -> Vec<u8> {
    // Simple run-length encoding
    let mut result = Vec::new();

    if pattern == 0 {
        // Special case: all zeros
        result.push(12); // 12 zeros
        return result;
    }

    let mut current_bit = (pattern & 1) != 0;
    let mut run_length = 1u8;

    for i in 1..12 {
        let bit = (pattern & (1 << i)) != 0;
        if bit == current_bit {
            run_length += 1;
        } else {
            // Output the current run
            result.push(if current_bit {
                0x80 | run_length
            } else {
                run_length
            });
            current_bit = bit;
            run_length = 1;
        }
    }

    // Add final run
    result.push(if current_bit {
        0x80 | run_length
    } else {
        run_length
    });

    result
}

fn decompress_bit_pattern(compressed: &[u8]) -> SerializationResult<u16> {
    let mut pattern = 0u16;
    let mut position = 0;

    for &byte in compressed {
        let is_set = (byte & 0x80) != 0;
        let length = byte & 0x7F;

        if length == 0 {
            continue; // Skip zero-length runs
        }

        if is_set {
            // Set bits for this run
            for _ in 0..length {
                if position < 12 {
                    pattern |= 1 << position;
                }
                position += 1;
            }
        } else {
            // Skip bits for this run (leave as 0)
            position += length as usize;
        }

        if position >= 12 {
            break;
        }
    }

    Ok(pattern)
}

fn base64_like_encode(data: &[u8]) -> String {
    // Proper base64-like encoding that preserves all data
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();

    // Process input in chunks of 3 bytes -> 4 chars (like base64)
    for chunk in data.chunks(3) {
        let b1 = chunk[0] as u32;
        let b2 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b3 = chunk.get(2).copied().unwrap_or(0) as u32;

        // Combine 3 bytes into 24 bits
        let combined = (b1 << 16) | (b2 << 8) | b3;

        // Extract 4 groups of 6 bits each
        result.push(CHARS[((combined >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((combined >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[(combined & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn base64_like_decode(encoded: &str) -> SerializationResult<Vec<u8>> {
    // Reverse of base64_like_encode
    let mut char_map = HashMap::new();
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    for (i, &ch) in CHARS.iter().enumerate() {
        char_map.insert(ch as char, i as u8);
    }

    let mut result = Vec::new();
    let chars: Vec<char> = encoded.chars().collect();

    // Process in chunks of 4 characters
    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            break;
        }

        let c1 = char_map.get(&chunk[0]).copied().unwrap_or(0) as u32;
        let c2 = char_map.get(&chunk[1]).copied().unwrap_or(0) as u32;
        let c3 = if chunk.len() > 2 && chunk[2] != '=' {
            char_map.get(&chunk[2]).copied().unwrap_or(0) as u32
        } else {
            0
        };
        let c4 = if chunk.len() > 3 && chunk[3] != '=' {
            char_map.get(&chunk[3]).copied().unwrap_or(0) as u32
        } else {
            0
        };

        // Combine 4 groups of 6 bits back to 3 bytes
        let combined = (c1 << 18) | (c2 << 12) | (c3 << 6) | c4;

        result.push((combined >> 16) as u8);

        if chunk.len() > 2 && chunk[2] != '=' {
            result.push((combined >> 8) as u8);
        }

        if chunk.len() > 3 && chunk[3] != '=' {
            result.push(combined as u8);
        }
    }

    Ok(result)
}

fn calculate_checksum(data: &str) -> u8 {
    let mut checksum = 0u8;
    for byte in data.bytes() {
        checksum = checksum.wrapping_add(byte);
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_hash() {
        let hash1 = fast_hash("test");
        let hash2 = fast_hash("test");
        let hash3 = fast_hash("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert!(hash1 < 0x80000000); // Should be positive
    }

    #[test]
    fn test_fold_hash() {
        let initial = fast_hash("initial");
        let folded = fold_hash(initial, "additional");

        assert_ne!(initial, folded);
        assert!(folded < 0x80000000); // Should be positive
    }

    #[test]
    fn test_scale40_roundtrip() {
        let original = [
            true, false, true, true, false, true, false, true, true, false, true, false,
        ];

        let encoded = scale40_encode(&original).unwrap();
        let decoded = scale40_decode(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_compression_functions_used() {
        // Test that compression functions work correctly
        let pattern = 0b101100110101u16; // Mixed pattern to test run-length encoding
        let compressed = compress_bit_pattern(pattern);
        let decompressed = decompress_bit_pattern(&compressed).unwrap();
        assert_eq!(pattern, decompressed);
    }

    #[test]
    fn test_base64_like_encoding() {
        // Test base64-like encoding functions
        let data = vec![0x12, 0x34, 0x56, 0x78, 0x9A];
        let encoded = base64_like_encode(&data);
        let decoded = base64_like_decode(&encoded).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_scale40_compression_efficiency() {
        // Test various patterns to ensure compression works correctly
        let test_cases = [
            // All false
            [false; 12],
            // All true
            [true; 12],
            // Alternating pattern
            [
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
            // Major scale pattern
            [
                true, false, true, false, true, true, false, true, false, true, false, true,
            ],
            // Random pattern
            [
                true, true, false, true, false, false, true, true, true, false, false, true,
            ],
        ];

        for (i, &pattern) in test_cases.iter().enumerate() {
            let encoded = scale40_encode(&pattern).unwrap();
            let decoded = scale40_decode(&encoded).unwrap();
            assert_eq!(pattern, decoded, "Failed roundtrip for test case {}", i);

            // Ensure encoded string is valid
            assert!(
                encoded.len() >= 3,
                "Encoded string too short for test case {}",
                i
            );
            assert!(
                encoded
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='),
                "Invalid characters in encoded string for test case {}",
                i
            );
        }
    }
}
