/*

https://projecteuler.net

In the card game poker, a hand consists of five cards and are ranked,
from lowest to highest, in the following way:

  - High Card: Highest value card.
  - One Pair: Two cards of the same value.
  - Two Pairs: Two different pairs.
  - Three of a Kind: Three cards of the same value.
  - Straight: All cards are consecutive values.
  - Flush: All cards of the same suit.
  - Full House: Three of a kind and a pair.
  - Four of a Kind: Four cards of the same value.
  - Straight Flush: All cards are consecutive values of same suit.
  - Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

The cards are valued in the order:
  2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.

If two players have the same ranked hands then the rank made up of the
highest value wins; for example, a pair of eights beats a pair of
fives (see example 1 below). But if two ranks tie, for example, both
players have a pair of queens, then highest cards in each hand are
compared (see example 4 below); if the highest cards tie then the next
highest cards are compared, and so on.

Consider the following five hands dealt to two players:

Hand	 	Player 1	 	Player 2	 	Winner

1	 	5H 5C 6S 7S KD          2C 3S 8S 8D TD          Player 2
                Pair of Fives           Pair of Eights

2	 	5D 8C 9S JS AC          2C 5C 7D 8S QH          Player 1
                Highest card Ace        Highest card Queen

3	 	2D 9C AS AH AC 	        3D 6D 7D TD QD          Player 2
                Three Aces              Flush with Diamonds

4	 	4D 6S 9H QH QC          3D 6D 7H QD QS          Player 1
                Pair of Queens          Pair of Queens
                Highest card Nine       Highest card Seven

5	 	2H 2D 4C 4D 4S          3C 3D 3S 9S 9D         Player 1
                Full House              Full House
                With Three Fours        with Three Threes

The file, `poker.txt`, contains one-thousand random hands dealt to two
players. Each line of the file contains ten cards (separated by a
single space): the first five are Player 1's cards and the last five
are Player 2's cards. You can assume that all hands are valid (no
invalid characters or repeated cards), each player's hand is in no
specific order, and in each hand there is a clear winner.

How many hands does Player 1 win?

NOTES:


*/

use std::io::BufRead;


#[derive(Debug)]
struct Card {
    rank : u8,
    suit : u8,
}

fn make_card(rank : u8, suit : u8) -> Card {
    let mut rv = Card { rank: 0, suit: 0 };
    match rank as char {
        '2'..='9' => rv.rank = rank as u8 - '0' as u8,
        'T' => rv.rank = 10,
        'J' => rv.rank = 11,
        'Q' => rv.rank = 12,
        'K' => rv.rank = 13,
        'A' => rv.rank = 14,
        _ => panic!("Unexpected rank: {}", rank),
    }
    match suit as char {
        'C' => rv.suit = 1,
        'D' => rv.suit = 2,
        'H' => rv.suit = 3,
        'S' => rv.suit = 4,
        _ => panic!("Unexpected suit: {}", suit),
    }
    rv
}


trait PokerHand {
    fn is_royal_flush(&self) -> bool;
    fn is_straight_flush(&self) -> bool;
    fn is_four(&self) -> bool;
    fn is_full_house(&self) -> bool;
    fn is_flush(&self) -> bool;
    fn is_straight(&self) -> bool;
    fn is_three(&self) -> bool;
    fn is_two_pair(&self) -> bool;
    fn is_pair(&self) -> bool;

    fn high_card(&self) -> u8;
}

impl PokerHand for Vec::<Card> {

    fn high_card(&self) -> u8 {
        let mut rv = 0;
        for c in self {
            if c.rank > rv {
                rv = c.rank;
            }
        }
        rv
    }

    fn is_flush(&self) -> bool {
        let suit = self[0].suit;
        for i in 1..self.len() {
            if self[i].suit != suit {
                return false;
            }
        }
        true
    }

    fn is_four(&self) -> bool {
        let mut v = [0_u32; 15];
        for c in self {
            v[c.rank as usize] += 1;
        }
        for n in v {
            if n == 4 {
                return true;
            }
        }
        false
    }

    fn is_full_house(&self) -> bool {
        let mut v = [0_u32; 15];
        for c in self {
            v[c.rank as usize] += 1;
        }
        let mut crit = 0;
        for n in v {
            if n == 3 {
                crit += 1;
            }
            else if n == 2 {
                crit += 1;
            }
        }
        crit == 2
    }

    fn is_pair(&self) -> bool {
        let mut v = [0_u32; 15];
        for c in self {
            v[c.rank as usize] += 1;
        }
        for n in v {
            if n == 2 {
                return true;
            }
        }
        false
    }

    fn is_royal_flush(&self) -> bool {
        self.high_card() == 14 && self.is_straight_flush()
    }

    fn is_straight(&self) -> bool {
        let mut v = Vec::<u8>::with_capacity(self.len());
        for c in self {
            v.push(c.rank);
        }
        v.sort();
        let mut rank = v[0];
        for i in 1..v.len() {
            rank += 1;
            if v[i] != rank {
                return false;
            }
        }
        return true;
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() &&  self.is_straight()
    }

    fn is_three(&self) -> bool {
        let mut v = [0_u32; 15];
        for c in self {
            v[c.rank as usize] += 1;
        }
        for n in v {
            if n == 3 {
                return true;
            }
        }
        false
    }

    fn is_two_pair(&self) -> bool {
        let mut v = [0_u32; 15];
        for c in self {
            v[c.rank as usize] += 1;
        }
        let mut crit = 0;
        for n in v {
            if n == 2 {
                crit += 1;
            }
        }
        crit == 2
    }

}



// return true if p1 beats p2
fn compare_hands(_p1 : Vec::<Card>, _p2 : Vec::<Card>) -> bool {
    false

}

fn solve() -> u64 {

    let args : std::vec::Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("needs a single argument for the input file");
    }

    let path = std::path::Path::new(&args[1]);
    // Open the path in read-only mode, returns `io::Result<File>`
    let infile = match std::fs::File::open(path) {
	Err(why) => panic!("couldn't open {}: {}", path.display(), why),
	Ok(infile) => infile,
    };

    let reader = std::io::BufReader::new(infile);

    for line in reader.lines() {
        if let Ok(s) = line {
            if s.len() != 29 {
                println!("bad size: {}", s.len());
                continue;
            }

            let mut p1 = Vec::<Card>::with_capacity(5);
            let mut p2 = Vec::<Card>::with_capacity(5);
            for i in 0..10 {
                let offset : usize = 3*i;
                let c = make_card( s.as_bytes()[offset], s.as_bytes()[offset+1]);
                if i < 5 {
                    p1.push(c);
                }
                else {
                    p2.push(c);
                }
            }

            println!("p1: {:?}\np2: {:?}\n", p1, p2);

        }
    }

    0
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


#[test]
fn test_bcd() {
    let mut n;

    n = sb::math::to_bcd(1);
    assert!(n == 0x01);
    n = sb::math::from_bcd(n);
    assert!(n == 1);

    n = sb::math::to_bcd(90);
    assert!(n == 0x09_00);
    n = sb::math::from_bcd(n);
    assert!(n == 90);

    n = sb::math::to_bcd(100);
    assert!(n == 0x01_00_00);
    n = sb::math::from_bcd(n);
    assert!(n == 100);

}
