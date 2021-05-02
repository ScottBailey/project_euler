/*

https://projecteuler.net

Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2. The
first ten pentagonal numbers are:

1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...

It can be seen that P4 + P7 = 22 + 70 = 92 = P8. However, their
difference, 70 − 22 = 48, is not pentagonal.

Find the pair of pentagonal numbers, Pj and Pk, for which their sum
and difference are pentagonal and D = |Pk − Pj| is minimised; what is
the value of D?

NOTES:

*/


fn pentagonal_to(v : &mut Vec::<u64>, val : u64) {
    while *v.last().unwrap() < val {
        pentagonal_next(v);
    }
}


fn pentagonal_next(v : &mut Vec::<u64>) {
    let n = v.len() as u64 + 1;
    let temp = (3*n-1)*n/2;
    v.push(temp);
}


fn solve() -> u64 {

    let mut v = Vec::<u64>::new();

    pentagonal_next(&mut v);
    pentagonal_next(&mut v);

    let mut n = 0_usize;
    loop {
        n += 1;
        if v.len() <= n {
            pentagonal_next(&mut v);
        }
        let last = v[n];
        for i in (0..v.len()-1).rev() {
            let delta = last - v[i];
            if let Err(_) = v.binary_search(&delta) {
                continue;
            }
            let sum = last + v[i];
            pentagonal_to(&mut v, sum);
            if let Err(_) = v.binary_search(&sum) {
                continue;
            }
            //println!("Found! {} {}", i, n);
            return delta;
        }
    }
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
