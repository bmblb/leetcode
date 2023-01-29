import { assert_eq } from "../utils.mjs";

function remove_element(nums, val) {
    for (let i = 0; i < nums.length;) {
        if (nums[i] == val) {
            nums.splice(i, 1);
        }
        else {
            i += 1;
        }
    }

    return nums.length;
}

export function run() {
    let v1 = [3,2,2,3];
    let k = remove_element(v1, 3);

    assert_eq(k, 2);
    assert_eq(v1, [2,2]);

    v1 = [0,1,2,2,3,0,4,2];
    k = remove_element(v1, 2);

    assert_eq(k, 5);
    assert_eq(v1, [0,1,3,0,4]);
}
