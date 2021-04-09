/*

https://projecteuler.net

The nth term of the sequence of triangle numbers is given by,
tn = ½n(n+1); so the first ten triangle numbers are:

     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

By converting each letter in a word to a number corresponding to its
alphabetical position and adding these values we form a word
value. For example, the word value for SKY is 19 + 11 + 25 = 55 =
t10. If the word value is a triangle number then we shall call the
word a triangle word.

Using words.txt (right click and 'Save Link/Target As...'), a 16K text
file containing nearly two-thousand common English words, how many are
triangle words?

NOTES:

*/

fn triangle_number_seq() -> Vec::<u64> {
    const MAX : usize = 30;
    let mut rv = Vec::<u64>::with_capacity(MAX);
    for n in 1..=MAX {
        rv.push( ((n*n + n) / 2) as u64 );
    }
    rv
}

#[test]
fn test_tns() {
    assert!(triangle_number_seq()[5] == 21);
}


fn solve() -> u64 {
    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("needs a single argument");
    }

    let path = std::path::Path::new(&args[1]);
    // Open the path in read-only mode, returns `io::Result<File>`
    let infile = match std::fs::File::open(path) {
	Err(why) => panic!("couldn't open {}: {}", path.display(), why),
	Ok(infile) => infile,
    };

    let mut words = std::vec::Vec::<String>::with_capacity(46*1024/4);

    let mut s = String::new();

    let reader = std::io::BufReader::new(infile);

    use std::io::prelude::*;  // needed for .bytes()
    for byte in reader.bytes() {
        let c = byte.unwrap() as char;
        if c == ',' {
            // do nothing for this
        }
        else if c == '"' {
            if !s.is_empty() {
                words.push(s);
                s = String::new();
            }
        }
        else {
            s.push(c);
        }
    }

    let tns = triangle_number_seq();

    let mut rv = 0;

    for w in words {
        let mut s = 0;
        for c in w.as_bytes() {
            s += (c - b'@') as u64;
        }
        if let Ok(_) = tns.binary_search(&s) {
            rv += 1;
        }
    }

    return rv;
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
