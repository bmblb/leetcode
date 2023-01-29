fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    // let mut i = 0;

    // if nums.iter().enumerate().all(|(index, v)| {
    //     if *v >= target {
    //         i = index as i32;
    //         false
    //     }
    //     else {
    //         true
    //     }
    // }) {
    //     nums.len() as i32
    // }
    // else {
    //     i
    // }
}

pub fn main() {
    assert_eq!(search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
    assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
}
