// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::time::Instant;
use std::env;
use log::{debug, error, info, warn};
use parse_display::Display;
use std::cmp::Ordering;

fn test() {

    debug!("{:?}",Hand::new("A333A 234",false));
    debug!("{:?}",Hand::new("A323A 2334",false));
    debug!("{:?}",Hand::new("2333A 234",false));
    debug!("{:?}",Hand::new("33333 33",false));
    debug!("{:?}",Hand::new("62344 234",false));
    debug!("{:?}",Hand::new("6234Q 2354",false));

    // Did we f up our Eq/Ord implementation - let's unit-test
    let k3 = Hand::new("6333A 234",false);
    let p2 = Hand::new("A363A 234",false);
    let p22 = Hand::new("3A63A 909",false);
    debug_assert!(k3 > p2);
    debug_assert!(k3 == k3);
    debug_assert!(p2 > p22);
    debug_assert!(p2 != p22);

    // What about with Jokers
    let a1 = Hand::new("J333A 234",false);
    let a2 = Hand::new("A3633 234",false); 
    let b1 = Hand::new("J333A 234",true);
    let b2 = Hand::new("A3633 234",true);  
    debug_assert!(a1<a2);
    debug_assert!(b1>b2);

    let b1 = Hand::new("3J33J 234",true);
    let b2 = Hand::new("J3J33 234",true);  
    debug_assert!(b1>b2);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
            , false
        ) == 6440
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
                , true
        ) == 5905
    );
}

#[derive(Debug,Ord,PartialOrd,Eq,PartialEq,Clone)]
enum HandType {
 HighCard,
 OnePair,
 TwoPair,
 ThreeKind,
 FullHouse,
 FourKind,
 FiveKind,
 Unknown
}

#[derive(Debug,Ord,PartialOrd,Eq,PartialEq,Clone)]
enum CardType {
 Joker,   
 Two,
 Three,
 Four,
 Five,
 Six,
 Seven,
 Eight,
 Nine,
 Ten,
 Jack,
 Queen,
 King,
 Ace,
 Unknown
}

#[derive(Debug,Display,Clone)]
#[display("format")]
struct Hand {
    hand_type: HandType,
    card_string: String,
    cards_v: Vec<CardType>,
    value: u64,
}

impl Hand {
    
    fn new(stri: &str,jokerz: bool) -> Self {

        let splitty:Vec<_>= stri.split(' ').collect();
        let v = splitty[1].parse::<u64>().unwrap();

        let mut joker_count = 0;
        let mut c:Vec<CardType> = splitty[0].chars().map(
            |x| match x {
                'A' => CardType::Ace,
                'K' => CardType::King,
                'Q' => CardType::Queen,
                'J' => if jokerz {joker_count +=1 ; CardType::Joker} else {CardType::Jack},
                'T' => CardType::Ten,
                '9' => CardType::Nine,
                '8' => CardType::Eight,
                '7' => CardType::Seven,
                '6' => CardType::Six,
                '5' => CardType::Five,
                '4' => CardType::Four,
                '3' => CardType::Three,
                '2' => CardType::Two,
                _  => { error!("What the foo card is {}", x); CardType::Unknown}
            }

        ).collect();
        let d=c.clone();

        c.sort();
        let mut ht = HandType::Unknown;
        if c[0] == c[4] {
            ht = HandType::FiveKind;
        } else if c[0] == c[3] || c[1] == c[4] { // Annoying bug I missed the 2nd of these... write more test-cases.
            ht = match joker_count {
                4 => HandType::FiveKind,
                1 => HandType::FiveKind,
                0 => HandType::FourKind,
                _ => { error!("Their morals, their code—it's a bad joke."); HandType::Unknown}
            }

        } else if (c[0] == c[2] && c[3] == c[4]) || ( c[0] == c[1] && c[2] == c[4]) {
            ht = match joker_count {
                3 => HandType::FiveKind,
                2 => HandType::FiveKind,
                0 => HandType::FullHouse,
                _ => { error!("As you know, madness is like gravity: All it takes is a little push."); HandType::Unknown}
            }

        } else if c[0] == c[2] || c[1] == c[3] || c[2] == c[4] {
            ht = match joker_count {
                3 => HandType::FourKind,
                2 => HandType::FiveKind,
                1 => HandType::FourKind,
                0 => HandType::ThreeKind,
                _ => { error!("What merry hell is this?"); HandType::Unknown}
            }
        } else if 
                    (c[0] == c[1] && (c[2] == c[3] || c[3] == c[4])) 
                    || (c[1] == c[2] && c[3] == c[4])
                 {
            ht = match joker_count {
                2 => HandType::FourKind,
                1 => HandType::FullHouse,
                0 => HandType::TwoPair,
                _ => { error!("The only sensible way to live in this world is without rules."); HandType::Unknown}
            }

        } else if c[0] == c[1] || c[1] == c[2] || c[2] == c[3] || c[3] == c[4] {
            ht = match joker_count {
                2 => HandType::ThreeKind,
                1 => HandType::ThreeKind,
                0 => HandType::OnePair,
                _ => { error!("A joke a day keeps the gloom away!"); HandType::Unknown}
            }           
        } else if c[0] != c[1] && c[0] != c[2] && c[0] != c[3] && c[0] != c[4]
        && c[1] != c[2] && c[1] != c[3] && c[1] != c[4] 
        && c[2] != c[3] && c[2] != c[4] 
        && c[3] != c[4] {
        // ^^ Was originally fall thru - but adding for sanity checking when my first solution passed the sample data - but not the answer.
            ht = match joker_count {
                1 => HandType::OnePair,
                0 => HandType::HighCard,
                _ => { error!("It's not about the money, it's about sending a message. Everything burns!"); HandType::Unknown}
            }     

        } else {
            error!("Hand unparseable... {:?}",splitty);
        }
        Self {
            value: v,
            card_string: splitty[0].to_string(),
            cards_v: d,
            hand_type: ht,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) ->Option<std::cmp::Ordering> {
        Some(self.cmp(other)) 
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }
        for (i,c) in self.cards_v.clone().into_iter().enumerate() {
            match c.cmp(&other.cards_v[i]) {
                Ordering::Less => { return Ordering::Less; },
                Ordering::Greater => { return Ordering::Greater; },
                Ordering::Equal => {}, // Consider the next card.
            }            
        }      
        warn!("If this happens our input data has indeterminate answer");
        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_string == other.card_string
    }
}

impl Eq for Hand {} // I'm not sure I understand these traits anymore... but the Rust docs say to do this?

fn part1(data: &str, b: bool) -> u64 {
    
    let mut hands: Vec<Hand> = data.split('\n')
    .filter(|y| !y.is_empty())
    .map(|x| Hand::new(x,b))
    .collect();
  
    hands.sort();
    // hands.clone().into_iter().map(|h|debug!("{:?}",h)).for_each(drop);
    hands.into_iter().enumerate().map(|(i,hand)| (1+u64::try_from(i).unwrap()) *hand.value).sum::<u64>()

}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(), false
    );
    info!("Part1: {}", p1);
    assert!(p1 != 251563193 ); // Too high...
    assert!(p1 == 251029473);
    let p2 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(),true);
    info!("Part2: {}", p2);
    assert!(p2 == 251003917);
    info!("Completed in {} us", now.elapsed().as_micros());
}
