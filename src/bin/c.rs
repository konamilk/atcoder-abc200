use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use itertools_num::structs::Cumsum;
use itertools_num::ItertoolsNum;

fn main() {
//     let source = AutoSource::from("6
// 123 223 123 523 200 2000
// ");
//     let source = AutoSource::from("8
// 199 100 200 400 300 500 600 200
//
// ");
    input!{
        // from source,
        n: usize,
        a: [i64; n]
    };

    let a_mod = a.iter().map(|&x| x % 200).collect::<Vec<i64>>();

    let mut val= [0; 200];

    for e in a_mod{
        val[e as usize] += 1;
    }

    let mut ans = 0i64;

    for i in 0..val.len() {
        ans += val[i] * (val[i] - 1) / 2;
    }

    println!("{}", ans);
}
