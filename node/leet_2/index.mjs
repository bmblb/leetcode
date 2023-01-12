import { assert_eq, ListNode } from "../utils.mjs";

function addTwoNumbers(l1, l2) {
    let sum = 0;
    let current = new ListNode(0);
    let result = current;
    
    while(l1 || l2) {
        
        if(l1) {
            sum += l1.val;
            l1 = l1.next;
        }
        
        if(l2) {
            sum += l2.val;
            l2 = l2.next;
        }
        
        current.next = new ListNode(sum % 10);
        current = current.next;
        
        sum = sum > 9 ? 1 : 0;
    }
    
    if(sum) {
        current.next = new ListNode(sum);
    }
    
    return result.next;
};

export function run() {
    // [2,4,3]
    // [5,6,4]
    // [7,0,8]
    const res = addTwoNumbers(
        new ListNode(2, new ListNode(4, new ListNode(3))),
        new ListNode(5, new ListNode(6, new ListNode(4)))
    );
    
    assert_eq(res.to_array(), [7,0,8]);
}

export function test() {
    // [2,4,3]
    // [5,6,4]
    // [7,0,8]
    const res = addTwoNumbers(
        new ListNode(2, new ListNode(4, new ListNode(3))),
        new ListNode(5, new ListNode(6), new ListNode(4))
    );
    
    assert_eq(res.to_array(), [7,0,8,1]);
}
