/*

https://projecteuler.net

The fraction 49/98 is a curious fraction, as an inexperienced
mathematician in attempting to simplify it may incorrectly believe
that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction,
less than one in value, and containing two digits in the numerator and
denominator.

If the product of these four fractions is given in its lowest common
terms, find the value of the denominator.

NOTES:


*/




fn solve() -> u64 {

    let mut sol_n = 1;
    let mut sol_d = 1;

    // numerator
    for n0 in 1..=9 as u64{
        for n1 in 1..=9 as u64 {
            // denominator
            for d0 in n0..=9 as u64 {
                for d1 in 1..=9 as u64 {
                    if n0 == d0 && n1 == d1 {
                        continue;
                    }

                    let n3;
                    let d3;
                    if n0 == d1 {
                        n3 = n1;
                        d3 = d0;
                    }
                    else if n1 == d0 {
                        n3 = n0;
                        d3 = d1;
                    }
                    else {
                        continue;
                    }
                    let n4 = n0*10 + n1;
                    let d4 = d0*10 + d1;

                    if fraction::GenericFraction::<u64>::new(n4,d4) == fraction::GenericFraction::<u64>::new(n3,d3) {
                        println!("{}/{}", n4,d4);
                        sol_n *= n4;
                        sol_d *= d4;
                    }
                }
            }
        }
    }

    let sol = fraction::GenericFraction::<u64>::new(sol_n, sol_d);
    println!("{}",sol);

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
