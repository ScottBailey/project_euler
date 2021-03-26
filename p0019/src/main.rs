// https://projecteuler.net/problem=19

/*
You are given the following information, but you may prefer to do some research for yourself.

 - 1 Jan 1900 was a Monday.

 - Thirty days has September,
   April, June and November.
   All the rest have thirty-one,
   Saving February alone,
   Which has twenty-eight, rain or shine.
   And on leap years, twenty-nine.

 - A leap year occurs on any year evenly divisible by 4, but not on a
   century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

*/

fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve2();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);
    println!("Elasped time: {} us", elapsed);

}

#[allow(dead_code)]
fn solve1() -> u32 {

    let mut rv = 0;

    let mut day = 365;
    for year in 1901..2001 {
        // January
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // February
        if is_sunday(day) {
            rv += 1;
        }
        day += 28;
        if is_leap_year(year) {
            day += 1;
        }
        // March
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // April
        if is_sunday(day) {
            rv += 1;
        }
        day += 30;
        // May
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // June
        if is_sunday(day) {
            rv += 1;
        }
        day += 30;
        // July
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // August
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // September
        if is_sunday(day) {
            rv += 1;
        }
        day += 30;
        // October
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
        // November
        if is_sunday(day) {
            rv += 1;
        }
        day += 30;
        // December
        if is_sunday(day) {
            rv += 1;
        }
        day += 31;
    }
    return rv;
}


// more compact, but much less efficient
#[allow(dead_code)]
fn solve2() -> u32 {

    let mut rv = 0;

    let mut day = 365;
    for year in 1901..2001 {
        for month in 1..=12 {

            if is_sunday(day) {
                rv += 1;
            }
            day += match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 =>
                    if is_leap_year(year) {
                        29
                    }
                else {
                    28
                },
                n => panic!("Somehow got month number{}", n),
            };
        }
    }
    return rv;
}


fn is_leap_year(year: u32) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 == 0 {
        return year % 400 == 0;
    }
    return true;
}


fn is_sunday(day: u32) -> bool {
    day % 7  == 6
}
