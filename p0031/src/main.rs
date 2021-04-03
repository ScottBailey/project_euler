/*

https://projecteuler.net

n the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).

It is possible to make £2 in the following way:

    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?

NOTES:

I decided to expirement with itterative vs recursive solutions with
this problem. I optimized the $2 coin as 1 different way and optimised
the 0.02 & 0.01 coins as 1 + remain/2.

The recursive solution - which is a bit easier to maintian, I think -
solved in about 70 us while the iterative solution took just over 30
us.

solve1() is iterative while solve2() is recursive.

*/

const CV : [u64; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

fn solve1() -> u64 {

    let mut sol = 1; // for the 2 pound coin

    //skipping c0

    for c1 in 0..=200/CV[1] {
        let sum1 = c1*CV[1];
        if sum1 == 200 {
            sol += 1;
            continue;
        }
        for c2 in 0..=(200-sum1)/CV[2] {
            let sum2 = sum1 + c2*CV[2];
            if sum2 == 200 {
                sol += 1;
                continue;
            }
            for c3 in 0..=(200-sum2)/CV[3] {
                let sum3 = sum2 + c3*CV[3];
                if sum3 == 200 {
                    sol += 1;
                    continue;
                }
                for c4 in 0..=(200-sum3)/CV[4] {
                    let sum4 = sum3 + c4*CV[4];
                    if sum4 == 200 {
                        sol += 1;
                        continue;
                    }
                    for c5 in 0..=(200-sum4)/CV[5] {
                        let sum5 = sum4 + c5*CV[5];
                        if sum5 == 200 {
                            sol += 1;
                            continue;
                        }
                        // we are left now figuring out how many ways we can combine 2 & 1 to get the remaining value...
                        sol += 1 + (200-sum5)/CV[6];
                        /*
                        for c6 in 0..=(200-sum5)/CV[6] {
                            let sum6 = sum5 + c6*CV[6];
                            if sum6 == 200 {
                                sol += 1;
                                continue;
                            }
                            // coin 7, the penny is worth 1
                            {
                                sol += 1;
                            }
                        }
                         */
                    }
                }
            }
        }
    }
    sol
}


fn solve2() -> u64 {
    solve_i(1,200) + 1
}

fn solve_i(index : usize, remain : u64) -> u64 {
    if index == 6 {
        return 1 + remain/CV[index];
    }
    let mut rv = 0;
    for i in 0..=remain/CV[index] {
        rv += solve_i(index+1, remain-i*CV[index]);
    }
    rv
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve1();

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
