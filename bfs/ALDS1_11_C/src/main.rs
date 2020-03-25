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

fn read<T : std::str::FromStr>() -> Vec<T> {
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    a.trim().split_whitespace().map(|x| x.parse().ok().unwrap()).collect()
}

fn main() {
    let aaa: Vec<usize> = read();
    let n = aaa[0];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n {
        let inputs: Vec<usize> = read();
        let v = inputs[0];
        let d = inputs[1];
        for i in 0..d {
            let value = inputs[i + 2];
            graph[v - 1].push(value - 1);
        }
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut dist: Vec<i32> = vec![-1; n];
    que.push_back(0);
    dist[0] = 0;
    while !que.is_empty() {
        let v = match que.pop_front() {
            Some(x) => x,
            _ => 0
        };
        let p = dist[v] + 1;

        for item in &graph[v] {
            if dist[*item] != -1 {continue;}
            dist[*item] = p;
            que.push_back(*item);
        }

    }

    for i in 0..n {
        println!("{} {}", i + 1, dist[i]);
    }
}
