pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for b in s.chars() {
        match b {
            '(' | '{' | '[' => stack.push(b),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_first() {
        let input = String::from("()");
        let output = true;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn case_second() {
        let input = String::from("()[]{}");
        let output = true;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn case_third() {
        let input = String::from("(]");
        let output = false;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn case_fourth() {
        let input = String::from("([])");
        let output = true;

        assert_eq!(is_valid(input), output);
    }
}
