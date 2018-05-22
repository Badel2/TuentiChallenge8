// Usage:
//  rustc -O p06.rs
//  ./p06 < input > output
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::rc::Rc;
use std::u32;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num_notes: u32 = input.trim().parse().unwrap();

        let mut notes = Vec::with_capacity(num_notes as usize);
        for _ in 0..num_notes {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let xlsp: Vec<u32> = input
                .trim()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();
            let note = Note::new(xlsp[0], xlsp[1], xlsp[2], xlsp[3]);
            notes.push(note);
        }

        let mut map = Map::new(notes, 0);
        let root = map.into_graph();
        let max_score = root.score_plus_max_of_children();

        println!("Case #{}: {}", i + 1, max_score);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Note {
    x: u32,
    l: u32,
    s: u32,
    p: u32,
    t0: u32,
    t1: u32,
}

impl Note {
    fn new(x: u32, l: u32, s: u32, p: u32) -> Note {
        let t0 = x / s;
        let t1 = (x + l) / s;
        Note { x, l, s, p, t0, t1 }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Interval {
    t0: u32,
    t1: u32,
}

impl Interval {
    fn from_note(n: &Note) -> Interval {
        Interval { t0: n.t0, t1: n.t1 }
    }
}

//#[derive(Clone, Debug)]
struct Node {
    t: Interval,
    s: u32,
    ch: Cell<Vec<Rc<Node>>>,
    max_of_children: Cell<Option<u32>>,
}

impl Node {
    fn new(t: Interval, s: u32) -> Node {
        Node {
            t,
            s,
            ch: Cell::new(vec![]),
            max_of_children: Cell::new(None),
        }
    }
    fn root() -> Node {
        Node {
            t: Interval { t0: 0, t1: 0 },
            s: 0,
            ch: Cell::new(vec![]),
            max_of_children: Cell::new(None),
        }
    }
    fn score_plus_max_of_children(&self) -> u32 {
        // Use cached result when possible
        if let Some(s) = self.max_of_children.get() {
            return self.s + s;
        }

        let ch = self.ch.take();
        let mut mc = 0;
        for n in ch.iter() {
            let a = n.score_plus_max_of_children();
            mc = max(mc, a);
        }
        self.ch.set(ch);

        self.max_of_children.set(Some(mc));

        self.s + mc
    }
}

#[derive(Clone, Debug)]
struct Map {
    score: u32,
    intervals: BTreeMap<Interval, u32>,
}

impl Map {
    fn new(notes: Vec<Note>, score: u32) -> Map {
        let mut intervals = BTreeMap::new();
        for n in notes {
            let a = Interval::from_note(&n);
            *intervals.entry(a).or_insert(0u32) += n.p;
        }
        Map { score, intervals }
    }
    // Build a graph where each node is only connected to the immediate nodes
    fn into_graph(self) -> Node {
        let rcn: HashMap<_, _> = self.intervals
            .iter()
            .map(|(i, s)| (i, Rc::new(Node::new(*i, *s))))
            .collect();

        for (i, n) in rcn.iter() {
            let (first, _first_s) = (*i, (n).s);
            let limit = Interval {
                t0: first.t1 + 1,
                t1: 0,
            };
            let mut true_limit = u32::MAX;
            for (x, _s) in self.intervals.range(limit..) {
                true_limit = min(true_limit, x.t1);
                if x.t0 > true_limit {
                    break;
                }
            }
            let mut children = vec![];
            let limit_up = Interval {
                t0: true_limit,
                t1: u32::MAX,
            };
            for (x, _s) in self.intervals.range(limit..limit_up) {
                children.push(Rc::clone(&rcn[x]));
            }

            n.ch.set(children);
        }

        let mut root = Node::root();

        let (first, _first_s) = self.intervals.iter().next().unwrap();
        let mut true_limit = first.t1;
        for (x, _s) in self.intervals.range(first..) {
            true_limit = min(true_limit, x.t1);
            if x.t0 > true_limit {
                break;
            }
        }
        let mut children = vec![];
        let limit_up = Interval {
            t0: true_limit,
            t1: u32::MAX,
        };
        for (x, _s) in self.intervals.range(first..&limit_up) {
            children.push(Rc::clone(&rcn[x]));
        }

        root.ch = Cell::new(children);

        root
    }
}
