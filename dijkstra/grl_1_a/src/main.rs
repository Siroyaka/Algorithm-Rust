#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
 
    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
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
 
    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };
 
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
 
    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };
 
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
 
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

struct Edge {
    cost: usize,
    node: usize
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input!{
        v: usize,
        e: usize,
        r: usize,
        points: [(usize, usize, usize); e]
    }
    let mut graph: Vec<Vec<Edge>> = Vec::new();
    for _ in 0..v {
        graph.push(Vec::new());
    }
    for (s, t, c) in points {
        graph[s].push(Edge { cost: c, node: t});
    }
    let mut dist: Vec<_> = (0..v).map(|_| std::usize::MAX).collect();
    let mut on: Vec<_> = (0..v).map(|_| false).collect();
    dist[r] = 0;
    on[r] = true;

    let mut heep = BinaryHeap::new();
    heep.push(State{cost: 0, position: r});
    while let Some(s) = heep.pop() {
        on[s.position] = true;
        if s.cost > dist[s.position] {continue;}

        for item in &graph[s.position] {
            let next = State{cost: s.cost + item.cost, position: item.node};
            if dist[item.node] > next.cost {
                heep.push(next);
                dist[item.node] = next.cost;
            }
        }

    }
    for i in 0..v {
        if on[i] {
            println!("{}", dist[i]);
        } else {
            println!("{}", "INF");
        }
    }
}
