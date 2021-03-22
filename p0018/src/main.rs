// https://projecteuler.net/problem=15

/*


By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

   3
  7 4
 2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

              75
             95 64
            17 47 82
           18 35 87 10
          20 04 82 47 65
         19 01 23 75 03 34
        88 02 77 73 07 63 67
       99 65 04 28 06 16 70 92
      41 41 26 56 83 40 80 70 33
     41 48 72 33 47 32 37 16 94 29
    53 71 44 65 25 43 91 52 97 51 14
   70 11 33 28 77 73 17 78 39 68 17 57
  91 71 52 38 17 14 91 43 58 50 27 29 48
 63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

*/


use std::vec::Vec;
use std::io::BufRead;


fn main() {
    let start_time = std::time::Instant::now();


    let args: std::vec::Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("needs a single argument");
    }

    let path = std::path::Path::new(&args[1]);
    println!("Path: {}", path.display());



    // https://doc.rust-lang.org/std/io/index.html

    // Open the path in read-only mode, returns `io::Result<File>`
    let infile = match std::fs::File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(infile) => infile,
    };

    let mut triangle = Vec::<Vec::<u32>>::with_capacity(20);

    {
        //let mut buffer = [0;1];
        let reader = std::io::BufReader::new(infile);
        for line in reader.lines() {  // I think there exists a way to use let here...
            match line {
                Err(why) => panic!("probblem reading data: {}", why),
                Ok(line) => {
                    if line.is_empty() {
                        panic!("probblem reading data: empty line: {}", triangle.len());
                    }
                    let bytes = line.as_bytes();

                    let mut vec = Vec::<u32>::with_capacity(20);
                    let mut val : u32 = 0;

                    for i in 0..line.len() {
                        if bytes[i] == ' ' as u8 {
                            vec.push(val);
                            val = 0;
                            continue;
                        }
                        let n = bytes[i] as u32 - '0' as u32;
                        if n > 9 {
                            panic!("Unexpected charachter in input: {}", bytes[i] as u64);
                        }
                        val *= 10;
                        val += n;
                    }
                    vec.push(val);
                    triangle.push(vec);
                }
            }
        }

    }

    for t in &triangle {
        println!("{:?}",*t);
    }


    for r in (0..triangle.len()-1).rev() { // row, but start at 1 above the bottom
        for c in 0..triangle[r].len() { // column
            let v1 = triangle[r+1][c];
            let v2 = triangle[r+1][c+1];
            if v1 < v2 {
                triangle[r][c] += v2;
            }
            else {
                triangle[r][c] += v1;
            }
        }
    }


    println!("\nSolution: {}", triangle[0][0]);

    let elapsed = start_time.elapsed().as_micros();
    println!("Elasped time: {} us", elapsed);

}
