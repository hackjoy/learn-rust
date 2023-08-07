const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

pub fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod test {
    use crate::examples::fibonnaci::fib;

    #[test]
    fn test_one() {
        let result = fib(6_u64);
        assert_eq!(result, 13_u64)
    }
}
