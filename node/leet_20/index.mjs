import { assert_eq } from "../utils.mjs";

function is_valid(str) {
    const stack = [];

    for (const c in str) {
        if (c === '(') {
            stack.push(')');
        }
        else if (c === '[') {
            stack.push(']');
        }
        else if (c === '{') {
            stack.push('}');
        }
        else if (c !== stack.pop()) {
            return false;
        }
    }

    return true;
};

export function run() {
    assert_eq(is_valid("()"));
    assert_eq(is_valid("()[]{}"));
    assert_eq(!is_valid("(]"));
}
