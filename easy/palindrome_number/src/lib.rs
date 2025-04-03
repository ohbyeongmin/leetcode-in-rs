pub fn is_palindrome(x: i32) -> bool {
    match x {
        _ if x < 0 => false,
        0..=9 => true,
        _ => {
            let mut v: Vec<i32> = Vec::new();
            let mut mut_x = x;

            while mut_x > 0 {
                v.push(mut_x % 10);
                mut_x /= 10;
            }

            let reversed: i32 = v.iter().fold(0, |acc, &digit| acc * 10 + digit);
            x == reversed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one() {
        let input = 121;
        let output = true;

        assert_eq!(is_palindrome(input), output);
    }

    #[test]
    fn case_second() {
        let input = -121;
        let output = false;

        assert_eq!(is_palindrome(input), output);
    }

    #[test]
    fn case_third() {
        let input = 10;
        let output = false;

        assert_eq!(is_palindrome(input), output);
    }
}
