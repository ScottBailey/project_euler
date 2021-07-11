/*

https://projecteuler.net

The primes 3, 7, 109, and 673, are quite remarkable. By taking any two
primes and concatenating them in any order the result will always be
prime. For example, taking 7 and 109, both 7109 and 1097 are
prime. The sum of these four primes, 792, represents the lowest sum
for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes
concatenate to produce another prime.

*/

use primes::PrimeSet;


fn concat(a : u64, b : u64) -> u64 {

    let mut d = a;

    let mut c = b;
    while c > 0 {
        c /= 10;
        d *= 10;
    }
    d + b
}


fn solve() -> u64 {

    const MAX : u64 = 50_000;
    let mut rv = MAX;
    let mut prime_set = primes::Sieve::new();

    for i1 in 0..1000 {
        let n1 = prime_set.get(i1);
        if n1 > rv {
            break;
        }

        for i2 in (i1+1).. {
            let n2 = prime_set.get(i2);
            if (n1 + (n2*4)) > rv {
                break;
            }
            if !prime_set.is_prime(concat(n1,n2)) || !prime_set.is_prime(concat(n2,n1)) {
                continue;
            }

            for i3 in (i2+1).. {
                let n3 = prime_set.get(i3);
                if (n1 + n2 + (n3*3)) > rv {
                    break;
                }
                if !prime_set.is_prime(concat(n1,n3)) || !prime_set.is_prime(concat(n3,n1))
                    || !prime_set.is_prime(concat(n2,n3)) || !prime_set.is_prime(concat(n3,n2)) {
                    continue;
                }

                for i4 in (i3+1).. {
                    let n4 = prime_set.get(i4);
                    if (n1 + n2 + n3 + (n4*2)) > rv {
                        break;
                    }
                    if !prime_set.is_prime(concat(n1,n4)) || !prime_set.is_prime(concat(n4,n1))
                        || !prime_set.is_prime(concat(n2,n4)) || !prime_set.is_prime(concat(n4,n2))
                        || !prime_set.is_prime(concat(n3,n4)) || !prime_set.is_prime(concat(n4,n3)) {
                            continue;
                        }

                    for i5 in (i4+1).. {
                        let n5 = prime_set.get(i5);
                        let sum = n1 + n2 + n3 + n4 + n5;
                        if sum > rv {
                            break;
                        }
                        if !prime_set.is_prime(concat(n1,n5)) || !prime_set.is_prime(concat(n5,n1))
                            || !prime_set.is_prime(concat(n2,n5)) || !prime_set.is_prime(concat(n5,n2))
                            || !prime_set.is_prime(concat(n3,n5)) || !prime_set.is_prime(concat(n5,n3))
                            || !prime_set.is_prime(concat(n4,n5)) || !prime_set.is_prime(concat(n5,n4)) {
                            continue;
                        }

                        rv = sum;
                    }
                }
            }
        }
    }

    if rv == MAX {
        panic!("Failed to find an answer");
    }
    if rv != 26033 {
        panic!("Found the wrong answer. Expected 26033, found {}", rv);
    }
    rv
}


fn main() {

    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    let mut remain = elapsed;
    let mut s = String::new();
    if remain == 0 {
        s.insert(0,'0');
    }
    while remain > 0 {
        let temp = remain%1000;
        remain /= 1000;
        if remain > 0 {
            s = format!(",{:03}",temp) + &s;
        }
        else {
            s = format!("{}",temp) + &s;
        }
    }
    println!("Elasped time: {} us", s);
}
