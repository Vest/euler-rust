const MAX_SIZE: usize = 20;

#[derive(PartialEq, Clone, Debug)]
enum Step {
    Right,
    Down,
}

fn is_valid(path: &Vec<Step>, step: &Step, max_size: usize) -> bool {
    let right_steps_count = path.iter()
        .filter(|&s| *s == Step::Right)
        .count();
    let down_steps_count = path.iter()
        .filter(|&s| *s == Step::Down)
        .count();

    right_steps_count + if step == &Step::Right { 1 } else { 0 } <= max_size
        && down_steps_count + if step == &Step::Down { 1 } else { 0 } <= max_size
}

fn count_combinations(path: &Vec<Step>, max_size: usize) -> usize {
    if path.len() == 2 * max_size {
        return 1;
    }

    [Step::Right, Step::Down].iter()
        .filter(|&step| is_valid(path, step, max_size))
        .map(|step| {
            let mut new_path: Vec<Step> = path.clone();
            new_path.push(step.clone());
            count_combinations(&new_path, max_size)
        }).sum::<usize>()
}

fn main() {
    println!("Problem 15. The number is {}.", count_combinations(&Vec::with_capacity(2 * MAX_SIZE), MAX_SIZE));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid(&vec![Step::Right; MAX_SIZE - 1], &Step::Right, MAX_SIZE));
        assert!(is_valid(&vec![Step::Right; MAX_SIZE - 1], &Step::Down, MAX_SIZE));
        assert!(!is_valid(&vec![Step::Right; MAX_SIZE], &Step::Right, MAX_SIZE));
        assert!(is_valid(&vec![Step::Right; MAX_SIZE], &Step::Down, MAX_SIZE));

        assert!(!is_valid(&vec![Step::Right, Step::Right], &Step::Right, 2));
        assert!(is_valid(&vec![Step::Right, Step::Right], &Step::Down, 2));
        assert!(is_valid(&vec![Step::Right, Step::Right, Step::Down], &Step::Down, 2));
        assert!(!is_valid(&vec![Step::Right, Step::Right, Step::Down, Step::Down], &Step::Down, 2));
        assert!(!is_valid(&vec![Step::Right, Step::Right, Step::Down, Step::Down], &Step::Right, 2));
        assert!(!is_valid(&vec![Step::Right, Step::Right, Step::Down, Step::Right], &Step::Right, 2));
    }

    #[test]
    fn test_count_combinations() {
        (1..11).for_each(|n| {
            assert_eq!(count_combinations(&Vec::with_capacity(2 * n), n), factorial(2 * n) / (factorial(n) * factorial(n)));
        })
    }
}