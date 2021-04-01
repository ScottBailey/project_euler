// https://projecteuler.net/problem=22

/*

Using names.txt (right click and 'Save Link/Target As...'), a 46K text
file containing over five-thousand first names, begin by sorting it
into alphabetical order. Then working out the alphabetical value for
each name, multiply this value by its alphabetical position in the
list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN,
which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the
list. So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?

*/

use std::io::prelude::*;  // needed for .bytes()
//use std::vec::Vec;
//use std::io::BufRead;


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);
    println!("Elasped time: {} us", elapsed);
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

    let mut v = std::vec::Vec::<String>::with_capacity(46*1024/4);

    let mut s = String::new();

    let reader = std::io::BufReader::new(infile);
    for byte in reader.bytes() {
        let c = byte.unwrap() as char;
        if c == ',' {
            // do nothing for this
        }
        else if c == '"' {
            if !s.is_empty() {
                v.push(s);
                s = String::new();
            }
        }
        else {
            s.push(c);
        }
    }

    v.sort();

    let mut rv = 0_u64;

    for (i,s) in v.iter().enumerate() {
        let mut val = 0_u64;
        for c in s.bytes() {
            val += (c - b'@') as u64;
        }
        val *= 1 + i as u64;
        //if i == 937 {
        //   println!("{} {}",s,val);
        //}
        rv += val;
    }


    return rv;
}
