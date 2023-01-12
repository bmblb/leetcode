export function assert_eq(a, b) {
    if (Array.isArray(a) && Array.isArray(b)) {
        if (a.toString() !== b.toString()) {
            throw new Error(`Got ${a}, expected ${b}`);
        }
    }
    else if (a != b) {
        throw new Error(`Failed ${a} not equal to ${b}`);
    }
}

export class ListNode {
    static from_array(arr) {
        if (arr.length) {
            const start = new ListNode(arr[0]);
            let node = start;
            arr.slice(1).forEach(v => {
                node = node.next = new ListNode(v);
            });

            return start;
        }
    }

    constructor(val, next) {
        this.val = (val===undefined ? 0 : val);
        this.next = (next===undefined ? null : next);
    }

    to_array() {
        let node = this;
        const result = [node.val];
        while (node.next) {
            node = node.next;
            result.push(node.val);
        }
        return result;
    }
}
