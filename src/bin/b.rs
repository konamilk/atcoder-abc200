use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        mut n: i64,
        k: usize

    };

    for _ in 0..k{
        if n % 200 == 0{
            n = n / 200
        }
        else {
            n = n * 1000 + 200
        }
    }

    println!("{}", n);
}
