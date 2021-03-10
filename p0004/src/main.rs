// https://projecteuler.net/problem=4

/*

A palindromic number reads the same both ways. The largest palindrome
made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

*/


fn main() {

    let mut sol = 0;
    let mut s1 = 0;
    let mut s2 = 0;

    let mut n1 = 999;
    while n1 > 100 {
        let mut n2 = n1;
        if n1 * n2 <  sol {
            break;
        }
        while n2 > 100 {
            let test = n1 * n2;
            if test < sol {
                break;
            }
            if is_palindrome(test) {
                s1 = n1;
                s2 = n2;
                sol = test;
                break;
            }
            n2 -= 1;
        }
        n1 -= 1;
    }
    println!("{} x {} = {}", s1, s2, sol);
}


fn is_palindrome(mut n : u64) -> bool {
    let mut v = std::vec::Vec::new();
    while n > 0 {
        v.push(n%10);
        n = n/10;
    }
    if v.len() < 2 {
        return false;
    }
    let mut l = 0;
    let mut r = v.len()-1;
    while l < r {
        if v[l] != v[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    return true;
}
