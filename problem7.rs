/* 10001st prime
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, 
we can see that the 6th prime is 13.

What is the 10001st prime number?
*/

fn main() {
    let mut i = 2;
    let mut prime = 0;
    let mut count = 0;
    while count < 10001 {
        if is_prime(i) {
            prime = i;
            count += 1;
        }
        i += 1;
    }
    println!("{}", prime);
}

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}