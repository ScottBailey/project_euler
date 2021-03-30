// module sb::math

#[allow(dead_code)]
pub fn factors<T: Clone + num::Integer>( n : &T) -> std::vec::Vec<T> where {
    let primes = prime_to(n);
    let mut rv = std::vec::Vec::<T>::new();
    while n > num::one() {
        for i in &primes {
            if *(n%*i) == num::zero() {
                rv.push(*i);
                n /= *i;
            }
        }
    }
    return rv;
}


#[allow(dead_code)]
pub fn is_prime<T: Clone + num::Integer>(n : T) -> bool {

    let sqrtn = (n as f64).sqrt() as T;

    let mut i : T = num::one() + num::one();
    while i < sqrtn {
        if n%i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}


#[allow(dead_code)]
pub fn prime_to<T: Clone + num::Integer>(m : T) -> std::vec::Vec<T> {
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
        let sqrt = (test as f64).sqrt() as T;

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
