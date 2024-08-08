use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut answer = Vec::new();
    let keta = 10;
    for i in (0..keta).rev() {
        let syo = n / 2_usize.pow(i);
        let amari = syo % 2;
        answer.push(amari);
    }
    let a_str = answer
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", a_str);
}
