use std::collections::BTreeMap;

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
/// TODO: add SIMD acceleration
pub(crate) fn search_bmh(map: &BTreeMap<usize, u8>, pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 || map.is_empty() {
        return vec![];
    }

    // Build bad match table
    let mut bad_match = [m as u8; 256];
    for i in 0..m - 1 {
        bad_match[pattern[i] as usize] = (m - 1 - i) as u8;
    }

    // Prepare result collection
    let mut results = Vec::new();

    // Build a Vec of (addr,byte) indices into map for sequential access.
    // NOTE: only address are stored in this Vec, not the bytes.
    let addrs: Vec<usize> = map.keys().copied().collect();

    // Check if length of address is less than the pattern
    let n = addrs.len();
    if n < m {
        return results;
    }

    let bytes_iter = |i: usize| -> u8 { map[&addrs[i]] };

    // Main BMH loop
    let mut i = 0; // index into addrs[]

    while i <= n - m {
        // Compare pattern from right to left
        let mut j = (m - 1) as isize;
        while j >= 0 && bytes_iter(i + j as usize) == pattern[j as usize] {
            j -= 1;
        }

        if j < 0 {
            // Match found
            results.push(addrs[i]);
            i += 1; // advance minimally
        } else {
            // Mismatch -> skip using last byte of window
            let last_byte = bytes_iter(i + m - 1);
            i += bad_match[last_byte as usize] as usize;
        }
    }
    results
}
