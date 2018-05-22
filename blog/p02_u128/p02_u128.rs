use std::io;

fn main() {
    // We just use a lookup table
    let l = lut();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let secret = input.trim().chars().count();
        println!("Case #{}: {}", i + 1, l[secret]);
    }
}

// Precalculate all possible values for n in [2, 26]
fn lut() -> Vec<u128> {
    let mut v = vec![];
    for i in 0..27 {
        v.push(nazi_base(i));
    }

    v
}

fn nazi_base(base: u64) -> u128 {
    if base < 2 {
        return 0u64.into();
    }

    let base = base as u128;
    let mut min = 0;
    let mut max = 0;
    let mut p = 1;
    for x in 0..base {
        max += x * p;
        // No leading zeros: transform 012 into 102
        min += match base - x - 1 {
            0 => 1,
            1 => 0,
            n => n,
        } * p;
        p *= base;
    }

    max - min
}
