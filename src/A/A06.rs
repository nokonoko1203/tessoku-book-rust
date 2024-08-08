use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q]
    }
    let mut s = Vec::new();
    let mut sum = 0;
    s.push(sum);
    for i in a {
        sum += i;
        s.push(sum);
    }
    for (j, k) in lr {
        println!("{:?}", s[k] - s[j - 1]);
    }
}
