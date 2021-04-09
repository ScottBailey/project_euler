/*

https://projecteuler.net

Take the number 192 and multiply it by each of 1, 2, and 3:

192 × 1 = 192
192 × 2 = 384
192 × 3 = 576

By concatenating each product we get the 1 to 9 pandigital,
192384576. We will call 192384576 the concatenated product of 192 and
(1,2,3)

The same can be achieved by starting with 9 and multiplying by 1, 2,
3, 4, and 5, giving the pandigital, 918273645, which is the
concatenated product of 9 and (1,2,3,4,5).

What is the largest 1 to 9 pandigital 9-digit number that can be
formed as the concatenated product of an integer with (1,2, ... , n)
where n > 1?

NOTES:

Look at this:
Lets add a pandigital trait for u64 & u32?
Also, could we add a concatenate trait to u64?
what about pop_left()? pop_right()? push_left()? push_right()?

    trait IsPandigital {
        fn is_pandigital(&self) -> bool;
    }

    impl IsPandigital for u64 {
        fn is_pandigital(&self) -> bool {
            let mut is_found = [false; 9];
            let mut mask = 1;
            for _ in 0..9 {
                let digit = *self / mask % 10;
                if digit == 0 {
                    return false;
                }
                if is_found[digit as usize - 1] {
                    return false;
                }
                is_found[digit as usize - 1] = true;
                mask *= 10;
            }
            true
        }
    }


*/

// n is a string containing ONLY digits 0-9 and is 9 chars long
fn is_pandigital(m : & String) -> bool {
    let mut v = [false; 10];
    let n = m.as_bytes();
    v[ (n[0_usize ]-b'0') as usize ] = true;
    v[ (n[1_usize ]-b'0') as usize ] = true;
    v[ (n[2_usize ]-b'0') as usize ] = true;
    v[ (n[3_usize ]-b'0') as usize ] = true;
    v[ (n[4_usize ]-b'0') as usize ] = true;
    v[ (n[5_usize ]-b'0') as usize ] = true;
    v[ (n[6_usize ]-b'0') as usize ] = true;
    v[ (n[7_usize ]-b'0') as usize ] = true;
    v[ (n[8_usize ]-b'0') as usize ] = true;
    return v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7] && v[8] && v[9];
}


fn solve() -> u64 {
    let mut max = 0;

    for n in 1..10_000_u64 { // max n is a 4 digit number + a 5 digit number, 10,000 results in 2 5 digit numbers
        let mut s = String::with_capacity(20);
        for i in 1_u64.. {
            s += &(n*i).to_string();
            if s.len() == 9 {
                if is_pandigital(&s) {
                    let temp : u64 = s.parse().unwrap();
                    if temp > max {
                        max = temp;
                    }
                    break;
                }
            }
            if s.len() > 10 {
                break;
            }
        }
    }

    max
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    //println!("Elasped time: {} us", elapsed);

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
