fn get_triangle_number(n: usize) -> usize {
    (1..=n).sum()
}

fn count_divisor(n: usize) -> usize {
    let is_even = n % 2 == 0;
    let max = if is_even { n / 2 } else { (n + 1) / 2 };
    (1..max)
        .map(|d| n % d)
        .filter(|&r| r == 0_usize)
        .take(500)
        .count() + if is_even { 2 } else { 1 }
}

fn main() {
    let mut begin = 1;
    let mut end = 2;

    let ans_begin =  count_divisor(get_triangle_number(1000000));
    println!("Problem 12. Answer is {}", ans_begin);
/*
    loop {
        let ans_begin =  count_divisor(get_triangle_number(begin));
        let ans_end =  count_divisor(get_triangle_number(end));

        if ans_end < 500 {
            begin = end;
            end *= 10;
        } else {
            println!("Found end {}", end)
        }
        if ans_begin < 500 {
          //  begin += 1;
        } else {
            println!("Problem 12. Answer is {}", begin);
            break;
        }
    }

 */
/*
    let answer = (1..)
        .map(get_triangle_number)
        .map(|t| (t, count_divisor(t)))
        .filter(|&(_, count)| count >= 500_usize)
        .map(|(t, _)| t)
        .next();
    println!("Problem 12. Answer is {}", answer.unwrap_or_default());

 */
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