/*

https://projecteuler.net

If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.

Not all numbers produce palindromes so quickly. For example,

   349 + 943 = 1292,
   1292 + 2921 = 4213
   4213 + 3124 = 7337

That is, 349 took three iterations to arrive at a palindrome.

Although no one has proved it yet, it is thought that some numbers,
like 196, never produce a palindrome. A number that never forms a
palindrome through the reverse and add process is called a Lychrel
number. Due to the theoretical nature of these numbers, and for the
purpose of this problem, we shall assume that a number is Lychrel
until proven otherwise. In addition you are given that for every
number below ten-thousand, it wi ll either (i) become a palindrome in
less than fifty iterations, or, (ii) no one, with all the computing
power that exists, has managed so far to map it to a palindrome. In
fact, 10677 is the first number to be shown to require over fifty
iterations before producing a palindrome: 4668731596684224866951378664
(53 iterations, 28-digits).

Surprisingly, there are palindromic numbers that are themselves
Lychrel numbers; the first example is 4994.

How many Lychrel numbers are there below ten-thousand?

NOTE: Wording was modified slightly on 24 April 2007 to emphasise the
theoretical nature of Lychrel numbers.

NOTES:

  u64 overflows, use u128

  We can use a vector of bools to indicate which numbers are or become
  palindromes. This results in a savings of 35% or so.
*/




fn reverse(mut n : u128) -> u128 {

    let mut rv = 0;
    while n > 0 {
        rv *= 10;
        rv += n%10;
        n /= 10;
    }

    rv
}


fn solve() -> u64 {

    // Use a vector to optimize the search
    let mut palindromes = Vec::<bool>::new();
    palindromes.resize(20_0001,false);

    let mut found = 0;

    for i in (1..=10_000_usize).rev() {

        let mut n = i as u128;

        // One iteration is ALWAYS required since some Lychrel number
        // are palindromes. Make sure to test and test the initial
        // number's palindrome value.
        let r = reverse(n);
        if n == r {
            palindromes[i] = true;
        }
        n += r;

        // Do 50 tests...
        for _ in 0..50 {

            // If it's less than 10_000 we've already checked it.
            if n <= 10_000 {
                if palindromes[n as usize] {
                    palindromes[i] = true;
                    found += 1;
                }
                break;
            }

            // Just reverse the number to see if it's a palindrome.
            let r = reverse(n);
            if n == r {
                palindromes[i] = true;
                found += 1;
                break;
            }
            n += r;
        }

    }

    let rv = 10_000 - found;
    if rv != 249 {
        panic!("solution {} mismatch", rv);
    }

    rv
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
