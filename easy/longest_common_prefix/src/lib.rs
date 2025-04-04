pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(a, c)| a == c)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_first() {
        let input = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let output = String::from("fl");

        assert_eq!(longest_common_prefix(input), output);
    }

    #[test]
    fn case_second() {
        let input = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        let output = String::from("");

        assert_eq!(longest_common_prefix(input), output);
    }

    #[test]
    fn case_third() {
        let input = vec![String::from("aaa"), String::from("aa"), String::from("aaa")];
        let output = String::from("aa");

        assert_eq!(longest_common_prefix(input), output);
    }
}
