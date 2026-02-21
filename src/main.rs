fn main() {
    println!("Hello, world!");
    println!("-3 to the power of 2 is:");
    println!("{}", power( -3, 2));
}

fn power( base: i64, n:i64 ) -> i64 {
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
    assert_eq!( power( -3, 2 ), 9);
    assert_eq!( power( -3, 3 ), -27);
}
