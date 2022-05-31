/* Largest palindrome product

A palindromic number reads the same both ways. 
The largest palindrome made from the product 
of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made 
from the product of two 3-digit numbers.
*/

fn main() {
    let mut largest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) && product > largest {
                largest = product;
            }
        }
    }
    println!("{}", largest);
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let mut rev = String::new();
    for c in s.chars() {
        rev.insert(0, c);
    }
    return s == rev;
}