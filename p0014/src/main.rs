// https://projecteuler.net/problem=14

/*

The following iterative sequence is defined for the set of positive
integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following
sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1)
contains 10 terms. Although it has not been proved yet (Collatz
Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one
million.

*/


fn main() {
    sol1();
    sol2();
    sol3();
}

fn sol1() {
    let start_time = std::time::Instant::now();

    let mut map = std::collections::HashMap::new();
    map.insert(1,1);

    let mut list = std::vec::Vec::<u64>::with_capacity(10_000);
    for n in 2..1_000_000 {
        list.push(n);
        while !map.contains_key(list.last().unwrap()) {
            let last = *list.last().unwrap();
            if last%2 == 0 {
                list.push( last / 2 );
            }
            else {
                list.push( 3 * last + 1 );
            }
        }
        let mut depth = map[&list.pop().unwrap()]; // pop off the found value and find it's depth
        for key in list.iter().rev() {
            depth += 1;
            map.insert(*key,depth);
        }

        list.clear();
    }

    let mut key : u64 = 0;
    let mut max : u64 = 0;

    for (n, count) in &map {
        if *count > max {
            key = *n;
            max = *count;
        }
    }

    println!("\nSolution 1: {} has {} elements.", key, max);

    let elapsed = start_time.elapsed().as_micros();
    println!("Elasped time: {} us", elapsed);

}


fn sol2() {

    let start_time = std::time::Instant::now();

    let mut key : u64 = 0;
    let mut max : u64 = 0;
    for i in 1..1_000_000 {
        let mut n = i;
        let mut depth = 1;
        while n > 1 {
            depth += 1;
            if n%2 == 0 {
                n /= 2;
            }
            else {
                n = n * 3 + 1;
            }
        }
        if depth > max {
            key = i;
            max = depth;
        }
    }


    println!("\nSolution 2: {} has {} elements.", key, max);

    let elapsed = start_time.elapsed().as_micros();
    println!("Elasped time: {} us", elapsed);
}


fn sol3() {
    let start_time = std::time::Instant::now();

    let mut vec = std::vec::Vec::<u64>::new();
    vec.resize(1_000_000, 0);
    vec[1] = 1;

    let mut list = std::vec::Vec::<u64>::with_capacity(10_000);
    for n in 2..1_000_000 {
        list.push(n);
        while *list.last().unwrap() >= vec.len() as u64 || vec[*list.last().unwrap() as usize] == 0 {
            let last = *list.last().unwrap();
            if last%2 == 0 {
                list.push( last / 2 );
            }
            else {
                list.push( 3 * last + 1 );
            }
        }
        let mut depth = vec[list.pop().unwrap() as usize]; // pop off the found value and find it's depth
        for key in list.iter().rev() {
            depth += 1;
            if *key < vec.len() as u64 { // because vec must be memory limited...
                vec[*key as usize] = depth;
            }
        }

        list.clear();
    }

    let mut key : u64 = 0;
    let mut max : u64 = 0;

    for i in 0..1_000_000 {
        if vec[i] > max {
            key = i as u64;
            max = vec[i];
        }
    }

    println!("\nSolution 3: {} has {} elements.", key, max);

    let elapsed = start_time.elapsed().as_micros();
    println!("Elasped time: {} us", elapsed);

}
