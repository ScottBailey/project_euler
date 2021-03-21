// https://projecteuler.net/problem=15

/*

215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 21000?

*/


fn main() {

    let start_time = std::time::Instant::now();

    let mut v = std::vec::Vec::<u32>::with_capacity(1000);
    v.push(1);

    for _i in 0..1000 {
        let mut carry = 0;
        for n in &mut v {
            *n *= 2;
            *n += carry;
            carry = *n / 10;
            *n %= 10;
        }
        if carry > 0 {
            v.push(carry);
        }
    }

    let mut sol = 0;
    for n in v {
        sol += n;
    }
    println!("\nSolution: {}",sol);

    println!("\nElasped time: {} us", start_time.elapsed().as_micros());
}
