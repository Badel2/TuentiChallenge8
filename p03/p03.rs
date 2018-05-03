// Usage:
//  rustc p03.rs
//  ./p03 < input > output
use std::io;

fn main() {
    //println!("{:#?}", Scale::all_scales().iter().map(|s| s.name()).collect::<Vec<_>>());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: u32 = input.trim().parse().unwrap();

    for i in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num_notes: u32 = input.trim().parse().unwrap();

        let nm = if num_notes > 0 {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input
                .trim()
                .split(" ")
                .map(|s| Note::new(s))
                .collect()
        } else {
            vec![]
        };

        println!("Case #{}: {}", i + 1, all_scales_containing(&nm));
    }
}

fn all_scales_containing(n: &[Note]) -> String {
    let mut a = Scale::all_scales();
    let mut uniq_notes = n.to_vec();
    uniq_notes.sort();
    uniq_notes.dedup();

    for ref n in uniq_notes {
        a.retain(|ref s| s.notes.contains(n));
    }

    if a.len() == 0 {
        return "None".into();
    }

    let mut s = a[0].name();
    for x in a.iter().skip(1) {
        s.push_str(" ");
        s.push_str(&x.name());
    }

    s
}

#[derive(Clone, Debug)]
struct Scale {
    notes: Vec<Note>,
}

impl Scale {
    fn new_major(tonic: Note) -> Scale {
        let a = Note::all_notes();
        let z = a.iter().position(|&x| x == tonic).unwrap();
        let notes = vec![
            a[z], a[(z+2)%12], a[(z+4)%12], a[(z+5)%12], a[(z+7)%12],
            a[(z+9)%12], a[(z+11)%12]
        ];


        Scale { notes }
    }
    fn new_minor(tonic: Note) -> Scale {
        let a = Note::all_notes();
        let z = a.iter().position(|&x| x == tonic).unwrap();
        let notes = vec![
            a[z], a[(z+2)%12], a[(z+3)%12], a[(z+5)%12], a[(z+7)%12],
            a[(z+8)%12], a[(z+10)%12]
        ];


        Scale { notes }
    }
    fn all_scales() -> Vec<Scale> {
        let mut s = vec![];
        let a = Note::all_notes();
        for n in a {
            s.push(Scale::new_major(n));
        }
        let a = Note::all_notes();
        for n in a {
            s.push(Scale::new_minor(n));
        }

        s
    }
    fn name(&self) -> String {
        let major = ((12 + self.notes[2] as u8 - self.notes[1] as u8) % 12) == 2;
        if major {
            format!("M{}", self.notes[0])
        } else {
            format!("m{}", self.notes[0])
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Note {
    A = 0,
    As,
    B,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
}

impl Note {
    fn new(s: &str) -> Note {
        use Note::*;
        match s {
            "A" => A,
            "A#" => As,
            "Bb" => As,
            "B" => B,
            "B#" => C,
            "Cb" => B,
            "C" => C,
            "C#" => Cs,
            "Db" => Cs,
            "D" => D,
            "D#" => Ds,
            "Eb" => Ds,
            "E" => E,
            "E#" => F,
            "Fb" => E,
            "F" => F,
            "F#" => Fs,
            "Gb" => Fs,
            "G" => G,
            "G#" => Gs,
            "Ab" => Gs,
            s => panic!("Nooo: {}", s),
        }
    }
    fn all_notes() -> Vec<Note> {
        use Note::*;
        vec![
            A,
            As,
            B,
            C,
            Cs,
            D,
            Ds,
            E,
            F,
            Fs,
            G,
            Gs,
        ]
    }
}

impl std::fmt::Display for Note {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Note::*;
        let s = match self {
            A => "A",
            As => "A#",
            B => "B",
            C => "C",
            Cs => "C#",
            D => "D",
            Ds => "D#",
            E => "E",
            F => "F",
            Fs => "F#",
            G => "G",
            Gs => "G#",
        };

        write!(f, "{}", s)
    }
}
