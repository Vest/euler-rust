use primes::factors;

fn find_answer() -> usize {
    0
}

fn u(n: i128) -> i128 {
    v2((3 * s(n) + 4) as u64) as i128
}

fn un(n: i128) -> i128 {
    (1..=n)
        .map(|k| u(k * k * k))
        .sum()
}

fn v2(n: u64) -> usize {
    factors(n)
        .iter()
        .filter(|&&d| d == 2)
        .count()
}

fn s(n: i128) -> i128 {
    (1..=n)
        .map(|k| (-2_i128).pow(k as u32) * optimized_binom(k))
        .sum()
}

fn binom(n: i128, k: i128) -> i128 {
    factorial(n) / factorial(k) / factorial(n - k)
}

fn optimized_binom(k: i128) -> i128 {
    let d = factorial(k);
    (k + 1..=2 * k).product::<i128>() / d
}

fn factorial(n: i128) -> i128 {
    (2..=n).product()
}

fn main() {
    println!("Problem 14. Answer is {}", find_answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binom() {
        assert_eq!(binom(5, 1), 5);
        assert_eq!(binom(5, 2), 10);
        assert_eq!(binom(5, 3), 10);
        assert_eq!(binom(5, 4), 5);
    }

    #[test]
    fn test_s() {
        assert_eq!(s(4), 980);
    }

    #[test]
    fn test_v2() {
        assert_eq!(v2(24), 3);
    }

    #[test]
    fn test_u() {
        assert_eq!(u(4), 7);
        assert_eq!(u(20), 24);
    }

    #[test]
    fn test_un() {
        assert_eq!(un(5), 214);
    }
}
