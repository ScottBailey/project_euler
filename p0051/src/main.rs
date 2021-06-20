/*

https://projecteuler.net

By replacing the 1st digit of the 2-digit number *3, it turns out that
six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all
prime.

By replacing the 3rd and 4th digits of 56**3 with the same digit, this
5-digit number is the first example having seven primes among the ten
generated numbers, yielding the family: 56003, 56113, 56333, 56443,
56663, 56773, and 56993. Consequently 56003, being the first member of
this family, is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number (not
necessarily adjacent digits) with the same digit, is part of an eight
prime value family.

NOTES:


*/


trait ToVec {
    fn to_vec(&self) -> Vec::<u64>;
}
impl ToVec for u64 {
    fn to_vec(&self) -> Vec::<u64> {
        let mut v = Vec::<u64>::new();
        v.reserve(6);
        let mut n = *self;
        while n > 0 {
            v.push( n%10 );
            n /= 10;
        }
        v
    }
}

trait ToU64 {
    fn to_u64(&self) -> u64;
}
impl ToU64 for Vec::<u64> {
    fn to_u64(&self) -> u64 {
        let mut rv = 0;
        for n in self.iter().rev() {
            rv *= 10;
            rv += n;
        }
        rv
    }
}

#[test]
fn test_vec_num_stuff() {

    let v = 234_u64.to_vec();
    assert!(v.len() == 3);
    assert!(v[0] == 4);
    assert!(v[1] == 3);
    assert!(v[2] == 2);

    let n = v.to_u64();
    assert!(n == 234);
}


fn solve() -> u64 {

    const MAX : u64 =  999_999;
    const MIN : u64 =  100_000;
    let mut primes = sb::math::prime_to(MAX); // get all the primes to MAX

    // remove primes less than MIN
    if let Err(loc) = primes.binary_search(&MIN) {
        primes.drain(..loc);
    }
    else {
        panic!("Unexpectedly, MIN ({}) is a prime!",MIN);
    }

    // for every prime...
    for num in &primes {

        // convert the digits into a vector
        let digits = num.to_vec();
        let mut crit = 99; // storage for the critical digit

        // count the instances of a given number occuring
        let mut v = [0_u64; 10];
        for n in &digits {
            v[*n as usize] += 1;
        }
        // store the digit
        for (i, count) in v.iter().enumerate() {
            if count >= &3 {
                crit = i as u64;
                break;
            }
        }
        // if we didn't find the digit, go on to the next prime
        if crit > 9 {
            continue;
        }

        let debug = *num == 56003;

        let mut fails = 0;
        let mut found = 1; // num is already found...
        for i in 0..10_u64 {
            if i == crit {
                // already found this one...
                continue;
            }
            let mut val = 0;
            for j in digits.iter().rev() {
                val *= 10;
                if *j == crit {
                    val += i;
                }
                else {
                    val += *j;
                }
            }

            if let Ok(_) = primes.binary_search(&val) {
                found += 1;
                if debug { println!("{} - {}",&num, val); }
            }
            else {
                fails += 1;
            }
            if found == 8 {
                return *num;
            }
            if fails == 3 {
                break;
            }
        }
    }


    panic!("No solution found!");
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


#[test]
fn test_bcd() {
    let mut n;

    n = sb::math::to_bcd(1);
    assert!(n == 0x01);
    n = sb::math::from_bcd(n);
    assert!(n == 1);

    n = sb::math::to_bcd(90);
    assert!(n == 0x09_00);
    n = sb::math::from_bcd(n);
    assert!(n == 90);

    n = sb::math::to_bcd(100);
    assert!(n == 0x01_00_00);
    n = sb::math::from_bcd(n);
    assert!(n == 100);

}
