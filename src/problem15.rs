const MAX_SIZE: usize = 2;

#[derive(PartialEq, Clone)]
enum Step {
    Right,
    Down,
}

fn is_valid(path: &Vec<Step>, step: &Step) -> bool {
    let right_steps_count = path.iter()
        .filter(|&s| *s == Step::Right)
        .count();
    let down_steps_count = path.iter()
        .filter(|&s| *s == Step::Down)
        .count();

    right_steps_count + if step == &Step::Right { 1 } else { 0 } <= MAX_SIZE
        && down_steps_count + if step == &Step::Down { 1 } else { 0 } <= MAX_SIZE
}

fn count_combinations(path: &Vec<Step>) -> usize {
    let valid_count = [Step::Right, Step::Down].iter()
        .filter(|&step| is_valid(path, step))
        .count();

    if valid_count == 0 {
        return 0;
    }

    let answer = valid_count * {
        [Step::Right, Step::Down].iter()
            .map(|step| {
                let mut new_path: Vec<Step> = path.clone();
                new_path.push(step.clone());
                count_combinations(&new_path)
            }).sum::<usize>()
    };

    if answer == 0 { 1 } else { answer }
}

fn main() {
    println!("Problem 15. The number is {}.", count_combinations(&vec![]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(&vec![Step::Right; MAX_SIZE - 1], &Step::Right));
        assert!(is_valid(&vec![Step::Right; MAX_SIZE - 1], &Step::Down));
        assert!(!is_valid(&vec![Step::Right; MAX_SIZE], &Step::Right));
        assert!(is_valid(&vec![Step::Right; MAX_SIZE], &Step::Down));
    }
}