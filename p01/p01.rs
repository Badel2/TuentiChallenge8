// Usage:
//  rustc p01.rs
//  ./p01 < input > output
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nm: Vec<u32> = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let n = nm[0];
        let m = nm[1];
        println!("Case #{}: {}", i + 1, waffles(n, m));
    }
}

fn waffles(n: u32, m: u32) -> u32 {
    (n - 1) * (m - 1)
}
