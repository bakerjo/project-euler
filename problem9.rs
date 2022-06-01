/* Special Pythagorean triplet
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut c;
    while a < 1000 {
        while b < 1000 {
            c = b;
            while c < 1000 {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    println!("{}", a * b * c);
                    return;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
        b = a;
    }
}