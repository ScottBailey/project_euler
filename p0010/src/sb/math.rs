// module sb::math

#[allow(dead_code)]
pub fn factors<T>( mut n : T) -> std::vec::Vec<u64>
    where T: u64, u32, u16, u8,
{
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
pub fn is_prime<T>(n : T) -> bool
    where T: u64, u32, u16, u8,
{

    let sqrtn = (n as f64).sqrt() as T;

    for i in 2..sqrtn {
        if n%i == 0 {
            return false;
        }
    }
    return true;
}


#[allow(dead_code)]
pub fn prime_to<T>(m : T) -> std::vec::Vec<T>
    where T: u64, u32, u16, u8,
{
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
