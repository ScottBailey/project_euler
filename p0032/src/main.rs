/*

https://projecteuler.net

We shall say that an n-digit number is pandigital if it makes use of
all the digits 1 to n exactly once; for example, the 5-digit number,
15234, is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254,
containing multiplicand, multiplier, and product is 1 through 9
pandigital.

Find the sum of all products whose multiplicand/multiplier/product
identity can be written as a 1 through 9 pandigital.

HINT: Some products can be obtained in more than one way so be sure to
only include it once in your sum.

NOTES:

I had to read this problem multiple times to get the correct answer.

Find all the single digit use numbers first using a recusive
function. This takes about 150ms alone to find 9 digits worth. I'm
sure that time could be halved with an iterative function, but it's
messy enough as it is. Regardless, for this exercise, we limit it to 4
digits anyway, cutting time into som much smaller fraction.

Then we check each number (c) in the list list by dividing it by all
the evenly divisible list numbers less than it's square
root (a).  We take that set of values (a x b = c) and test if a, b, c
contain all the digits 1..=n to make a

*/

fn make_singledigits_i(chosen : Vec::<u64>, available : Vec::<u64>, rv : & mut Vec::<u64>) { // this could use a better name
    // add any current semi-pandigitals in in
    if !chosen.is_empty() {  // this is here for the first call, where chosen is empty, we could optimize this out better...
        let mut temp = 0_u64;
        for n in &chosen {
            temp *= 10;
            temp += *n;
        }
        rv.push(temp);
    }

    // we can limit our chosen values to 5 digits for this exercise
    if chosen.len() == 5 {
        return;
    }

    for n in &available {

        let mut new_chosen = chosen.clone();
        new_chosen.push(*n);

        let mut new_avail = available.clone();
        new_avail.retain(|&x| x != *n);

        make_singledigits_i(new_chosen, new_avail, rv);
    }
}


fn make_singledigits() -> Vec::<u64> {
    let chosen = Vec::<u64>::new();
    let available : Vec::<u64> = vec![1,2,3,4,5,6,7,8,9];
    let mut rv = Vec::<u64>::with_capacity(1_100_000); // testing says this is an adequately sized number
    make_singledigits_i(chosen, available, & mut rv);
    rv.sort();
    rv
}


fn count_digits(v : &mut Vec::<u64>, n : u64) {
    // increment the count (v[digit]) for each digit of n
    let mut temp = n;
    while temp > 0 {
        v[ (temp%10) as usize] += 1;
        temp /= 10;
    }
}


fn is_pandigital(a : u64, b : u64, c : u64) -> bool {
    // create a vector of 0 counts
    let mut v = Vec::<u64>::new();
    v.resize(10,0);

    // increment each count for digits in a, b, and c
    count_digits(& mut v, a);
    count_digits(& mut v, b);
    count_digits(& mut v, c);

    // sanity check that we sent the correct size numbers
    let sum : u64 = v.iter().sum();

    //if sum > 9 { // this test finds all the pandigitals 1 to n
    if sum != 9 { // this test finds all the 9 digit pandigitals
        return false;
    }

    // test all numbers 1 to n accunted for once and only once
    for i in 1..=sum as usize{
        if v[i] != 1 {
            return false;
        }
    }

    true
}


fn solve() -> u64 {
    let mut sol = 0;
    let v = make_singledigits();

    for c in &v {
        // don't test small number
        if *c < 1000 {
            continue;
        }
        // large numbers aren't actually in the list, so no need to test for them
        let max = (*c as f64).sqrt() as u64;
        for a in &v {
            if *a > max {
                break;
            }
            if *c % *a == 0 {
                let b = *c / *a;
                if is_pandigital(*a,b,*c) {
                    //println!("{} x {} = {}", a, b, c);
                    sol += *c;
                    break; // only add the product, c, once
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
