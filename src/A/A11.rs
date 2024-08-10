use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut l = 0;
    let mut r = a.len();
    let mut m = (l + r) / 2;
    while x != a[m] {
        if x > a[m] {
            l = m + 1;
        }
        if x < a[m] {
            r = m - 1
        }
        m = (l + r) / 2;
    }
    let pos = a.iter().position(|&b| b == x).unwrap() + 1;
    println!("{}", pos);
}
