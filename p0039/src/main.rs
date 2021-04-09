/*

https://projecteuler.net

If p is the perimeter of a right angle triangle with integral length
sides, {a,b,c}, there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?


NOTES:

*/

fn count_solutions(p : u64) -> usize {
    let mut rv = 0_usize;


    for a in 1..p/3 {
        let a2 = a*a;

        for b in a..(p-a/2) {
            let b2 = b*b;

            let c = p - a - b;
            let c2 = c*c;

            let a2b2 = a2+b2;
            if a2b2 == c2 {
                rv += 1;
                continue;
            }
            if a2b2 > c2 {
                break;
            }
        }
    }

    rv
}
#[test]
fn test_count() {
    assert!(count_solutions(120) == 3);
}

#[allow(dead_code)]
fn solve1() -> u64 {
    let mut rv = 0;
    let mut max = 0;
    for n in 1..=1000 {
        let count = count_solutions(n);
        if count > max {
            max = count;
            rv = n;
        }
    }

    rv
}


fn solve2() -> u64 {
    let mut perimeter = [0_u64; 1001];
    for a in 1..333 {
        let a2 = a*a;
        for b in a..666 {
            let a2b2 = a2 + b*b;
            let c = (a2b2 as f64).sqrt() as u64;
            let abc = a+b+c;
            if abc > 1000 || c < b {
                break;
            }
            if a2b2 == c*c {
                perimeter[abc as usize] += 1;
                continue;
            }
        }
    }

    let mut rv = 0;
    let mut max = 0;
    for (i, j) in perimeter.iter().enumerate() {
        if *j > max {
            max = *j;
            rv = i;
        }
    }

    rv as u64
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve2();

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
