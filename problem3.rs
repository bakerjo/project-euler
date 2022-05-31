/* Largest prime factor
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
 */

 fn main() {
    let n = 600851475143;
    let mut prime = 2;
    for i in 2..(square_root(n)) {
        if n % i == 0 {
            if is_prime(i) {
                prime = i;
            }
        }

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
 
 fn square_root(n: u64) -> u64 {
    let mut i = 1;
    while i * i < n {
        i += 1;
    }
    return i;
 }