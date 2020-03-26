#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        n: usize,
        q: usize,
        edges: [(usize, usize); n-1],
        points: [(usize, i64); q]
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in edges {
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut add_values: Vec<i64> = vec![0; n];
    for (v, p) in points {
        add_values[v-1] += p;
    }
    let mut total_points: Vec<i64> = vec![0; n];
    let mut dist: Vec<bool> = vec![false; n];
    let mut que: VecDeque<usize> = VecDeque::new();
    dist[0] = true;
    que.push_front(0);
    while !que.is_empty() {
        let p = match que.pop_front() {
            Some(x) => x,
            _ => 0
        };
        total_points[p] += add_values[p];
        for v in &graph[p] {
            if dist[*v] {continue;}
            dist[*v] = true;
            total_points[*v] += total_points[p];
            que.push_front(*v);
        }
    }

    let a: Vec<String> = total_points.iter().map(|x| x.to_string()).collect();
    println!("{}", a.join(" "));
}
