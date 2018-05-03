extern crate num_bigint;
extern crate num_integer;
// Usage:
//  cargo run -- < input > output
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num_doors: u32 = input.trim().parse().unwrap();

        let mut doors = Vec::with_capacity(num_doors as usize);
        for j in 0..num_doors {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let pt: Vec<_> = input
                .trim()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();

            let d = Door::new(pt[0], pt[1], j);
            doors.push(d);
        }

        let mcm = merge_all_doors(&doors);
        println!(
            "Case #{}: {}",
            i + 1,
            mcm.map(|x| format!("{}", (&x.p - &x.t) % &x.p))
                .unwrap_or("NEVER".into())
        );
    }
}

#[derive(Clone, Debug)]
struct Door {
    p: BigUint,
    t: BigUint,
}

impl Door {
    fn new(p: u32, t: u32, j: u32) -> Door {
        let p = p.into();
        // Sum door index to t so the problem becomes
        // finding when all doors are open at the same time
        let t = (t + j) % &p;
        Door { p, t }
    }
    fn open_at(&self, x: &BigUint) -> bool {
        (&self.t + x) % &self.p == 0u32.into()
    }
    fn merge(a: Door, b: Door) -> Option<Door> {
        let (a, b) = if a.p > b.p { (a, b) } else { (b, a) };
        let p = a.p.lcm(&b.p);

        let mut t = &a.p - &a.t;
        let mut steps = &p / &a.p;
        let mut found = false;
        if steps == 1u32.into() {
            if b.open_at(&t) {
                found = true;
            }
        } else {
            let gcd = a.p.gcd(&b.p);
            if &a.t % &gcd != &b.t % &gcd {
                return None;
            }
            let mut at = (&a.p - &a.t) % &a.p;
            let mut bt = (&b.p - &b.t) % &b.p;
            // Move b.t to 0
            let offset = bt;
            bt = 0u32.into();
            at = (at + &p - &offset) % &a.p;
            //let t_t = (&b.p * (&a.p % &b.p)) % &p;
            //let t_t = &a.p + 1u32;
            // t_t % b == 0 and t_t % a == 1
            let t_t = &b.p * mod_inv(&b.p, &a.p);
            /*
            println!("{:?}", a);
            println!("{:?}", b);
            println!("tt: {}, offset: {}, at: {}, bt: {}", t_t, offset, at, bt);
            */
            t = (t_t * (&at)) % &p;
            t = (t + &offset) % &p;
            if a.open_at(&t) == b.open_at(&t) {
                found = true;
            } else {
                //panic!("WTF!");
                //println!("Using old algorithm...");
                t = &a.p - &a.t;
                while steps > 0u32.into() {
                    if b.open_at(&t) {
                        found = true;
                        break;
                    }
                    t = (t + &a.p) % &p;
                    steps -= 1u32;
                }
            }
        }

        if found {
            let t = (&p - &t) % &p;
            Some(Door { p, t })
        } else {
            None
        }
    }
}

fn merge_all_doors(a: &[Door]) -> Option<Door> {
    let mut acc = a[0].clone();
    for x in a.iter().skip(1) {
        if let Some(a) = Door::merge(acc, x.clone()) {
            acc = a;
        } else {
            return None;
        }
    }

    Some(acc)
}

// Modular inverse, adapted from
// https://rosettacode.org/wiki/Modular_inverse#Rust
fn mod_inv(a: &BigUint, module: &BigUint) -> BigUint {
    let sign = num_bigint::Sign::Plus;
    let module = BigInt::from_biguint(sign, module.clone());
    let mut mn: (BigInt, BigInt) = (module.clone(), BigInt::from_biguint(sign, a.clone()));
    let mut xy: (BigInt, BigInt) = (0u32.into(), 1u32.into());

    while mn.1 != 0u32.into() {
        xy = (xy.1.clone(), xy.0 - (mn.0.clone() / mn.1.clone()) * xy.1);
        mn = (mn.1.clone(), mn.0 % mn.1);
    }

    while xy.0 < 0u32.into() {
        xy.0 = xy.0 + &module;
    }
    xy.0.to_biguint().unwrap()
}
