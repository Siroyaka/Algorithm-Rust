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
        h: usize,
        w: usize,
        n: usize,
        maps: [chars; h]
    }
    let pinzip = |y: usize, x: usize| y * w + x;

    let mut graph: Vec<Vec<usize>> = vec![vec![]; h * w]; 
    let mut points: Vec<usize> = vec![0; n + 1];
    for y in 0..h {
        for x in 0..w {
            let node = maps[y][x];
            if node == 'X' {continue;}
            let zips = pinzip(y, x);
            if node != '.' {
                let parses = if node == 'S' {0} else {(node as u8 - 48) as usize};
                points[parses] = zips;
            }
            let mut vecters: Vec<usize> = Vec::new();
            if y < h-1 && maps[y+1][x] != 'X' {vecters.push(pinzip(y + 1, x))}
            if x < w-1 && maps[y][x+1] != 'X' {vecters.push(pinzip(y, x + 1))}
            for val in vecters {
                graph[zips].push(val);
                graph[val].push(zips);
            }
        }
    }
    let mut answer = 0;
    for i in 0..n {
        let start = points[i];
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut dist: Vec<i32> = vec![-1; w * h];
        que.push_back(start);
        dist[start] = 0;

        while !que.is_empty() {
            let p = match que.pop_front() {
                Some(x) => x,
                _ => 0
            };
            let next_val = dist[p] + 1;
            for node in &graph[p] {
                if dist[*node] != -1 {continue;}
                dist[*node] = next_val;
                que.push_back(*node);
            }
        }
        answer += dist[points[i + 1]];
    }

    println!("{}", answer);
}
