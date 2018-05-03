// Usage:
//  rustc -O p10.rs
//  ./p10 < input > output
use std::collections::{BTreeMap, HashMap};
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
        let mut bgr = BTreeMap::new();
        for (k, mut v) in gr.into_iter() {
            v.sort();
            //vv[k as usize] = v;
            bgr.insert(k, v);
        }

        let x = Dance::new(p, bgr);
        let mut cache = Cache::new();

        //println!("p: {}, g: {}", p, g);
        println!("Case #{}: {}", i + 1, x.calc(&mut cache));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Dance {
    p: u32,
    // Offset-based grudges:
    // 4,3: g[3].push(4-3);
    //g: &'a [Vec<u32>],
    // Unfortunately using a cache when the key has lifetimes is a pain
    // So we use a BTreeMap because a HashMap can't be hashed
    // (ironic, he could hash others but not himself)
    g: BTreeMap<u32, Vec<u32>>,
}

impl Dance {
    fn new(p: u32, g: BTreeMap<u32, Vec<u32>>) -> Dance {
        Dance { p, g }
    }
    fn calc(&self, cache: &mut Cache) -> u32 {
        // The cache is magic, it converts O(n!) into O(n^2)
        if let Some(x) = cache.get(self) {
            return x;
        }
        //println!("{:#?}", self);
        if self.p % 2 != 0 {
            return 0;
        }

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
        // recursion!
        let mut acc: u64 = 0;
        let mut i = 1;
        let mut j = 0;
        let empty_vec = vec![];
        let mut next_gr = self.g.get(&0).unwrap_or(&empty_vec).get(j);
        while i < self.p {
            // Skip impossible pairs
            if let Some(nid) = next_gr {
                if i == *nid {
                    j += 1;
                    next_gr = self.g[&0].get(j);
                    i += 2;
                    continue;
                }
            }
            // Num combinations = product of combinations of each part
            if let Some((dl, dr)) = self.split_into_two(0, i) {
                acc += (dl.calc(cache) as u64) * (dr.calc(cache) as u64);
                acc = acc % 1_000_000_007;
            }
            i += 2;
        }

        let acc = acc as u32;
        let clon = self.clone();
        cache.insert(clon, acc);

        acc
    }
    fn split_into_two(&self, i: u32, j: u32) -> Option<(Dance, Dance)> {
        // Assumes there is no grudge between i and j
        if i != 0 {
            unimplemented!()
        }
        let (i, j) = if i < j { (i, j) } else { (j, i) };

        /*
        let drp = j - i - 1;
        let dlp = self.p - drp - 2;
        let drv = &self.g[i as usize+1..j as usize];
        let dlv = &self.g[j as usize+1..self.p as usize];
        let dr = Dance::new(drp, drv);
        let dl = Dance::new(dlp, dlv);
        */
        // We can avoid this operations by storing self.g as Rc<HashMap> + offset,
        // or even &[Vec<u32>], as above
        let drp = j - i - 1;
        let mut drg = BTreeMap::new();
        if drp > 0 {
            for x in i + 1..j {
                if let Some(grs) = self.g.get(&x) {
                    for &gr in grs {
                        if gr < j - x {
                            drg.entry(x - 1).or_insert(vec![]).push(gr);
                        }
                    }
                }
            }
        }
        let dr = Dance::new(drp, drg);

        let dlp = self.p - drp - 2;
        let mut dlg = BTreeMap::new();
        if dlp > 0 {
            for x in j + 1..self.p {
                if let Some(grs) = self.g.get(&x) {
                    for &gr in grs {
                        dlg.entry(x - j - 1).or_insert(vec![]).push(gr);
                    }
                }
            }
        }
        let dl = Dance::new(dlp, dlg);

        Some((dl, dr))
    }
}

struct Cache {
    c: HashMap<Dance, u32>,
}

impl Cache {
    fn new() -> Cache {
        Cache { c: HashMap::new() }
    }
    fn get(&self, x: &Dance) -> Option<u32> {
        self.c.get(x).map(|a| *a)
    }
    fn insert(&mut self, k: Dance, v: u32) {
        self.c.insert(k, v);
    }
}
