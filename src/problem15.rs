const MAX_SIZE: usize = 20;

#[derive(PartialEq, Clone, Debug)]
#[allow(unused)]
enum Step {
    Right,
    Down,
}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
fn count_steps(x: usize, y: usize) -> usize {
    if x == 0 && y == 0 {
        0
    } else if x == 0 || y == 0 {
        1
    } else {
        count_steps(x - 1, y) + count_steps(x, y - 1)
    }
}

struct Cache {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Cache {
    fn new(w: usize, h: usize) -> Cache {
        let mut cache = Cache {
            grid: vec![vec![0; w + 1]; h + 1],
            width: w + 1,
            height: h + 1,
        };
        (1..w + 1).for_each(|i| cache.grid[0][i] = 1);
        (1..h + 1).for_each(|i| cache.grid[i][0] = 1);
        cache
    }

    fn count_steps(&mut self, h: usize, v: usize) {
        let up = self.grid[v - 1][h];
        let left = self.grid[v][h - 1];

        if up == 0 {
            self.count_steps(v - 1, h);
        }
        if left == 0 {
            self.count_steps(v, h - 1);
        }

        self.grid[v][h] = self.grid[v - 1][h] + self.grid[v][h - 1];
    }

    fn fill_cache(&mut self) {
        (1..self.height).for_each(|h| {
            (1..self.width).for_each(|w| {
                self.count_steps(h, w);
            })
        })
    }
}

fn main() {
    // println!("Problem 15. The number is {}.", count_combinations(&Vec::with_capacity(2 * MAX_SIZE), MAX_SIZE));
    // println!("Problem 15. The number is {}.", count_steps(MAX_SIZE, MAX_SIZE));
    let mut cache = Cache::new(MAX_SIZE, MAX_SIZE);
    cache.fill_cache();
    println!("Problem 15. The number is {}.", cache.grid[MAX_SIZE][MAX_SIZE]);
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
        (1..9).for_each(|n| {
            assert_eq!(count_combinations(&Vec::with_capacity(2 * n), n), factorial(2 * n) / (factorial(n) * factorial(n)));
        })
    }

    #[test]
    fn test_count_steps() {
        assert_eq!(count_steps(2, 2), 6);
        assert_eq!(count_steps(3, 3), factorial(2 * 3) / (factorial(3) * factorial(3)));
    }

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(3, 3);
        cache.fill_cache();
        assert_eq!(cache.grid[3][3], factorial(2 * 3) / (factorial(3) * factorial(3)));
    }
}