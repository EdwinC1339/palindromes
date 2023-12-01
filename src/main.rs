use rayon::prelude::*;
fn main() {
    assert!(is_palindrome(404));
    assert!(!is_palindrome(405));
    assert!(is_palindrome(4004));
    assert!(!is_palindrome(4005));

    println!("Palindrome numbers from 1 to 10^n");
    println!("n         manual    smart");
    for n in 1..=9 {
        let manual = manual_count(n);
        let smart = smart_way(n);
        println!("{n:<10}{manual:<10}{smart:<10}")
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut digits = to_digits(n).into_iter();
    while let (Some(front), Some(back)) = (digits.next(), digits.next_back()) {
        if front != back {
            return false;
        }
    }
    true
}

fn ceil_div(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

fn to_digits(v: u64) -> Vec<u8> {
    v.to_string()
        .as_str()
        .bytes()
        .map(|b| (b - b'0') as u8)
        .collect()
}

fn manual_count(n: i32) -> i32 {
    let range = 1u64..=10u64.pow(u32::try_from(n).expect("n is a nonnegative i32"));
    range
        .into_par_iter()
        .filter(|n: &u64| -> bool { is_palindrome(*n) })
        .count()
        .try_into()
        .unwrap()
}

fn smart_way(n: i32) -> i32 {
    let ans: u64 = (1..=n)
        .into_iter()
        .map(|i: i32| -> u64 { 9 * 10u64.pow((ceil_div(i as u64, 2) - 1) as u32) })
        .sum();

    ans.try_into().unwrap()
}
