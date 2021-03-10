// https://projecteuler.net/problem=9

/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which
a + b + c = 1000.  Find the product abc.

*/


fn main() {
    // smaalest c is 335 because a < b < c : 332+333+335=1000
    for c in 335..1000 {
        for b in ((1000-c)/2)..c-1 {
            let a = 1000-c-b;
            if a >= b || a+b+c != 1000{
                continue;
            }
            if a*a + b*b == c*c {
                println!("{}^2 + {}^2 = {}^2", a, b, c);
                return;
            }
        }
    }

    println!("no solution");
}
