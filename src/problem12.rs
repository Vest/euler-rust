use itertools::Itertools;
use primes::{factors};

fn get_triangle_number(n: usize) -> usize {
    (1 + n) * n / 2
}

fn count_divisor(n: usize) -> usize {
    factors(n as u64)
        .into_iter()
        .counts()
        .values()
        .map(|c| c + 1)
        .product::<usize>()
}

fn main() {
    let answer = (1..)
        .map(get_triangle_number)
        .map(|t| (t, count_divisor(t)))
        .filter(|&(_, count)| count >= 500_usize)
        .map(|(t, _)| t)
        .next();
    println!("Problem 12. The number is {}th, or indeed {}.", answer.unwrap_or_default(), get_triangle_number(answer.unwrap_or_default()), );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_triangle_number() {
        assert_eq!(get_triangle_number(0), 0);
        assert_eq!(get_triangle_number(1), 1);
        assert_eq!(get_triangle_number(2), 3);
        assert_eq!(get_triangle_number(3), 6);
        assert_eq!(get_triangle_number(4), 10);
        assert_eq!(get_triangle_number(5), 15);
        assert_eq!(get_triangle_number(6), 21);
    }

    #[test]
    fn test_count_divisor() {
        assert_eq!(count_divisor(1), 1);
        assert_eq!(count_divisor(3), 2);
        assert_eq!(count_divisor(6), 4);
        assert_eq!(count_divisor(10), 4);
        assert_eq!(count_divisor(15), 4);
        assert_eq!(count_divisor(21), 4);
        assert_eq!(count_divisor(28), 6);
        assert_eq!(count_divisor(210), 16);
    }
}