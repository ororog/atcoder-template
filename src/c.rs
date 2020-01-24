#[allow(dead_code)]
fn is_whitespace(c: &char) -> bool {
    c.is_whitespace()
}

#[allow(dead_code)]
fn is_eol(c: &char) -> bool {
    ['\r', '\n'].contains(c)
}

#[allow(unused_macros)]
macro_rules! input {
    ($var:ident, $($r:tt)*) => {
        input_inner!{$var, $($r)*}
    };

    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        #[allow(unused_mut, unused_variables, bare_trait_objects)]
        let mut next = move |f: &Fn(&char)->bool| -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|f(&c))
                .take_while(|c|!f(&c))
                .collect()
        };
        input_inner!{next, $($r)*};
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

    ($next:expr, [ $t:tt ; eol ]) => {
        $next(&is_eol).split(' ').map(|x| x.parse::<$t>().unwrap()).collect::<Vec<_>>()
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

    ($next:expr, parser) => {
        $next
    };

    ($next:expr, $t:ty) => {
        $next(&is_whitespace).parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

#[allow(dead_code)]
struct Edge {
    from: usize,
    to: usize,
}

#[allow(dead_code)]
type Graph = Vec<Vec<usize>>;
#[allow(dead_code)]
const M: usize = 1_000_000_007;

fn main() {
    input! {}
}
