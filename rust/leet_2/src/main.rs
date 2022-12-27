// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn to_node(input: Vec<i32>) -> Option<Box<ListNode>> {
    input.iter().rev().fold(None, |acc, &x| {
        Some(Box::new(ListNode { val: x, next: acc }))
    })
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = Box::new(ListNode::new(0));
    let mut left = l1;
    let mut right = l2;
    let mut res = &mut result;
    let mut carry = 0;

    loop {
        let (l1, l2) = match (left.as_mut(), right.as_mut()) {
            (Some(l), Some(r)) => (l.val, r.val),
            (None, Some(r)) | (Some(r), None) => (0, r.val),
            (None, None) => (0, 0)
        };

        let sum = l1 + l2 + carry;

        carry = sum / 10;

        res.val = sum % 10;

        if let Some(l) = left {
            left = l.next;
        }

        if let Some(l) = right {
            right = l.next;
        }

        if left == None && right == None && carry == 0 {
            break;
        }

        // let mut next = Some(Box::new(ListNode { val: 0, next: None }));
        res.next = Some(Box::new(ListNode::new(0)));
        res = res.next.as_mut().unwrap();
    }

    Some(result)
}

fn main() {
    // [2,4,3]
    // [5,6,4]
    // [7,0,8]
    let node1 = to_node(vec![2,4,3]);
    let node2 = to_node(vec![5,6,4]);

    println!("{:?}", node1);
    println!("{:?}", node2);

    let res = add_two_numbers(node1, node2);

    println!("{:?}", res);
}
