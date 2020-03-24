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

fn test1() {
    input!{
        n: usize,
        m: usize,
        v: [(usize, usize); m]
    }

    // 各点の辺情報を2次元配列にする
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in v {
        graph[a].push(b);
        graph[b].push(a);
    }

    // que: 探索を行う点の集合
    let mut que: VecDeque<usize> = VecDeque::new();
    // dist: 探索の結果、何手で到達したかを格納する(-1の場合まだ到達していない)
    let mut dist: Vec<i32> = vec![-1; n];
    let already = |x: i32| {x != -1};

    // 始点を格納する
    dist[0] = 0;
    que.push_back(0);
    while !que.is_empty() {
        // 探索を行う点を取り出す
        let node = match que.pop_front() {
            Some(x) => x,
            None => 0
        };

        // 道の先の手順数
        let p = dist[node] + 1;
        // 点に所属している各点について調査する
        // 探索済みであるか...探索済みならば何もしない
        // 探索先に手順数を格納する
        // 次の始点として格納する
        for i in &graph[node] {
            if already(dist[*i]) {continue;}
            dist[*i] = p;
            que.push_back(*i);
        }

    }

    for i in &dist {
        println!("{}" , *i);
    }
}

fn main() {
    test1();
}
