use std::io;
use std::process::exit;

fn is_divisible_by_7(n: i64) {
    if n == 7 {
        println!("The number entered is 7 which is divisible by 7");
        exit(0);
    }
    
    let mut a: i64 = n/10;
    let mut b: i64= n%10;
    let mut c: i64 = a-2*b;

    loop {

        if c == 0 || c == 7 || c == -7 {

            println!("The number {} is divisible by 7", n);
            break;
        }
        else if c < 10 && (c != 0 || c !=7 || c!=-7)  {

            println!("The number {} is not divisible by 7", n);
            break;
        }
        else {
            a = c/10;
            b = c%10;
            c = a-(2*b);
        }
    }
}

fn main() {
    let mut n = String::new();
    println!("Please enter the number:to be tested for divisibility by 7");
    io::stdin().read_line(&mut n).expect("Number not entered");
    let n: i64 = n.trim().parse().expect("Please type a number");
    is_divisible_by_7(n);
}
