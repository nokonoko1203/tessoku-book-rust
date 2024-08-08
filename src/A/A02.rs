use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize ,
        a: [usize;n],
    }
    let mut answer = "No";
    for i in a {
        if i == x {
            answer = "Yes";
        }
    }
    println!("{}", answer);
}
