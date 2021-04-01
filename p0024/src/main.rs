// https://projecteuler.net/problem=24

/*

A permutation is an ordered arrangement of objects. For example, 3124
is one possible permutation of the digits 1, 2, 3 and 4. If all of the
permutations are listed numerically or alphabetically, we call it
lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2,
3, 4, 5, 6, 7, 8 and 9?

*/


fn solve() -> String {

    use itertools::Itertools; // 0.8.2

    let mut count = 0_u64;
    let v = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for perm in v.iter().permutations(v.len()).unique() {
        count += 1;
        if count == 1_000_000 {
            let mut rv = String::new();
            for c in perm {
                rv.push(*c);
            }
            return rv;
        }
    }
    "failure".to_string()
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
        s.insert_str(0,"0");
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
