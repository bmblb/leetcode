fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|v| {
        *v != val
    });

    nums.len() as i32
}

pub fn main() {
    let mut v1 = vec![3,2,2,3];
    let k = remove_element(&mut v1, 3);

    assert_eq!(k, 2);
    assert_eq!(v1, vec![2,2]);

    let mut v1 = vec![0,1,2,2,3,0,4,2];
    let k = remove_element(&mut v1, 2);

    assert_eq!(k, 5);
    assert_eq!(v1, [0,1,3,0,4]);
}
