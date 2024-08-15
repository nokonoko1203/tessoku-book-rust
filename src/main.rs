use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let mut set: HashSet<(u64, u64)> = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..n {
            set.insert((a[i], a[j]));
        }
    }

    let mut counter = 0;
    for (x, y) in set {
        if x > y {
            if ((x - y) as i64).abs() <= k as i64 {
                counter += 1;
            }
        } else if x < y && ((y - x) as i64).abs() <= k as i64 {
            counter += 1;
        }
    }
    println!("{}", counter);
}
