/*

https://projecteuler.net

The decimal number, 585 = 10010010012 (binary), is palindromic in both
bases.

Find the sum of all numbers, less than one million, which are
palindromic in base 10 and base 2.

(Please note that the palindromic number, in either base, may not
include leading zeros.)

NOTES:

*/

// return vector of digits least significant digit first
fn digits(n : u64) -> Vec::<u64> {
    let mut rv = Vec::<u64>::new();
    let mut temp = n;
    while temp > 0 {
        rv.push(temp%10);
        temp /= 10;
    }
    if rv.is_empty() {
        rv.push(0);
    }
    rv
}


// return vector of bits least significant bit first
fn bits(n : u64) -> Vec::<u64> {
    let mut rv = Vec::<u64>::new();
    let mut temp = n;
    while temp > 0 {
        rv.push(temp&1);
        temp >>= 1;
    }
    if rv.is_empty() {
        rv.push(0);
    }
    rv
}


fn is_palindrome(v : &Vec::<u64>) -> bool {

    let mut left = 0;
    let mut right = v.len()-1;
    while left < right {
        if v[left] != v[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}


fn solve() -> u64 {

    let mut rv = 0;

    for n in 1..1_000_000 {
        // should this have been done as strings???
        if is_palindrome(&digits(n)) && is_palindrome(&bits(n)) {
            rv += n;
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
