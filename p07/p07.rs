use std::io;

// Convert PikaLang to BrainFuck
//
// rustc p07.rs
// ./p07 < input
//
// (BrainFuck code execution is left as an exersice for the reader)
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", pika_to_bf(&input));
    println!("REVERSED:");
    println!("{}", pika_to_bf(&input).chars().rev().collect::<String>());
}

fn pika_to_bf(p: &str) -> String {
    let ref p = str::replace(p, "pikachu", ".");
    let ref p = str::replace(p, "pikapi", ",");
    let ref p = str::replace(p, "pipi", ">");
    let ref p = str::replace(p, "pichu", "<");
    let ref p = str::replace(p, "pika", "[");
    let ref p = str::replace(p, "pi", "+");
    let ref p = str::replace(p, "ka", "-");
    let p = str::replace(p, "chu", "]");

    p
}
