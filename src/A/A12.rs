use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
        a: [u64; n],
    }

    let mut left = 0;
    let mut right = 10_u64.pow(9);

    while left < right {
        let medium_second = (left + right) / 2;

        let sum = calc_sum(&a, medium_second);

        if sum >= k {
            right = medium_second;
        } else {
            left = medium_second + 1;
        }
    }
    println!("{}", (left + right) / 2);
}

fn calc_sum(a: &Vec<u64>, second: u64) -> u64 {
    let mut sum = 0;
    for i in a {
        sum += second / *i;
    }
    sum
}
