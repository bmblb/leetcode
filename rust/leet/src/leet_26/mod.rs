fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev = i32::MAX;

    nums.retain(|v| {
        let res = *v != prev;
        prev = *v;
        res
    });

    nums.len() as i32
}

pub fn main() {
    let mut v1 = vec![1,1,2];
    let k = remove_duplicates(&mut v1);
    
    assert_eq!(k, 2);
    assert_eq!(v1, vec![1,2]);

    let mut v1 = vec![0,0,1,1,1,2,2,3,3,4];
    let k = remove_duplicates(&mut v1);

    assert_eq!(k, 5);
    assert_eq!(v1, vec![0,1,2,3,4]);
}
