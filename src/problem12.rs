
fn get_triangle_number(n: usize) -> usize {
    (1..=n).sum()
}

fn count_divisor(n:usize) -> usize {
    let is_even = n % 2 == 0;
    let max = if is_even {n / 2} else {(n + 1) / 2};
    (1..max)
        .map(|d| n % d)
        .filter(|&r| r == 0_usize)
        .count() + if is_even {2} else {1}
}

fn main() {
    println!("Hello from Problem 12");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_triangle_number() {
        assert_eq!(get_triangle_number(0),0);
        assert_eq!(get_triangle_number(1),1);
        assert_eq!(get_triangle_number(2),3);
        assert_eq!(get_triangle_number(3),6);
        assert_eq!(get_triangle_number(4),10);
        assert_eq!(get_triangle_number(5),15);
        assert_eq!(get_triangle_number(6),21);
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