/*

https://projecteuler.net

Surprisingly there are only three numbers that can be written as the
sum of fourth powers of their digits:

  1634 = 1^4 + 6^4 + 3^4 + 4^4
  8208 = 8^4 + 2^4 + 0^4 + 8^4
  9474 = 9^4 + 4^4 + 7^4 + 4^4

As 1 = 1^4 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of
fifth powers of their digits.


*/

const P : u32 = 5;

//const POWS : Vec<u64> = vec![0_u64, 1_u64.pow(P), 2_u64.pow(P), 3_u64.pow(P), 4_u64.pow(P), 5_u64.pow(P),
//6_u64.pow(P), 7_u64.pow(P), 8_u64.pow(P), 9_u64.pow(P)];

fn solve() -> u64 {
    let mut v = std::vec::Vec::<u64>::with_capacity(99*99);

    let pows = vec![0_u64.pow(P), 1_u64.pow(P), 2_u64.pow(P), 3_u64.pow(P), 4_u64.pow(P), 5_u64.pow(P),
                    6_u64.pow(P), 7_u64.pow(P), 8_u64.pow(P), 9_u64.pow(P)];

    for a in 0..10_u64 {
        let sum1 = pows[a as usize];
        let num1 = a;
        for b in 0..10_u64 {
            let sum2 = sum1 + pows[b as usize];
            let num2 = num1 + 10 * b;
            if b > 0 && num2 == sum2 {
                v.push(num2);
            }
            for c in 0..10_u64 {
                let sum3 = sum2 + pows[c as usize];
                let num3 = num2 + 100 * c;
                if c > 0 && num3 == sum3 {
                    v.push(num3);
                }
                for d in 0..10_u64 {
                    let sum4 = sum3 + pows[d as usize];
                    let num4 = num3 + 1000 * d;
                    if d > 0 && num4 == sum4 {
                        v.push(num4);
                    }
                    for e in 0..10_u64 {
                        let sum5 = sum4 + pows[e as usize];
                        let num5 = num4 + 10000 * e;
                        if e > 0 && num5 == sum5 {
                            v.push(num5);
                        }
                        for f in 1..10_u64 {
                            let sum6 = sum5 + pows[f as usize];
                            let num6 = num5 + 100000 * f;
                            if num6 == sum6 {
                                v.push(num6);
                            }
                        }
                    }
                }
            }
        }
    }

    //println!("{:?}", &v);

    return v.iter().sum();
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
