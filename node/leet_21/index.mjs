import { assert_eq, ListNode } from "../utils.mjs";

function merge_two_lists(left, right) {
    if (!left && !right) {
        return null;
    }
    else {
        const result = new ListNode();
        let node = result;

        while (left || right) {
            if (left && !right) {
                node.val = left.val;
                left = left.next;
            }
            else if (!left && right) {
                node.val = right.val;
                right = right.next;
            }
            else if (left.val <= right.val) {
                node.val = left.val;
                left = left.next;
            }
            else if (left.val > right.val) {
                node.val = right.val;
                right = right.next;
            }

            if (left || right) {
                node = node.next = new ListNode();
            }
            else {
                break;
            }
        }

        return result;
    }
}

export function run() {
    assert_eq(merge_two_lists(ListNode.from_array([1,2,4]), ListNode.from_array([1,3,4])).to_array(), [1,1,2,3,4,4]);
    assert_eq(merge_two_lists(ListNode.from_array([]), ListNode.from_array([])), null);
    assert_eq(!merge_two_lists(ListNode.from_array([]), ListNode.from_array([0])).to_array(), [0]);
}
