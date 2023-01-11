// Cons: long solution, extra string conversion
// Pros: fn will iterate all strings char by char bailing out early
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut iterators: Vec<_> = strs.iter().map(|value| {
        value.chars()
    }).collect();

    let mut result: Vec<char> = Vec::with_capacity(200);

    let mut main_iter = strs.get(0).unwrap().chars();

    loop {
        if let Some(c) = main_iter.next() {
            let matched = iterators.iter_mut().map(|iter| {
                iter.next()
             }).all(|v| {
                if let Some(val) = v {
                    val == c
                }
                else {
                    false
                }
             });

             if matched {
                result.push(c);
             }
             else {
                break;
             }
        }
        else {
            break;
        }
    }

    let str = String::from("");

    result.iter().fold(str, |mut result, v| {
        result.push(*v);
        result
     })
}

// fn longest_common_prefix(strs: Vec<String>) -> String { 
//     match strs.is_empty() {
//         true => "".to_string(),
//         _ => {
//             strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
//                  acc
//                     .chars()
//                     .zip(x.chars())
//                     .take_while(|(x,y)| x == y)
//                     .map(|(x, _)| x)
//                     .collect()
//             })
//         }
//     }
// }

pub fn main() {
    assert_eq!(
        longest_common_prefix(
            vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]
        ),
         "fl"
    );
    
    assert_eq!(
        longest_common_prefix(
            vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]
        ),
         ""
    );
}
