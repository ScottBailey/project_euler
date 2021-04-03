/*

https://projecteuler.net

n the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

  1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).

It is possible to make £2 in the following way:

  1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?

*/

fn solve() -> u64 {

    0
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
