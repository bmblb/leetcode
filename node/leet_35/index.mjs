import { assert_eq } from "../utils.mjs";

function search_insert(nums, target) {
    let lo = 0, hi = nums.length;

    while(lo < hi) {
        let mid = lo + Math.floor((hi-lo)/2);

        if (target > nums[mid]) {
            lo = mid + 1
        } else {
            hi = mid
        }
    }
    return lo;
}

export function run() {
    assert_eq(search_insert([1,3,5,6], 5), 2);
    assert_eq(search_insert([1,3,5,6], 2), 1);
    assert_eq(search_insert([1,3,5,6], 7), 4);
}
