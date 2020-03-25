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
        maps: [chars; h]
    }

    let p_zip = |y: usize, x: usize| y*w+x;
    let mut s_node = 0;
    let mut e_node = 0;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; h * w];
    for y in 0..h {
        for x in 0..w {
            if maps[y][x] == '#' {continue;}
            let p = p_zip(y, x);
            if maps[y][x] == 's' {s_node = p;}
            if maps[y][x] == 'g' {e_node = p;}

            let mut moving: Vec<usize> = Vec::new();
            if y < h-1 && maps[y+1][x] != '#' {moving.push(p_zip(y+1, x));}
            if x < w-1 && maps[y][x+1] != '#' {moving.push(p_zip(y, x+1));}
            for i in moving {
                graph[p].push(i);
                graph[i].push(p);
            }
        }
    }

    let mut dist = vec![false; h * w];
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut answer = false;
    dist[s_node] = true;
    que.push_front(s_node);
    while !que.is_empty() {
        let p = match que.pop_front() {
            Some(x) => x,
            _ => 0
        };
        answer = e_node == p;
        if answer {break;}
        for i in &graph[p] {
            if dist[*i] {continue;}
            dist[*i] = true;
            que.push_front(*i);
        }
    }

    println!("{}", if answer {"Yes"} else {"No"});
}
