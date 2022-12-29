use std::collections::HashMap;
use std::env;
use std::time::{Instant, Duration};

mod leet_1;
mod leet_2;
mod leet_9;

fn format_duration(value: Duration) -> String {
    let units = vec!["ns", "μs", "ms", "s"];
    let mut ns = value.as_nanos() as u64;
    let mut result = vec![];

    for name in units {
        let val = ns % 1000;
        result.push((name, val));
        ns = ns / 1000;
    }

    result.iter().filter(|(_, value)| {
        *value > 0
    }).map(|(name, value)| {
        format!("{value}{name}")
    }).rev().collect::<Vec<String>>().join(" ")
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let map: HashMap<&str, fn()> = HashMap::from([
        ("leet_2", leet_2::main as fn()),
        ("leet_9", leet_9::main as fn())
    ]);    

    if args.len() > 0 {
        let name = args[0].as_str();

        if let Some(f) = map.get(name) {
            let start = Instant::now();
            f();
            let end = Instant::now();

            println!("{name} took {}", format_duration(end.checked_duration_since(start).unwrap()));
        }
        else {
            println!("Cannot find test {}", args[0]);
        }
    }
}
