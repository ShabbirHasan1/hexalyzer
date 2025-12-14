use std::collections::btree_map;

/// Parse `str` hex representation into `Vec<u8>`
pub(crate) fn parse_hex_str_into_vec(input: &str) -> Option<Vec<u8>> {
    // Must have even length
    if !input.len().is_multiple_of(2) {
        return None;
    }

    // Convert each 2-char chunk to a byte
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).ok())
        .collect()
}

/// Boyer–Moore–Horspool algorithm for BTreeMap<usize, u8>.
/// Returns the starting addresses of all matches.
///
/// TODO: 1) add SIMD acceleration; 2) Replace with KMP search?
pub(crate) fn search_bmh(map_iter: btree_map::Iter<usize, u8>, pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 || m > u8::MAX as usize {
        return vec![];
    }

    // Consume the iterator once into an indexable representation.
    // This does not clone the BTreeMap, only copies (usize, u8) pairs.
    let haystack: Vec<(usize, u8)> = map_iter.map(|(&addr, &byte)| (addr, byte)).collect();

    // Check if length of address is less than the pattern
    let n = haystack.len();
    if n < m {
        return vec![];
    }

    // Build bad match table
    let mut bad_match = [m as u8; 256];
    for i in 0..m - 1 {
        bad_match[pattern[i] as usize] = (m - 1 - i) as u8;
    }

    // Prepare result collection
    let mut results = Vec::new();

    // Main BMH loop
    let mut i = 0; // index into addrs[]
    while i <= n - m {
        // Compare pattern from right to left
        let mut j = (m - 1) as isize;
        while j >= 0 && haystack[i + j as usize].1 == pattern[j as usize] {
            j -= 1;
        }

        if j < 0 {
            // Match found
            results.push(haystack[i].0);
            i += 1; // advance minimally
        } else {
            // Mismatch -> skip using last byte of window
            let last_byte = haystack[i + m - 1].1;
            i += bad_match[last_byte as usize] as usize;
        }
    }
    results
}
