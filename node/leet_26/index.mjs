import { assert_eq } from "../utils.mjs";

function remove_duplicates(nums) {
    for (let i = 1; i < nums.length;) {
        if (nums[i - 1] == nums[i]) {
            nums.splice(i, 1);
        }
        else {
            i += 1;
        }
    }

    return nums.length;
}

export function run() {
    let v1 = [1,1,2];
    let k = remove_duplicates(v1);
    
    assert_eq(k, 2);
    assert_eq(v1, [1,2]);

    v1 = [0,0,1,1,1,2,2,3,3,4];
    k = remove_duplicates(v1);

    assert_eq(k, 5);
    assert_eq(v1, [0,1,2,3,4]);
}
