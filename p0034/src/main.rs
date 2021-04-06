/*

https://projecteuler.net

145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the
factorial of their digits.

Note: As 1! = 1 and 2! = 2 are not sums they are not included.


NOTES:


*/


fn solve() -> u64 {

    // factorials for digits 0 to 9
    //           0      1  2  3   4    5    6     7      8       9
    let v = vec![1_u64, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let max = 7*v[9];

    let mut rv = 0;
    let mut n = vec![0_u64, 1];
    loop {
        let mut sum = 0;
        let mut val = 0;
        for i in n.iter().rev() {
            val *= 10;
            val += i;
            sum += v[*i as usize];
        }

        if val == max {
            break;
        }
        if val == sum {
            rv += val;
            println!("{} {}, {}", val, sum, rv);
        }
        else {
            //println!("{} {}", val, sum);
        }
        let mut i = 0;
        loop {
            if i < n.len() {
                n[i] += 1;
            }
            else {
                n.push(1);
            }
            if n[i] < 10 {
                break;
            }
            n[i] = 0;
            i += 1;
        }
    }

    rv
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
