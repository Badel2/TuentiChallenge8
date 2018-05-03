// Usage:
//  rustc p05.rs
//  ./p05 < input > output
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect(("52.49.91.111", 3241)).unwrap();
    let mut buffer = vec![0u8; 1024];

    //let state = b"TEST\n";
    let state = b"SUBMIT\n";

    'tcp: loop {
        match stream.read(&mut buffer) {
            Ok(0) => break 'tcp,
            Ok(n) => {
                let buf = &buffer[0..n];
                let s = String::from_utf8_lossy(&buf);
                println!("{}", s);
                if let Some(_) = find_subsequence(&buf, br#"> Please, provide "TEST" or "SUBMIT""#)
                {
                    stream.write(state).unwrap();
                } else if buf[0] != b'>' {
                    // DATA
                    let parts: Vec<_> = s.trim()
                        .split(" ")
                        .enumerate()
                        .map(|(i, s)| Part::new(i + 1, s.as_bytes().to_vec()))
                        .collect();

                    println!("PARTS: {:#?}", parts);

                    let idx = find_original_samples(&parts);
                    println!("Result: {}", idx);
                    stream.write(idx.as_bytes()).unwrap();
                }
            }
            Err(e) => panic!("Error reading stream: {:?}", e),
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn share_common_start(a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
    if a[0..b.len()] == *b {
        Some(a[b.len()..a.len()].to_vec())
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    idx: usize,
    data: Vec<u8>,
}

impl Part {
    fn new(idx: usize, data: Vec<u8>) -> Part {
        for d in &data {
            assert!(b"ATCGatcg".contains(d), "Invalid data: {}", d);
        }
        Part { idx, data }
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }
}

/* We need to find the longest subsequence which appears twice.
 * We can begin by the beginning or by the end:
 * Assuming the second sequence must begin at a part start,
 * so the first sequence must end at a part end,
 * We can find potential starts and potential ends.
 *
 * Nice recursive algorithm
 */

fn find_original_samples(p: &[Part]) -> String {
    let mut v = find_samples(&[], p)
        .expect("It's guaranteed that there is at least one valid solution in each case.");
    v.sort();
    let mut s = format!("{}", v[0]);
    for x in v.iter().skip(1) {
        s.push_str(&format!(",{}", x));
    }
    s.push_str("\n");

    s
}

fn find_samples(start: &[u8], p: &[Part]) -> Option<Vec<usize>> {
    println!("Searching for start: {}", Part::new(0, start.to_vec()));
    println!("Remaining parts: ");
    for x in p {
        println!("{}: {}", x.idx, x);
    }
    if p.len() == 0 && start.len() > 0 {
        return None;
    }

    if start.len() == 0 {
        for x in p {
            for y in p.iter().skip_while(|i| i.idx <= x.idx) {
                if let Some(start) = share_common_start(&x.data, &y.data) {
                    let mut rem_p = p.to_vec();
                    // Remove x and y from p
                    rem_p.retain(|t| t != x && t != y);
                    if let Some(indexes) = find_samples(&start, &rem_p) {
                        let mut ix = indexes;
                        ix.push(x.idx);
                        ix.push(y.idx);
                        return Some(ix);
                    }
                }
            }
        }
        return Some(vec![]);
    }

    for x in p {
        if let Some(new_start) = share_common_start(&x.data, start) {
            let mut rem_p = p.to_vec();
            rem_p.retain(|t| t != x);
            if let Some(indexes) = find_samples(&new_start, &rem_p) {
                let mut ix = indexes;
                ix.push(x.idx);
                return Some(ix);
            }
        }
    }

    None
}
