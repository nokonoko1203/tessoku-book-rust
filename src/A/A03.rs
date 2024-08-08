use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut answer = "No";
    for i in p {
        for j in q.iter() {
            if i + j == k {
                answer = "Yes";
            }
        }
    }
    println!("{}", answer);
}
