/*

https://projecteuler.net

Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

   [21] 22 23 24 [25]
    20  [7] 8 [9] 10
    19   6 [1] 2  11
    18  [5] 4 [3] 12
   [17] 16 15 14 [13]

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

*/

fn solve() -> u64 {
    let mut v = std::vec::Vec::< std::vec::Vec::<u64> >::with_capacity(1001);

    for _ in 0..1001 {
        let mut t : std::vec::Vec::<u64> = vec![0,1001];
        v.push(t);
    }

    {
        let mut x = 500;
        let mut y = 500;
        let n = 1;



    }


    let mut rv = 0;
    for i in 0..1001 {
        rv += v[i][i];
    }
    return rv;
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
