use numerics::Numerics;

fn build_word(i: usize) -> String {
    let converter = Numerics::default();

    match converter.convert_number(i) {
        Ok(words) => {
            if i > 100 {
                let l = words.len();
                if i % 10 != 0 {
                    [
                        // fix numerics, because it doesn't use "and". See the test test_build_word.
                        &words[0..l - 2],
                        &vec![String::from("and")],
                        &words[l - 2..l]
                    ]
                        .concat()
                        .concat()
                } else {
                    [
                        // fix numerics, because it doesn't use "and". See the test test_build_word.
                        &words[0..l - 1],
                        &vec![String::from("and")],
                        &words[l - 1..l]
                    ]
                        .concat()
                        .concat()
                }
            } else {
                words.join("")
            }
        }
        _ => String::new(),
    }
}

fn build_words(to: usize) -> String {
    (1..=to)
        .map(build_word)
        .collect()
}


fn main() {
    //  let answer = get_answer(1000);
    //  println!("Problem 16. The answer is {}.", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_word() {
        assert_eq!(build_word(5), "five");
        assert_eq!(build_word(342), "threehundredandfortytwo");
        assert_eq!(build_word(115), "onehundredandfifteen");
    }

    #[test]
    fn test_build_words() {
        assert_eq!(build_words(5), "onetwothreefourfive");
    }
}
