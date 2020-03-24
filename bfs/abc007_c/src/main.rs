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
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        ey: usize,
        ex: usize,
        maps: [chars; r]
    }
    let zipcoord = |y: usize, x: usize| y*c+x;
    let unzipcoord = |point: usize| (point/c, point%c);

    let mut graph: Vec<Vec<usize>> = vec![vec![]; r * c];
    for y in 0..r {
        for x in 0..c {
            if maps[y][x] == '#' {continue;}
            let cp = zipcoord(y, x);

            let mut movepoint: Vec<usize> = Vec::new();
            if y != r-1 {movepoint.push(zipcoord(y+1, x))}
            if x != c-1 {movepoint.push(zipcoord(y, x+1))}
            for p in movepoint {
                let (py, px) = unzipcoord(p);
                if maps[py][px] == '#' {continue;}
                graph[p].push(cp);
                graph[cp].push(p);
            }
        }
    }
    let mut dist: Vec<i32> = vec![-1; r*c];
    let mut que: VecDeque<usize> = VecDeque::new();
    let sp = zipcoord(sy-1, sx-1);
    let ep = zipcoord(ey-1, ex-1);
    dist[sp] = 0;
    que.push_back(sp);
    while !que.is_empty() {
        let p = match que.pop_front() {
            Some(x) => x,
            _ => 0
        };
        let moving = dist[p] + 1;
        for point in &graph[p] {
            if dist[*point] != -1 {continue; }
            dist[*point] = moving;
            que.push_back(*point);
        }

    }

    println!("{}", dist[ep]);
}
