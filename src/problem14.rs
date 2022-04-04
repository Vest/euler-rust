struct Collatz {
    curr: usize,
}

impl Collatz {
    fn new(start: usize) -> Collatz {
        Collatz { curr: if start > 1 { start } else { 1 } }
    }
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 1 {
            return None;
        }

        let next = match self.curr {
            n if n % 2 == 0 => n / 2,
            n => 3 * n + 1,
        };

        self.curr = next;

        Some(next)
    }
}

fn find_answer() -> usize {
    (1..1_000_000_usize)
        .map(|n| (n, Collatz::new(n).count()))
        .max_by(|&(_, c1),&(_, c2)| c1.cmp(&c2))
        .unwrap()
        .0
}

fn main() {
    println!("Problem 14. Answer is {}", find_answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_iterator() {
        let mut col = Collatz::new(13);
        assert_eq!(col.count(), 10 - 1);
    }
}
