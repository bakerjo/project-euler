/* Smallest multiple
2520 is the smallest number that can be divided 
by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly
divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let mut i = 20*3*5*7*11*13*17*19;
    while true {
        let mut divisible = true;
        for j in 11..20 {
            if i % j != 0 {
                divisible = false;
                break;
            }
        }
        if divisible {
            println!("{}", i);
            break;
        }
        i += 20;
    }
}