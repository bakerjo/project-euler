/* Summation of primes
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

fn main() {
    let mut sum = 0;
    let mut i = 2;
    while i < 2000000 {
        if is_prime(i) {
            sum += i;
        }
        i += 1;
    }
    println!("{}", sum);
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