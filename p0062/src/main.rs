/*

https://projecteuler.net

The cube, 41063625 (345^3), can be permuted to produce two other
cubes: 56623104 (384^3) and 66430125 (405^3). In fact, 41063625 is the
smallest cube which has exactly three permutations of its digits which
are also cube.

Find the smallest cube for which exactly five permutations of its
digits are cube.

*/

use std::collections::HashMap;

// Take any number n and change it to a permutation with largest
// digits first. For example: 7248 becomes 8742.
fn to_max_permutation(mut n : u128) -> u128 {

    // Create a vector of digits of the digits in n.
    let mut v = Vec::<u128>::with_capacity(20);
    while n > 0 {
        v.push( n%10 );
        n /= 10;
    }
    // Reverse sort the digits.
    v.sort_by(|a,b|b.cmp(a));
    // Convert the digits back to a return value
    let mut rv = 0;
    for n in v {
        rv *= 10;
        rv += n;
    }

    rv
}


fn solve() -> u128 {
    // Map of permutations to lowest and count of cubes.
    let mut map = HashMap::<u128,(u128,usize)>::new();

    // For all numbers!
    for i in 0_u128.. {
        // Find the cube and its permutation.
        let n = i*i*i;
        let p = to_max_permutation(n);
        // Store the initial value or increment the count...
        match map.get_mut(&p) {
            None => {map.insert(p, (n,1));},
            Some(mut t) => {
                t.1 += 1;
                // ...testing to see if we found the soulution.
                if t.1 == 5 {
                    return n;
                }
            },
        };
    }

    panic!("There is no possible way to get here.");
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




#[test]
fn test_px() {

    assert!( p(3,1) == 1 );
    assert!( p(3,2) == 3 );
    assert!( p(3,3) == 6 );

    assert!( p(4,1) == 1 );
    assert!( p(4,2) == 4 );
    assert!( p(4,3) == 9 );

    assert!( p(5,1) == 1 );
    assert!( p(5,2) == 5 );
    assert!( p(5,3) == 12 );

    assert!( p(6,1) == 1 );
    assert!( p(6,2) == 6 );
    assert!( p(6,3) == 15 );

    assert!( p(7,1) == 1 );
    assert!( p(7,2) == 7 );
    assert!( p(7,3) == 18 );

    assert!( p(8,1) == 1 );
    assert!( p(8,2) == 8 );
    assert!( p(8,3) == 21 );


}
