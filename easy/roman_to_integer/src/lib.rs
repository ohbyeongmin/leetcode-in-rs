pub fn roman_to_int(s: String) -> i32 {
    let s2: &str = &s;

    s2.chars().rev().fold(0, |acc, e| match e {
        'I' => {
            if acc >= 5 {
                acc - 1
            } else {
                acc + 1
            }
        }
        'V' => acc + 5,
        'X' => {
            if acc >= 50 {
                acc - 10
            } else {
                acc + 10
            }
        }
        'L' => acc + 50,
        'C' => {
            if acc >= 500 {
                acc - 100
            } else {
                acc + 100
            }
        }
        'D' => acc + 500,
        'M' => acc + 1000,
        _ => 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_first() {
        let input = String::from("III");
        let output = 3;

        assert_eq!(roman_to_int(input), output);
    }

    #[test]
    fn case_second() {
        let input = String::from("LVIII");
        let output = 58;

        assert_eq!(roman_to_int(input), output);
    }

    #[test]
    fn case_third() {
        let input = String::from("MCMXCIV");
        let output = 1994;

        assert_eq!(roman_to_int(input), output);
    }
}
