use std::env;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {                                                                              // Thsi algorithmus does % until it is no longer possible doing so taking always the biggest number to the left, this ensures it is the gratest, and on the next iteration if the result from the x%y was 0 BOOM - GCD magic done!
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]                                                                                         // Test function just to make sure everyting works. "Cargo test" will reveal you the truth
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {

        let mut numbers = Vec::new();                                                           // Creating a vector to store the command line args (a.k.a. arguments) into
        for arg in env::args().skip(1) {                                                        // skip(1) skipes the first element, the first element is always the name of the program itself
            numbers.push(u64::from_str(&arg).expect("error parsing argument"));                 // pushing all the string converted into u64 from the command line into the array
        }
        if numbers.len() == 0 {
            eprintln!("Usage: gcd NUMBER ...");
            std::process::exit(1);
        }
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);                                                                     // Since the greatest common divisor is ... common ... we can do it taking d as reference point, and changing the *m to the next number until done
        }
        println!("The greatest common divisor of {:?} is {}", numbers, d);
    
}
