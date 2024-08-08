use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    }
    let mut day: Vec<usize> = vec![0; d];
    for (l, r) in lr {
        let ume = (l..r + 1).collect::<Vec<_>>();
        for i in ume {
            day[i - 1] += 1
        }
    }
    for d in day {
        println!("{}", d);
    }
}
