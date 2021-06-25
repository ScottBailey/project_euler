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


impl PartialEq for Card
{
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}


fn make_card(rank : u8, suit : u8) -> Card {
    let mut rv = Card { rank: 0, suit: 0 };

    const TWO : char = 2 as char;
    const FOURTEEN : char = 14 as char;
    match rank as char {
        '2'..='9' => rv.rank = rank as u8 - '0' as u8,
        'T' => rv.rank = 10,
        'J' => rv.rank = 11,
        'Q' => rv.rank = 12,
        'K' => rv.rank = 13,
        'A' => rv.rank = 14,
        (TWO ..= FOURTEEN) => rv.rank = rank,
        _ => panic!("Unexpected rank: {}", rank),
    }

    const ONE : char = 1 as char;
    const FOUR : char = 4 as char;
    match suit as char {
        'C' => rv.suit = 1,
        'D' => rv.suit = 2,
        'H' => rv.suit = 3,
        'S' => rv.suit = 4,
        (ONE ..= FOUR) => rv.suit = suit,
        _ => panic!("Unexpected suit: {}", suit),
    }
    rv
}


#[derive(Debug)]
enum Rank {
    HighCard{ rank: Vec::<u8> }, // rank[0] is high card, followed by remaining card ranks in descending order
    Pair{ rank: Vec::<u8>  },    // rank[0] is pair cards, followed by remaining card ranks in descending order
    TwoPair{ rank: Vec::<u8> },  // rank[0] is high pair cards, rank[1] is low pair cards, rank[3] is remaining singleton
    Three{ rank : Vec::<u8> },   // rank[0] is 3 of a kind cards, followed by remaining card ranks in descending order
    Straight{ rank: u8 },        // rank is high card, no other values
    Flush{ rank: Box<Rank> },    // card_rank is the next rank below (i.e. 3kind, 2pair, pair, high card)
    FullHouse{ rank: Vec::<u8> },// rank[0] is three cards, rank[1] is pair cards
    Four{ rank: Vec::<u8> },     // rank[0] is four cards, rank[1] is remaining singleton
    StraightFlush{ rank: u8 },   // rank is high card, no other values
    RoyalFlush,                  // no values
}

trait RankHand {
    fn rank(&self) -> Rank;
}

#[test]
fn test_rank() {

    let hand_rf1 : Vec::<Card> = vec![ make_card(14, 1), make_card(13, 1), make_card(12, 1), make_card(11, 1), make_card(10,1) ];
    let hand_rf2 : Vec::<Card> = vec![ make_card(14, 2), make_card(13, 2), make_card(12, 2), make_card(11, 2), make_card(10,2) ];

    let hand_straight_ace : Vec::<Card> = vec![ make_card(14, 2), make_card(13, 1), make_card(12, 1), make_card(11, 1), make_card(10,1) ];
    let hand_straight_king : Vec::<Card> = vec![ make_card(13, 1), make_card(12, 1), make_card(11, 1), make_card(10,1), make_card(9,3) ];

    assert!(hand_rf1.rank() == hand_rf2.rank());

    println!("{:?}", hand_rf1.rank());
    println!("{:?}", hand_straight_ace.rank());

    assert!(hand_rf1.rank() != hand_straight_ace.rank());
    assert!(hand_straight_king.rank() != hand_straight_ace.rank());
}


impl PartialEq for Rank {

    fn eq(&self, other: &Self) -> bool {

        match self {

            Rank::HighCard { rank : self_rank } => {
                if let Rank::HighCard { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::Pair { rank : self_rank } => {
                if let Rank::Pair { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::TwoPair { rank : self_rank } => {
                if let Rank::TwoPair { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::Three { rank : self_rank } => {
                if let Rank::Three { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::Straight { rank : self_rank } => {
                if let Rank::Straight { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::Flush { rank : self_rank } => {
                if let Rank::Flush { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::FullHouse { rank : self_rank } => {
                if let Rank::FullHouse { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::Four { rank : self_rank } => {
                if let Rank::Four { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::StraightFlush { rank : self_rank } => {
                if let Rank::StraightFlush { rank : other_rank } = other {
                    return self_rank == other_rank;
                }
                else {
                    return false;
                }
            },

            Rank::RoyalFlush => {
                if let Rank::RoyalFlush = other {
                    return true;
                }
                else {
                    return false;
                }
            },

        }
    }

}


/*
impl PartialEq for Vec::<Card> {

    fn eq(&self, other: &Self) -> bool {

    }

}
 */


impl RankHand for Vec::<Card> {

    fn rank(&self) -> Rank {

        // test for flush
        let mut is_flush = true;
        let suit = self[0].suit;
        for i in 1..self.len() {
            if self[i].suit != suit {
                is_flush = false;
                break;
            }
        }

        // rank cards, test for straight
        let mut ranked = Vec::<u8>::with_capacity(self.len());
        for c in self {
            ranked.push(c.rank);
        }
        ranked.sort();
        ranked.dedup();
        let mut is_straight = true;
        if ranked.len() != self.len() {
            is_straight = false;
        }
        else {
            let mut r = ranked[0];
            for i in 1..ranked.len() {
                r += 1;
                if ranked[i] != r {
                    is_straight = false;
                    break;
                }
            }
        }

        // If it's a straight, decide on the flavor (Straight, StraightFlush, or RoyalFlush) and return.
        if is_straight {
            let high_card = ranked.last().unwrap();
            if is_flush {
                if *high_card == 14_u8 { // ace
                    return Rank::RoyalFlush;
                }
                return Rank::StraightFlush { rank : *high_card };
            }
            return Rank::Straight { rank : *high_card };
        }


        // Test for rank multiples
        let mut multiples = [0_u32; 15];
        for c in self {
            multiples[c.rank as usize] += 1;
        }

        panic!("not completed yet!");
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
