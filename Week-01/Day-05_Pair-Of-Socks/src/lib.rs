#[warn(non_snake_case)]
use std::collections::HashMap;

pub fn sock_pairs(socks: &str) -> i32 {
    let mut sock_map = HashMap::new();

    // Count each type of sock, if no sock of type 'char' is in map adds default to map with valve = 0, otherwise adds 1
    for sock in socks.chars() {
        *sock_map.entry(sock).or_insert(0) += 1;
    }

    // Iterates over each type of sock, divides by 2 [as we are looking for pairs] and sums.
    sock_map.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(sock_pairs("AA"), 1);
        assert_eq!(sock_pairs("ABABC"), 2);
        assert_eq!(sock_pairs("CABBACCC"), 4);
    }

    #[test]
    fn test_for_empty() {
        assert_eq!(sock_pairs(""), 0);
    }
}
