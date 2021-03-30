// module sb::math

#[allow(dead_code)]
// this should get renamed
pub fn factors( mut n : u64) -> std::vec::Vec<u64> {
    let primes = prime_to(n);
    let mut rv = std::vec::Vec::<u64>::new();
    while n > 1 {
        for i in &primes {
            if n%i == 0 {
                rv.push(*i);
                n /= i;
            }
        }
    }
    return rv;
}


#[allow(dead_code)]
pub fn sum(v : &std::vec::Vec::<u64>) -> u64 {
    let mut rv = 0;
    for n in v {
        rv = rv + n;
    }
    return rv;
}


#[allow(dead_code)]
pub fn is_prime(n : u64) -> bool {
    let sqrtn = (n as f64).sqrt() as u64;

    for i in 2..sqrtn {
        if n%i == 0 {
            return false;
        }
    }
    return true;
}


#[allow(dead_code)]
pub fn prime_to(m : u64) -> std::vec::Vec<u64> {
    let mut v = std::vec::Vec::new();
    v.reserve((m/5) as usize);
    if m == 1 {
        return v;
    }
    v.push(2);
    if m == 2 {
        return v;
    }
    v.push(3);

    let mut test : u64 = 5;
    while test <= m {
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


#[allow(dead_code)]
pub fn divisors(m : u64) -> std::vec::Vec<u64> {
    let mut rv1 = std::vec::Vec::new();
    let mut rv2 = std::vec::Vec::new();
    rv1.reserve((m/5) as usize);
    rv2.reserve((m/5) as usize);

    if m == 0 {
        return rv1;
    }

    rv1.push(1);
    rv2.push(m);

    let sqrt = (m as f64).sqrt() as u64;
    for n in 2..=sqrt {

        if m%n == 0 {
            rv1.push(n);
            rv2.push(m/n);
        }

    }

    if !rv1.is_empty() && rv1.last() == rv2.last() {
        rv2.pop();
    }

    for n in rv2.iter().rev() {
        rv1.push(*n);
    }

    return rv1;
}


#[allow(dead_code)]
pub fn proper_divisors(m : u64) -> std::vec::Vec<u64> {
    let mut rv = divisors(m);
    rv.pop();
    return rv;
}


#[allow(dead_code)]
pub fn factorial(n : u64) -> u64 {
    let mut rv = 1;
    for n in 2..=n {
        rv *= n;
    }
    return rv;
}
