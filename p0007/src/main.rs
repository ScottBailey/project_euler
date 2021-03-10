// https://projecteuler.net/problem=7

/*

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
see that the 6th prime is 13.

What is the 10,001st prime number?

*/

fn main() {

    let count : usize = 10_001;
    let v = prime_n(count);

    println!("{}", v.last().unwrap());
}

pub fn prime_n(m : usize) -> std::vec::Vec<u64> {
    let mut v = std::vec::Vec::new();
    v.reserve(m);
    if m == 1 {
        return v;
    }
    v.push(2);
    if m == 1 {
        return v;
    }
    v.push(3);

    let mut test : u64 = 5;
    while v.len() < m {
        let sqrt = (test as f64).sqrt() as u64;

        let mut is_prime = true;
        for n in v.iter() {
            if n > &sqrt {
                break;
            }
            if test%n == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            v.push(test);
        }
        test += 2;
    }

    return v;
}
