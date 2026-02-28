use std::env;
use std::str::FromStr;

fn main() {
    println!("Hello, maths powers!");

    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(i64::from_str(&arg).expect("error parsing arg"));
    }

    if numbers.is_empty() {
        eprintln!("Usage: powers NUMBER EXP");
        std::process::exit(1);
    }
    if numbers.len() > 2 {
        eprintln!("Usage: powers NUMBER EXP");
        eprintln!("Error: only two values expected saw {}", numbers.len());
        std::process::exit(1);
    }

    println!("{}", power(numbers[0], numbers[1]));
}

fn power(base: i64, n: i64) -> i64 {
    assert!(base != 0 && n != 0);
    let mut i = 1;
    let mut calc = base;
    while i < n {
        calc *= base;
        i += 1;
    }
    calc
}

#[test]
fn test_power() {
    assert_eq!(power(-3, 2), 9);
    assert_eq!(power(-3, 3), -27);
}
