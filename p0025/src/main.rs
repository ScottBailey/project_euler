// https://projecteuler.net/problem=25

/*

The Fibonacci sequence is defined by the recurrence relation:

Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
Hence the first 12 terms will be:

F1 = 1
F2 = 1
F3 = 2
F4 = 3
F5 = 5
F6 = 8
F7 = 13
F8 = 21
F9 = 34
F10 = 55
F11 = 89
F12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to
contain 1000 digits?

*/

use num_bigint::BigUint;


fn solve() -> u64 {


    let mut n1 = BigUint::from(1_u64);
    let mut n2 = BigUint::from(1_u64);
    let mut n3;

    let mut index = 2;

    loop {

        index += 1;
        n3 = &n1 + &n2;
        if n3.to_string().len() >= 1000 {
            //println!("{}\n{}", n3.to_string().len(), n3.to_string());
            return index;
        }

        index += 1;
        n1 = &n2 + &n3;
        if n1.to_string().len() >= 1000 {
            //println!("{}\n{}", n1.to_string().len(), n1.to_string());
            return index;
        }

        index += 1;
        n2 = &n1 + &n3;
        if n2.to_string().len() >= 1000 {
            //println!("{}\n{}", n2.to_string().len(), n2.to_string());
            return index;
        }

        /*
        if index > 8 && index < 13 {
            println!("{} {}", index-1, n2);
        }
         */
    }
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
