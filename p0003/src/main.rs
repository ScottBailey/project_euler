// https://projecteuler.net/problem=3

/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

*/


fn main() {

    // problem source number
    let target : u64 = 600851475143;
    let f = factors(target);
    //println!("factor count: {}", f.len());

    // test all the factors in reverse order until a prime is found
    let mut sol = 0;
    for n in f.iter().rev() {
        if is_prime(*n) {
            sol = *n;
            break;
        }
    }

    println!("The largest prime factor of {} is {}", target, sol);
}


fn is_prime(n : u64) -> bool {
    let sqrtn = (n as f64).sqrt() as u64;

    for i in 2..sqrtn {
        if n%i == 0 {
            return false;
        }
    }
    return true;
}


fn factors(m : u64) -> std::vec::Vec<u64> {
    let mut v1 = std::vec::Vec::new();
    let mut v2 = std::vec::Vec::new();

    let sqrt = (m as f64).sqrt() as u64;
    for n in 2..sqrt {
        if m%n == 0 {
            v1.push(n);
            v2.push(m/n);
        }
    }

    v1.reserve(2*v1.len());
    for n in v2.iter().rev() {
        v1.push(*n);
    }

    return v1;
}
