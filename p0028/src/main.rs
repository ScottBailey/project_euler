/*

https://projecteuler.net

Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

   [21] 22 23 24 [25]
    20  [7] 8 [9] 10
    19   6 [1] 2  11
    18  [5] 4 [3] 12
   [17] 16 15 14 [13]

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

NOTES:

  My first solution was not the most optimal, but it was the most
  fun!! I wanted to fill out the grid!!! :lol:

  Redid this with math.

*/

const SZ : usize = 1001;
const MID : usize = SZ/2;


fn solve() -> u64 {

    let mut last = 1;
    let mut sum = 1;
    for width in (2..SZ as u64).step_by(2) {
        last += width;
        sum += last;
        last += width;
        sum += last;
        last += width;
        sum += last;
        last += width;
        sum += last;
    }

    sum
}


fn solve_slow() -> u64 {

    let mut v = std::vec::Vec::< std::vec::Vec::<u64> >::with_capacity(SZ);

    for _ in 0..SZ {
        let t : std::vec::Vec::<u64> = vec![0;SZ];
        v.push(t);
    }


    {
        let mut x = MID;
        let mut y = MID;
        let mut n = 1;
        let mut width = 0;

        loop {
            for _ in 0..width+1 { // across right
                v[x][y] = n;
                x += 1;
                n += 1;
            }

            if x >= SZ {
                break;
            }

            width += 2;

            for _ in 0..width-1 { // down right
                v[x][y] = n;
                y += 1;
                n += 1;
            }
            for _ in 0..width { // across bottom
                v[x][y] = n;
                x -= 1;
                n += 1;
            }
            for _ in 0..width { // up left
                v[x][y] = n;
                y -= 1;
                n += 1;
            }
        }

    }

    if SZ < 22 {
        dump(&v);
    }

    let mut rv = 0;
    for i in 0..SZ {
        rv += v[i][i];
        rv += v[i][SZ-i-1];
    }
    rv -= v[MID][MID]; // more optimal to subtract the extra than to test inside the loop

    rv
}


fn dump( v : &std::vec::Vec::< std::vec::Vec::<u64> > ) {

    let max_x = v.len();
    let max_y = v[0].len();

    for y in 0..max_y {
        for x in 0..max_x {
            print!(" {:4}", v[x][y]);
        }
        print!("\n");
    }
}



fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    let slow = solve_slow();
    if sol != slow {
        panic!("expected: {}, received: {}", slow, sol);
    }

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
