// https://projecteuler.net/problem=15

/*

If the numbers 1 to 5 are written out in words: one, two, three, four,
five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were
written out in words, how many letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred
and forty-two) contains 23 letters and 115 (one hundred and fifteen)
contains 20 letters. The use of "and" when writing out numbers is in
compliance with British usage.

*/


fn main() {

    let start_time = std::time::Instant::now();

    let nums1 = [
        0,
        "one".len(),
        "two".len(),
        "three".len(),
        "four".len(),
        "five".len(),
        "six".len(),
        "seven".len(),
        "eight".len(),
        "nine".len(),
        "ten".len(),
        "eleven".len(),
        "tweleve".len(),
        "thirteen".len(),
        "fourteen".len(),
        "fifteen".len(),
        "sixteen".len(),
        "seventeen".len(),
        "eighteen".len(),
        "ninteen".len(),
    ];

    let nums2 = [
        0, // single digit
        0, // tens and teens
        "twenty".len(),
        "thirty".len(),
        "forty".len(),
        "fifty".len(),
        "sixty".len(),
        "seventy".len(),
        "eighty".len(),
        "ninety".len(),
    ];


    let hundred = "hundred".len();
    let and = "and".len();

    let max = 1000;

    let mut sol = 0;

    for n in 1..max {
        let hundreds = n / 100;
        let teens = n%100;
        let tens = (n/10) % 10;
        let ones = n%10;

        if teens == 0 {
            sol += nums1[hundreds] + hundred;
            continue;
        }

        if hundreds > 0 {
            sol += nums1[hundreds] + hundred + and;
        }
        if teens < 20 {
            sol += nums1[teens];
        }
        else {
            sol += nums2[tens] + nums1[ones];
        }
    }

    if max == 1000 {
        sol += "onethousand".len();
    }

    /*
    // calculate values 1-9
    let mut sub_ten = 0;
    for i in 1..10 {
        sub_ten += nums1[i];
    }
    let sub_ten = sub_ten;

    // calculate values 10-19
    let mut ten_plus = 0;
    for i in 10..20 {
        ten_plus += nums1[i];
    }
    let ten_plus = ten_plus;

    // calculate the values 1-99
    let mut sub_hundred = 0;
    sub_hundred += sub_ten * 9; // add for 0x, 2x, 3x..9x; skip the teens!
    sub_hundred += ten_plus;
    for i in 2..10 {
        sub_hundred += num2[i];
    }
    */



    println!("\nSolution: {}",sol);

    println!("\nElasped time: {} us", start_time.elapsed().as_micros());
}
