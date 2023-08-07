use std::collections::HashMap;

// Use caching to store results (dyamic programming)
pub fn dynamic_fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = dynamic_fib(n - 1, map) + dynamic_fib(n - 2, map);
                map.insert(n, val);
                val
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::examples::dynamic_fibonnaci;

    #[test]
    fn test_one() {
        let mut map = HashMap::new();
        let result = dynamic_fibonnaci::dynamic_fib(6_u64, &mut map);
        assert_eq!(result, 13_u64)
    }
}
