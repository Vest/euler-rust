use primes::{factors, factors_uniq};

fn get_triangle_number(n: usize) -> usize {
    (1 + n) * n / 2
}

fn count_divisor(n: usize) -> usize {
    let divisors = factors(n as u64);
    let unique_divisors = factors_uniq(n as u64);
    let count_divisors = divisors.len();
    let count_unique_divisors = unique_divisors.len();

    dbg!(n, divisors, unique_divisors, count_divisors);

    (1..count_unique_divisors)
        .map(|c| count_comb(count_divisors + c - 1, c))
        .sum::<usize>() + if n == 1 { 1 } else { 2 }
}

fn factorial(n: usize) -> usize {
    (1..n)
        .product()
}

fn count_comb(n: usize, k: usize) -> usize {
    (n - k + 1..=n).product::<usize>() / factorial(k)
}

fn main() {
    let answer = (1..)
        .map(get_triangle_number)
        .map(|t| (t, count_divisor(t)))
        .filter(|&(_, count)| dbg!(count) >= 500_usize)
        .map(|(t, _)| t)
        .next();
    println!("Problem 12. Answer is {}", answer.unwrap_or_default());
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
    }
}