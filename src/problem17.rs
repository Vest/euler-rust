use numerics::Numerics;

fn build_word(i: usize) -> String {
    let converter = Numerics::default();

    match converter.convert_number(i) {
        Ok(words) => {
            if i > 100 && i % 100 != 0 {
                let l = words.len();

                let index = if i % 10 != 0 && i % 100 > 19 { 2 } else { 1 };
                [
                    // fix numerics, because it doesn't use "and". See the test test_build_word.
                    &words[0..l - index],
                    &vec![String::from("and")],
                    &words[l - index..l]
                ]
                    .concat()
                    .concat()
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
    let answer = build_words(1000);
    println!("Problem 17. The answer is {}.", answer.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_word() {
        assert_eq!(build_word(5), "five");
        assert_eq!(build_word(342), "threehundredandfortytwo");
        assert_eq!(build_word(342).len(), 23);
        assert_eq!(build_word(115), "onehundredandfifteen");
        assert_eq!(build_word(115).len(), 20);
        assert_eq!(build_word(111), "onehundredandeleven");
        assert_eq!(build_word(119), "onehundredandnineteen");
        assert_eq!(build_word(120), "onehundredandtwenty");
        assert_eq!(build_word(130), "onehundredandthirty");
        assert_eq!(build_word(999), "ninehundredandninetynine");
        assert_eq!(build_word(1000), "onethousand");
    }

    #[test]
    fn test_build_words() {
        assert_eq!(build_words(5), "onetwothreefourfive");
    }
}
