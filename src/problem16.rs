use num_bigint::{BigUint, ToBigUint};

fn get_bignum(exponent: u32) -> BigUint {
    BigUint::from(2_u8).pow(exponent)
}

fn get_answer(exponent: u32) -> BigUint {
    let input = get_bignum(exponent);
    input.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .filter_map(|d| d.to_biguint())
        .sum()
}

fn main() {
    let answer = get_answer(1000);
    println!("Problem 16. The answer is {}.", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bignum() {
        assert_eq!(get_bignum(15), BigUint::from(32768_u32));
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(15), BigUint::from(26_u32));
    }
}
