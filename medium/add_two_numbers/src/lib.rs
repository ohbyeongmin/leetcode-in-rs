#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_linked_list(input: Vec<i32>) -> Option<Box<ListNode>> {
        let mut pointer = Box::new(ListNode::new(input[input.len() - 1]));

        for v in input.iter().rev().skip(1) {
            let node = ListNode {
                val: *v,
                next: Some(pointer),
            };
            pointer = Box::new(node);
        }

        Some(pointer)
    }

    #[test]
    fn case_one() {
        let l1 = make_linked_list(vec![2, 4, 3]);
        let l2 = make_linked_list(vec![5, 6, 4]);
        let result = make_linked_list(vec![7, 0, 8]);

        assert_eq!(add_two_numbers(l1, l2), result);
    }

    #[test]
    fn case_second() {
        let l1 = make_linked_list(vec![0]);
        let l2 = make_linked_list(vec![0]);
        let result = make_linked_list(vec![0]);

        assert_eq!(add_two_numbers(l1, l2), result);
    }

    #[test]
    fn case_third() {
        let l1 = make_linked_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = make_linked_list(vec![9, 9, 9, 9]);
        let result = make_linked_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);

        assert_eq!(add_two_numbers(l1, l2), result);
    }
}
