// Usage:
//  rustc -O p04.rs
//  ./p04 < input > output
//  # The -O is important because it disables overflow checking
//  # and also the program runs faster
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;
use std::u64;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let grid_size: Vec<_> = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let mut lines = vec![];
        for _ in 0..grid_size[0] {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let line = input.trim().to_string();
            lines.push(line);
        }

        let m = Map::new(grid_size[1], grid_size[0], &lines);
        let mut cost = "IMPOSSIBLE".into();
        let a = shortest_path(&m, m.s, m.p);
        if let Some(a) = a {
            let b = shortest_path(&m, m.p, m.d);
            if let Some(b) = b {
                cost = format!("{}", a + b);
            }
        }

        println!("Case #{}: {}", i + 1, cost);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Material {
    Ground,
    Trampoline,
    Lava,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Map {
    size: (u32, u32),
    square: Vec<Material>,
    s: Pos,
    p: Pos,
    d: Pos,
}

impl Map {
    fn new(size_x: u32, size_y: u32, l: &[String]) -> Map {
        let mut s = Pos {
            x: size_x,
            y: size_y,
        };
        let mut p = s;
        let mut d = s;
        let mut square = vec![];
        for (y, m) in l.iter().enumerate() {
            for (x, c) in m.chars().enumerate() {
                let s = match c {
                    '.' => Material::Ground,
                    '*' => Material::Trampoline,
                    '#' => Material::Lava,
                    'S' => {
                        s = Pos {
                            x: x as u32,
                            y: y as u32,
                        };
                        Material::Ground
                    }
                    'P' => {
                        p = Pos {
                            x: x as u32,
                            y: y as u32,
                        };
                        Material::Ground
                    }
                    'D' => {
                        d = Pos {
                            x: x as u32,
                            y: y as u32,
                        };
                        Material::Ground
                    }
                    _ => panic!("Invalid char: {}", c),
                };
                square.push(s);
            }
        }
        Map {
            size: (size_x, size_y),
            square,
            s,
            p,
            d,
        }
    }
    fn get_edges(&self, p: &Pos) -> Option<Vec<Edge>> {
        if let Some(x) = self.get_square(p) {
            if x == Material::Trampoline {
                let mut pos = vec![];
                pos.push(Pos {
                    x: p.x + 4,
                    y: p.y + 2,
                });
                pos.push(Pos {
                    x: p.x + 4,
                    y: p.y - 2,
                });
                pos.push(Pos {
                    x: p.x - 4,
                    y: p.y + 2,
                });
                pos.push(Pos {
                    x: p.x - 4,
                    y: p.y - 2,
                });
                pos.push(Pos {
                    x: p.x + 2,
                    y: p.y + 4,
                });
                pos.push(Pos {
                    x: p.x + 2,
                    y: p.y - 4,
                });
                pos.push(Pos {
                    x: p.x - 2,
                    y: p.y + 4,
                });
                pos.push(Pos {
                    x: p.x - 2,
                    y: p.y - 4,
                });

                pos.retain(|&p| p.x < self.size.0 && p.y < self.size.1);

                Some(pos.into_iter().map(|p| Edge { cost: 1, node: p }).collect())
            } else if x == Material::Ground {
                let mut pos = vec![];
                pos.push(Pos {
                    x: p.x + 2,
                    y: p.y + 1,
                });
                pos.push(Pos {
                    x: p.x + 2,
                    y: p.y - 1,
                });
                pos.push(Pos {
                    x: p.x - 2,
                    y: p.y + 1,
                });
                pos.push(Pos {
                    x: p.x - 2,
                    y: p.y - 1,
                });
                pos.push(Pos {
                    x: p.x + 1,
                    y: p.y + 2,
                });
                pos.push(Pos {
                    x: p.x + 1,
                    y: p.y - 2,
                });
                pos.push(Pos {
                    x: p.x - 1,
                    y: p.y + 2,
                });
                pos.push(Pos {
                    x: p.x - 1,
                    y: p.y - 2,
                });

                pos.retain(|&p| p.x < self.size.0 && p.y < self.size.1);

                Some(pos.into_iter().map(|p| Edge { cost: 1, node: p }).collect())
            } else {
                // If on lava, die
                None
            }
        } else {
            None
        }
    }
    fn get_square(&self, p: &Pos) -> Option<Material> {
        if p.x >= self.size.0 || p.y >= self.size.1 {
            None
        } else {
            let idx = p.x as usize + p.y as usize * self.size.0 as usize;
            Some(self.square[idx])
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u64,
    position: Pos,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Edge {
    node: Pos,
    cost: u64,
}

// Dijkstra's shortest path algorithm.
// Based on the example from https://doc.rust-lang.org/std/collections/binary_heap/

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue.
fn shortest_path(adj_list: &Map, start: Pos, goal: Pos) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    //let mut dist: Vec<_> = (0..goal+1).map(|_| u64::MAX).collect();
    let mut dist: HashMap<Pos, u64> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost as usize);
        }

        // Important as we may have already found a better way
        if cost > *dist.get(&position).unwrap_or(&u64::MAX) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        if let Some(edges) = adj_list.get_edges(&position) {
            for edge in edges.iter() {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                // If so, add it to the frontier and continue
                if next.cost < *dist.get(&next.position).unwrap_or(&u64::MAX) {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);
                }
            }
        }
    }

    // Goal not reachable
    None
}
