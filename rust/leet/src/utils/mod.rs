#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }

    pub fn new_wrapped(val: i32) -> Option<Box<Self>> {
        Some(Box::new(Self::new(val)))
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut result: Vec<i32> = vec![self.val];

        let mut next = &self.next;

        loop {
            match next {
                Some(value) => {
                    result.push(value.val);
                    next = &value.next;
                },
                None => {
                    break;
                }
            }
        }

        return result;
    }
}

pub fn to_node(input: Vec<i32>) -> Option<Box<ListNode>> {
    input.iter().rev().fold(None, |acc, &x| {
        Some(Box::new(ListNode { val: x, next: acc }))
    })
}

pub fn from_node(input: &Option<Box<ListNode>>) -> i32 {
    let mut result = vec![];

    let mut node = input;

    while let Some(l) = node {
        result.push(l.val);
        node = &l.next;
    }

    result.iter().enumerate().map(|(index, value)| { value * 10_i32.pow(index as u32) }).sum()
}
