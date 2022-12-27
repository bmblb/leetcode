#[derive(Clone, Copy)]
struct Node {
    id: u32,
    name: &'static str,
    formula: fn() -> ()
}

fn main() {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i.try_into().unwrap());
                    result.push(j.try_into().unwrap());
                }
            }
        }

        return result;
    }

    let result = two_sum(vec![2,7,11,15], 9);

    assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(two_sum(vec![3,3], 6), vec![0, 1]);
}
