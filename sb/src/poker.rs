// module sb::poker::rank

use std::cmp::Ordering;


//--- card ------------------------------------------------------------------------------------------

#[derive(Debug,Copy,Clone)]
pub struct Card {
    rank : u8,
    suit : u8,
}


impl PartialEq for Card
{
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}


impl PartialOrd for Card {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            return Some(Ordering::Equal);
        }
        if self.rank > other.rank {
            return Some(Ordering::Greater);
        }
        //if self.rank < other.rank {
        return Some(Ordering::Less);
    }
}



#[allow(dead_code)]
// should this be new() ?
pub fn make_card(rank : u8, suit : u8) -> Card {
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


//--- rank ------------------------------------------------------------------------------------------


#[derive(Debug,Clone)]
#[allow(dead_code)]
pub enum Rank {
    HighCard{ rank: Vec::<u8> }, // rank[0] is high card, followed by remaining card ranks in descending order
    Pair{ rank: Vec::<u8>  },    // rank[0] is pair cards, followed by remaining card ranks in descending order
    TwoPair{ rank: Vec::<u8> },  // rank[0] is high pair cards, rank[1] is low pair cards, rank[3] is remaining singleton
    Three{ rank : Vec::<u8> },   // rank[0] is 3 of a kind cards, followed by remaining card ranks in descending order
    Straight{ rank: u8 },        // rank is high card, no other values
    Flush{ rank: Vec::<u8> },    // rank[0] is high card, followed by remaining card ranks in descending order
    FullHouse{ rank: Vec::<u8> },// rank[0] is three cards, rank[1] is pair cards
    Four{ rank: Vec::<u8> },     // rank[0] is four cards, rank[1] is remaining singleton
    StraightFlush{ rank: u8 },   // rank is high card, no other values
    RoyalFlush,                  // no values
}

pub trait RankHand {
    fn rank(&self) -> Rank;
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


impl PartialOrd for Rank {


    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        fn rank_to_int(rank : &Rank) -> u8 {

            match rank {
                Rank::HighCard{ rank : _ } => { return 0; },
                Rank::Pair{ rank : _ } => { return 1; },
                Rank::TwoPair{ rank : _ } => { return 2; },
                Rank::Three{ rank : _ } => { return 3; },
                Rank::Straight{ rank : _ } => { return 4; },
                Rank::Flush{ rank : _ } => { return 5; },
                Rank::FullHouse{ rank : _ } => { return 6; },
                Rank::Four{ rank : _ } => { return 7; },
                Rank::StraightFlush{ rank : _ } => { return 8; },
                Rank::RoyalFlush => { return 9; },
            }
            // panic!("Invalid rank!");
        }

        let self_int = rank_to_int(self);
        let other_int = rank_to_int(other);

        if self_int > other_int {
            return Some(Ordering::Greater);
        }
        else if self_int < other_int {
            return Some(Ordering::Less);
        }

        match self {

            Rank::HighCard { rank : self_rank } => {
                if let Rank::HighCard { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::Pair { rank : self_rank } => {
                if let Rank::Pair { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::TwoPair { rank : self_rank } => {
                if let Rank::TwoPair { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::Three { rank : self_rank } => {
                if let Rank::Three { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::Straight { rank : self_rank } => {
                if let Rank::Straight { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::Flush { rank : self_rank } => {
                if let Rank::Flush { rank : other_rank } = other {
                    if *self_rank == *other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::FullHouse { rank : self_rank } => {
                if let Rank::FullHouse { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::Four { rank : self_rank } => {
                if let Rank::Four { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::StraightFlush { rank : self_rank } => {
                if let Rank::StraightFlush { rank : other_rank } = other {
                    if self_rank == other_rank {
                        return Some(Ordering::Equal);
                    }
                    else if self_rank > other_rank {
                        return Some(Ordering::Greater);
                    }
                    else {
                        return Some(Ordering::Less);
                    }
                }
            },

            Rank::RoyalFlush => {
                if let Rank::RoyalFlush = other {
                    return Some(Ordering::Equal);
                }
            },
        }
        None
    }

}


impl RankHand for Vec::<Card> {

    fn rank(&self) -> Rank {

        // Test for flush.
        let mut is_flush = true;
        let suit = self[0].suit;
        for i in 1..self.len() {
            if self[i].suit != suit {
                is_flush = false;
                break;
            }
        }

        // Rank cards... (Note that duplicates are removed in ranked!)
        let mut ranked = Vec::<u8>::with_capacity(self.len());
        for c in self {
            ranked.push(c.rank);
        }

        ranked.sort_by(|a,b|a.cmp(b));
        ranked.dedup();

        // Test for straight.
        let mut is_straight = true;
        if ranked.len() != self.len() {
            is_straight = false;
        }
        else {
            let mut r = ranked[0];
            for i in 1..ranked.len() {
                r -= 1;
                if ranked[i] != r {
                    is_straight = false;
                    break;
                }
            }
        }

        // Royal Flush and Straigh Flush
        // If it's a straight AND it's a flush, decide on the flavor (StraightFlush, or RoyalFlush) and return
        if is_straight && is_flush {
            let high_card = ranked[0];
            if high_card == 14_u8 { // ace
                return Rank::RoyalFlush;
            }
            return Rank::StraightFlush { rank : high_card };
        }

        // Test for rank multiples
        let mut multiples = [0_u32; 15];  // 0 and 1 are dummy values, 2 is 2, J-11, Q-12, K-13, A-14
        for c in self {
            multiples[c.rank as usize] += 1;
        }

        // Four of a kind and Full house
        if ranked.len() == 2 {
            let mut high : u8 = 0;
            let mut low : u8 = 0;
            let mut is_four = false;
            // Four of a kind
            for (card,count) in multiples.iter().enumerate() {
                match count {
                    4 => {
                        is_four = true;
                        high = card as u8;
                    },
                    3 => high = card as u8,
                    _ => low = card as u8,
                }
                if is_four {
                    return Rank::Four{ rank : vec!(high, low) };
                }
                else {
                    return Rank::FullHouse{ rank : vec!(high, low) };
                }
            }

        }

        if is_straight {
            return Rank::StraightFlush { rank : ranked[0] };
        }


        // Three of a kind, two pair, and single pair
        let mut is_pair = 0_u8;
        for (card,count) in multiples.iter().enumerate() {
            match count {
                3 => {
                    let mut result = Vec::<u8>::with_capacity(ranked.len());
                    result.push(card as u8);
                    for c in ranked {
                        if c != card as u8 {
                            result.push(c);
                        }
                    }
                    return Rank::Three { rank : result };
                },
                2 => {
                    if is_pair != 0 {
                        let mut result = Vec::<u8>::with_capacity(ranked.len());
                        result.push(card as u8);
                        result.push(is_pair);
                        result.sort_by(|a,b|a.cmp(b));
                        for c in ranked {
	                    if c != card as u8 && c != is_pair {
                                result.push(c);
                            }
                        }
                        return Rank::TwoPair { rank : result };
                    }
                    else {
                        is_pair = card as u8;
                    }
                }
                _ => {},
            }
        }

        if is_pair != 0 {
            let mut result = Vec::<u8>::with_capacity(ranked.len());
            result.push(is_pair);
            for c in ranked {
	        if c != is_pair {
                    result.push(c);
                }
            }
            return Rank::Pair { rank : result };
        }

        return Rank::HighCard { rank : ranked };
    }

}


//--- hand ------------------------------------------------------------------------------------------

#[derive(Debug,Clone)]
pub struct Hand {
    cards : Vec::<Card>,
}

impl Hand {

    pub fn new() -> Self {
        Hand { cards : Vec::<Card>::with_capacity(5) }
    }

    pub fn add_card(&mut self, rank : u8, suit : u8) {
        self.cards.push( make_card(rank,suit) );
    }

}


impl RankHand for Hand {

    fn rank(&self) -> Rank {
        self.cards.rank()
    }
}


impl PartialEq for Hand
{
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}


impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank().partial_cmp(&other.rank())
    }
}


//--- tests ------------------------------------------------------------------------------------------

#[test]
pub fn test_rank() {

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
