/*

https://projecteuler.net

n the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).

It is possible to make £2 in the following way:

    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?

*/

fn solve() -> u64 {

    let mut sol = 1; // for the 2 punund coin

    for c1 in 0..=200 {
        let sum1 = c1;
        if sum1 == 200 {
            sol += 1;
            continue;
        }
        for c2 in 0..=200/2 {
            let sum2 = sum1 + c2*2;
            if sum2 == 200 {
                sol += 1;
                continue;
            }
            if sum2 > 200 {
                continue;
            }
            for c3 in 0..=200/5 {
                let sum3 = sum2 + c3*5;
                if sum3 == 200 {
                    sol += 1;
                    continue;
                }
                if sum3 > 200 {
                    continue;
                }
                for c4 in 0..=200/10 {
                    let sum4 = sum3 + c4*10;
                    if sum4 == 200 {
                        sol += 1;
                        continue;
                    }
                    if sum4 > 200 {
                        continue;
                    }
                    for c5 in 0..=200/20 {
                        let sum5 = sum4 + c5*20;
                        if sum5 == 200 {
                            sol += 1;
                            continue;
                        }
                        if sum5 > 200 {
                            continue;
                        }
                        for c6 in 0..=200/50 {
                            let sum6 = sum5 + c6*50;
                            if sum6 == 200 {
                                sol += 1;
                                continue;
                            }
                            if sum6 > 200 {
                                continue;
                            }
                            for c7 in 0..=200/100 {
                                let sum7 = sum6 + c7*100;
                                if sum7 == 200 {
                                    sol += 1;
                                    continue;
                                }
                                if sum7 > 200 {
                                    continue;
                                }
                                /*
                                // optimize this out to adding one at the beginning
                                for c8 in 0..=200/200 {
                                    let sum8 = sum7 + c8*200;
                                    if sum8 == 200 {
                                        sol += 1;
                                        continue;
                                    }
                                    if sum8 > 200 {
                                        continue;
                                    }
                                }
                                */
                            }
                        }
                    }
                }
            }
        }
    }

    sol
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
