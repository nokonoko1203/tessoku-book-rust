use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize ,
    }
    let mut answer = "No";
    for i in a..b + 1 {
        if 100 % i == 0 {
            answer = "Yes";
        }
    }
    println!("{}", answer);
}
