// https://projecteuler.net/problem=15

/*

Starting in the top left corner of a 2×2 grid, and only being able to
move to the right and down, there are exactly 6 routes to the bottom
right corner.

rrdd - right right down down
rdrd
rddr
drrd
drdr
ddrr

How many such routes are there through a 20×20 grid?

*/



fn main() {
    let start_time = std::time::Instant::now();

    //const SZ : usize = 2+1;
    const SZ : usize = 20+1;

    let mut locs : [ [u64; SZ] ; SZ ] = [ [0 ; SZ] ;SZ ];

    // set the south and east edges:
    for i in 0..SZ {
        locs[SZ-1][i] = 1;
        locs[i][SZ-1] = 1;
    }

    // starting in the far south east corner, fill rows east first, then north
    for y in (0..SZ-1).rev() {
        for x in (0..SZ-1).rev() {
            locs[x][y] = locs[x+1][y] + locs[x][y+1];
            /*
            println!("{} {}", x, y);
            for i in 0..SZ {
                println!("  {:?}", locs[i]);
            }
            */
        }
    }

    println!("\nSolution: {}", locs[0][0]);

    let elapsed = start_time.elapsed().as_micros();
    println!("Elasped time: {} us", elapsed);

}
