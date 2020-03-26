#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::HashMap;

fn read() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_split<T : std::str::FromStr>(s: String) -> Vec<T> {
    s.trim().split_whitespace().map(|x| x.parse().ok().unwrap()).collect()
}

fn land_count(h: usize, w: usize) {
    let mut inputs: Vec<Vec<usize>> = vec![];
    for _ in 0..h {
        let p: Vec<usize> = parse_split(read());
        inputs.push(p);
    }
}

fn main() {
    let first_input: Vec<usize> = parse_split(read());
    let mut h = first_input[0];
    let mut w = first_input[1];
    while h != 0 && w != 0 {
        land_count(h, w);
        let next_input: Vec<usize> = parse_split(read());
        h = next_input[0];
        w = next_input[1];
    }
}
