#[allow(unused_macros)]
macro_rules! input_byline {
    ($($r:tt)* ) => {
        let stdin = std::io::stdin();
        let mut q = std::collections::VecDeque::new();
        let mut next = || {
            if q.is_empty() {
                let mut s = String::new();
                stdin.read_line(&mut s);
                for v in s.split_whitespace() {
                    q.push_back(v.to_string());
                }
            }
            q.pop_front().unwrap()
        };
        input_inner!{next, $($r)*}
    };

}

#[allow(unused_macros)]
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
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

trait Printable {
    fn p(&self);
}

impl<T: std::fmt::Display> Printable for T {
    fn p(&self) {
        println!("{}", self);
    }
}

impl<T: std::fmt::Display> Printable for [T] {
    fn p(&self) {
        for i in 0..self.len() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", self[i]);
        }
        println!("");
    }
}

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
type Graph = Vec<Vec<usize>>;

fn main() {
    input! {

    }
}

