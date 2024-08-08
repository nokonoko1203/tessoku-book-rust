use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut answer = 0;
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            let l = k - i - j;
            if l > 0 && l <= n {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
