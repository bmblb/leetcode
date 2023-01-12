use crate::utils::{ListNode,to_node};

fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut left = &list1;
    let mut right = &list2;

    if list1 == None && list2 == None {
        return None;
    }

    let mut result = Some(Box::new(ListNode::new(0)));
    let mut node = result.as_mut();

    loop {
        let n = node.unwrap();

        match (left, right) {
            (Some(l), Some(r)) if l.val <= r.val => {
                n.val = l.val;
                left = &l.next;
            },
            (Some(l), None) => {
                n.val = l.val;
                left = &l.next;
            },
            (Some(l), Some(r)) if l.val > r.val => {
                n.val = r.val;
                right = &r.next;
            },
            (None, Some(r)) => {
                n.val = r.val;
                right = &r.next;
            },
            _ => { }
        }

        if *left != None || *right != None {
            n.next = ListNode::new_wrapped(0);
            node = n.next.as_mut();
        }
        else {
            break;
        }
    };

    result
}

pub fn main() {
    let list1 = to_node(vec![1,2,4]);
    let list2 = to_node(vec![1,3,4]);

    assert_eq!(merge_two_lists(list1, list2).unwrap().to_vec(), vec![1,1,2,3,4,4]);
    assert_eq!(merge_two_lists(None, None), None);
    assert_eq!(merge_two_lists(None, to_node(vec![0])).unwrap().to_vec(), vec![0]);
}
