// use itertools::Itertools;
//use std::collections::{HashSet, VecDeque};
// use std::hash::Hash;
use std::time::Instant;
use std::env;
use log::{debug, error, info, log_enabled, warn, Level};
use parse_display::Display;
use std::cmp::Ordering;

fn test() {

    debug!("{:?}",Hand::new("A333A 234"));
    debug!("{:?}",Hand::new("A323A 2334"));
    debug!("{:?}",Hand::new("2333A 234"));
    debug!("{:?}",Hand::new("33333 33"));
    debug!("{:?}",Hand::new("62344 234"));
    debug!("{:?}",Hand::new("6234Q 2354"));


    // Did we f up our Eq/Ord implementation - let's unit-test
    let k3 = Hand::new("6333A 234");
    let p2 = Hand::new("A363A 234");
    let p22 = Hand::new("3A63A 909");
    debug_assert!(k3 > p2);
    debug_assert!(k3 == k3);
    debug_assert!(p2 > p22);
    debug_assert!((p2 == p22) == false);


    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 6440
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
        ) == 888
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
}

#[derive(Debug,Ord,PartialOrd,Eq,PartialEq,Clone)]
enum CardType {
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
    
    fn new(stri: &str) -> Self {

        let splitty:Vec<_>= stri.split(' ').collect();
        let v = splitty[1].parse::<u64>().unwrap();

        let mut c:Vec<CardType> = splitty[0].chars().into_iter().map(
            |x| match x {
                'A' => CardType::Ace,
                'K' => CardType::King,
                'Q' => CardType::Queen,
                'J' => CardType::Jack,
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
        let mut ht = HandType::HighCard;
        if c[0] == c[4] {
            ht = HandType::FiveKind;
        } else if c[0] == c[3] || c[1] == c[4] { // Annoying bug I missed the 2nd of these... write more test-cases.
            ht = HandType::FourKind       
        } else if (c[0] == c[2] && c[3] == c[4]) || ( c[0] == c[1] && c[2] == c[4]) {
            ht = HandType::FullHouse;
        } else if c[0] == c[2] || c[1] == c[3] || c[2] == c[4] {
            ht = HandType::ThreeKind;
        } else if 
                    (c[0] == c[1] && (c[2] == c[3] || c[3] == c[4])) 
                    || (c[1] == c[2] && c[3] == c[4])
                 {
            ht = HandType::TwoPair;
        } else if c[0] == c[1] || c[1] == c[2] || c[2] == c[3] || c[3] == c[4] {
            ht = HandType::OnePair
        } else if c[0] != c[1] && c[0] != c[2] && c[0] != c[3] && c[0] != c[4]
        && c[1] != c[2] && c[1] != c[3] && c[1] != c[4] 
        && c[2] != c[3] && c[2] != c[4] 
        && c[3] != c[4] {
        // ^^ Was originally fall thru - but adding for sanity checking when my first solution passed the sample data - but not the answer.
            ht = HandType::HighCard;
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
        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }
        for (i,c) in self.cards_v.clone().into_iter().enumerate() {
            if c > other.cards_v[i] {
                return Some(Ordering::Greater);
            } else if c < other.cards_v[i] {
                return Some(Ordering::Less);
            }
            // If equal - consider the next card.
        }      
        Some(Ordering::Equal)
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
            if c > other.cards_v[i] {
                return Ordering::Greater;
            } else if c < other.cards_v[i] {
                return Ordering::Less;
            }
            // If equal - consider the next card.
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

impl Eq for Hand {} // I'm not sure I understand these traits anymore...


fn part1(data: &str) -> u64 {
    
    let mut hands: Vec<Hand> = data.split('\n')
    .filter(|y| !y.is_empty())
    .map(|x| Hand::new(x))
    .collect();
  
    hands.sort();

    // for (i,h) in hands.clone().into_iter().enumerate() {
    //     debug!("{}  * {} = {}",i,h.value,1+u64::try_from(i).unwrap() *h.value);
    // }
    info!("====");
    hands.clone().into_iter().map(|h|debug!("{:?}",h)).for_each(drop);

    
    hands.into_iter().enumerate().map(|(i,hand)| (1+u64::try_from(i).unwrap()) *hand.value).sum::<u64>()

}
fn part2(data: &str) -> u64 {
    888
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str()
    );
    info!("Part1: {}", p1);
    assert!(p1 != 251563193 ); // Too high...
    assert!(p1 == 251029473);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    info!("Part2: {}", p2);
    assert!(p2 == 888);
    info!("Completed in {} us", now.elapsed().as_micros());
}
