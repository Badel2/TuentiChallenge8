// Usage:
//  rustc -O p10.rs
//  ./p10 < input > output
use std::collections::{HashMap};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let pg: Vec<u32> = input
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();

        let p = pg[0];
        let g = pg[1];
        let mut gr = HashMap::new();
        for _x in 0..g {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let ab: Vec<u32> = input
                .trim()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();
            let a = ab[0];
            let b = ab[1];
            let (a, b) = if a < b { (a, b) } else { (b, a) };
            if a % 2 != b % 2 {
                // 0 and 2 cannot form a pair anyway
                // offsets are relative
                gr.entry(a).or_insert(vec![]).push(b - a);
            }
        }

        //let mut vv = vec![vec![]; p as usize];
        let mut bgr = HashMap::new();
        for (k, mut v) in gr.into_iter() {
            v.sort();
            //vv[k as usize] = v;
            bgr.insert(k, v);
        }

        let mut x = Dance::new(bgr);

        //println!("p: {}, g: {}", p, g);
        println!("Case #{}: {}", i + 1, x.calc(p));
    }
}

struct Dance {
    g: HashMap<u32, Vec<u32>>,
    cache: Cache,
}

impl Dance {
    fn new(g: HashMap<u32, Vec<u32>>) -> Dance {
        let cache = Cache::new();
        
        Dance { g, cache }
    }
    fn calc(&mut self, p: u32) -> u32 {
        let x = DancePart { offset: 0, p: p };

        x.calc(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct DancePart {
    offset: u32,
    p: u32,
}

impl DancePart {
    fn calc(&self, dance: &mut Dance) -> u32 {
        let Dance { ref g, ref mut cache } = dance;
        // The cache is magic, it converts O(n!) into O(n^2)
        if let Some(x) = cache.get(self) {
            return x;
        }
        //println!("{:#?}", self);
        if self.p % 2 != 0 {
            return 0;
        }

        if self.p == 0 {
            return 1;
        }

        /*
        let n = self.p as usize / 2;
        const CATALAN: &[u32] = &[1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];
        if n < CATALAN.len() {
            //let no_grudges = self.g.iter().all(|x| x.get(0).unwrap_or(&self.p) >= &self.p);
            let no_grudges = self.g.is_empty();
            if no_grudges {
                //println!("{}", self.p);
                return CATALAN[n];
            }
        }
        */

        // recursion!
        let mut acc: u64 = 0;
        let mut i = 1;
        let mut j = 0;
        let empty_vec = vec![];
        let mut next_gr = g.get(&self.offset).unwrap_or(&empty_vec).get(j);
        while i < self.p {
            // Skip impossible pairs
            if let Some(nid) = next_gr {
                if i == *nid {
                    j += 1;
                    next_gr = g[&self.offset].get(j);
                    i += 2;
                    continue;
                }
            }
            // Num combinations = product of combinations of each part
            if let Some((dl, dr)) = self.split_into_two(0, i) {
                acc += (dl.calc(dance) as u64) * (dr.calc(dance) as u64);
                acc = acc % 1_000_000_007;
            }
            i += 2;
        }

        let acc = acc as u32;
        let clon = self.clone();
        dance.cache.insert(clon, acc);

        acc
    }
    fn split_into_two(&self, i: u32, j: u32) -> Option<(DancePart, DancePart)> {
        // Assumes there is no grudge between i and j
        if i != 0 {
            unimplemented!()
        }
        let (i, j) = if i < j { (i, j) } else { (j, i) };

        let drp = j - i - 1;
        let dlp = self.p - drp - 2;

        let dr = DancePart { offset: self.offset + 1, p: drp };
        let dl = DancePart { offset: self.offset + 1 + j, p: dlp };

        Some((dl, dr))
    }
}

struct Cache {
    c: HashMap<DancePart, u32>,
}

impl Cache {
    fn new() -> Cache {
        Cache { c: HashMap::new() }
    }
    fn get(&self, x: &DancePart) -> Option<u32> {
        self.c.get(x).map(|a| *a)
    }
    fn insert(&mut self, k: DancePart, v: u32) {
        self.c.insert(k, v);
    }
}
