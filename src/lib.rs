pub mod cards;

use cards::Card;

use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Score {
    HighCard{
        card: Card,
        // The kickers, from lowest to highest rank card.
        kickers: [Card; 4]
    },
    Pair{ 
        card: Card,
        // The kickers, from lowest to highest rank card.
        kickers: [Card; 3]
    },
    TwoPair{
        low_pair: Card,
        high_pair: Card,
        kicker: Card
    },
    ThreeOfAKind {
        card: Card,
        high_kicker: Card,
        low_kicker: Card
    },
    Straight(Card),
    FullHouse{
        pair: Card,
        three_of_a_kind: Card
    },
    FourOfAKind {
        four_of_a_kind: Card,
        kicker: Card
    }
}

#[derive(Debug, PartialEq)]
pub struct PokerHand {
    cards: [Card; 5]
}

impl PokerHand {
    pub fn from_str(input:&str) -> Result<PokerHand, String> {
        // Rust has a very strict rules around indexing into strings.  Something something grapheme clusters and 
        // English is a pretty boring language as far as unicode goes.
        let chars:Vec<char> = input.chars().collect();
        if chars.len() != 5 {
            return Err(format!("Required 5 characters but found {}.", chars.len()));
        }

        Ok(PokerHand {
            cards: [Card::from_char(&chars[0])?, 
                Card::from_char(&chars[1])?, 
                Card::from_char(&chars[2])?, 
                Card::from_char(&chars[3])?, 
                Card::from_char(&chars[4])?]
        })
    }

    pub fn score(&self) -> Score {
        let mut cards: Vec<&Card> = Vec::new();
            cards.extend(self.cards().iter());
            cards.sort_by_key(|card| card_seq(card));

            // Count the number of each card.  
            let mut counts: BTreeMap<&Card, usize> = BTreeMap::new();
            for card in &cards {
                counts.entry(card).and_modify(|e| *e += 1).or_insert(1);
            }

            let mut pairs:Vec<Card> = Vec::new();
            let mut three_of_a_kind: Option<Card> = None;
            let mut four_of_a_kind: Option<Card> = None;
            
            for (card, count) in counts.iter() {
                if *count == 2 {
                    pairs.push((*card).clone());
                } else if *count == 3 {
                    three_of_a_kind = Some((*card).clone());
                } else if *count == 4 {
                    four_of_a_kind = Some((*card).clone());
                } else if *count == 1 {
                    // don't care
                } else {
                    panic!(format!("Unexpected card card {}", count));
                }
            }

            pairs.sort_by_key(|card| card_seq(card));


            // Four of a kind
            if four_of_a_kind.is_some() {
                let mut kickers: Vec<Card> = Vec::new();
                let c = four_of_a_kind.unwrap().clone();
                for card in self.cards() {
                    if card != &c {
                        kickers.push(card.clone());
                    }
                }

                return Score::FourOfAKind{ four_of_a_kind: c, kicker: kickers[0].clone() };
            }

            // What's a full house?
            if !pairs.is_empty() && three_of_a_kind.is_some() {
                return Score::FullHouse{pair:pairs.last().unwrap().clone(), three_of_a_kind: three_of_a_kind.unwrap()};
            }

            // Straight (Five Highh)
            if cards[0] == &Card::Two &&
                cards[1] == &Card::Three &&
                cards[2] == &Card::Four &&
                cards[3] == &Card::Five &&
                cards[4] == &Card::Ace {
                return Score::Straight(Card::Five);
            }

            // Straight (Ace High)
            let first_card = card_seq(cards[0]);
            if card_seq(cards[1]) == first_card + 1 &&
                card_seq(cards[2]) == first_card + 2 &&
                card_seq(cards[3]) == first_card + 3 &&
                card_seq(cards[4]) == first_card + 4 {
                return Score::Straight(cards[4].clone());
            }

            // Two pair
            if pairs.len() == 2 {
                let mut kickers: Vec<Card> = Vec::new();
                for card in self.cards() {
                    if card != &pairs[0] && card != &pairs[1] {
                        kickers.push(card.clone());
                    }
                }

                return Score::TwoPair{ low_pair:pairs[0].clone(), high_pair:pairs[1].clone(), kicker:kickers[0].clone()};
            }

            // Three of a kind
            if three_of_a_kind.is_some() { // todo if let
                let mut kickers: Vec<Card> = Vec::new();
                let toc = three_of_a_kind.unwrap();
                for card in self.cards() {
                    if card != &toc {
                        kickers.push(card.clone());
                    }
                }

                kickers.sort_by_key(|card| card_seq(card));
                return Score::ThreeOfAKind{card:toc, low_kicker:kickers[0].clone(), high_kicker:kickers[1].clone()};
            }

            // Pairs
            if !pairs.is_empty() {
                let mut kickers: Vec<Card> = Vec::new();
                for card in self.cards() {
                    if card != &pairs[0] {
                        kickers.push(card.clone());
                    }
                }

                kickers.sort_by_key(|card| card_seq(card));

                return Score::Pair{
                    card:pairs.last().unwrap().clone(),
                    kickers: [kickers[0].clone(), kickers[1].clone(), kickers[2].clone()]

                };
            }

            // Final score, just the highest card.
            Score::HighCard{ card:cards[4].clone(),
                kickers: [cards[0].clone(), cards[1].clone(), cards[2].clone(), cards[3].clone() ]
            }
    }

    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }
}

/**
 * Return an integer that represents the sequence of a card as
 * played in Poker.
 */
fn card_seq(card: &Card) -> usize {
    match card {
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::Jack => 11,
        Card::Queen => 12,
        Card::King => 13,
        Card::Ace => 14,
        
    }
}

fn score_seq(score:&Score) -> usize {
    match score {
        Score::HighCard{..} => 1,
        Score::Pair{..} => 2,
        Score::TwoPair{..} => 3,
        Score::ThreeOfAKind{..} => 4,
        Score::Straight(_) => 5, 
        Score::FullHouse{..} => 6,
        Score::FourOfAKind{..} => 7,
    }
}


fn cmp_cards(left_card:&Card, right_card:&Card) -> Ordering {
    card_seq(&right_card).cmp(&card_seq(&left_card))
}

/** Compare two scored poker hands.
 * 
 */
pub fn compare_hands(left_score:&Score, right_score:&Score) -> Ordering {
    // Compare the score of a poker hand.  There are rules that need to be applied if the score is the same
    // for each hand.   
    match (left_score, right_score) {
        (Score::HighCard{card:left_card, kickers: left_kickers}, Score::HighCard{card:right_card, kickers: right_kickers}) => {
            match cmp_cards(left_card, right_card)
            {
                Ordering:: Equal => match cmp_cards(&left_kickers[0], &right_kickers[0]) {
                    Ordering:: Equal => match cmp_cards(&left_kickers[1], &right_kickers[1]) {
                        Ordering:: Equal => match cmp_cards(&left_kickers[2], &right_kickers[2]) {
                            Ordering:: Equal => cmp_cards(&left_kickers[3], &right_kickers[3]),
                            x => x
                        },
                        x => x
                    },
                    x => x
                },
                x => x
            }
        },
        (Score::Pair{card:left_card, kickers: left_kickers}, Score::Pair{card:right_card, kickers: right_kickers}) => {
            match cmp_cards(left_card, right_card)
            {
                Ordering:: Equal => match cmp_cards(&left_kickers[0], &right_kickers[0]) {
                    Ordering:: Equal => match cmp_cards(&left_kickers[1], &right_kickers[1]) {
                        Ordering:: Equal => cmp_cards(&left_kickers[2], &right_kickers[2]),
                        x => x
                    },
                    x => x
                },
                x => x
            }
        },
        (Score::TwoPair{low_pair:left_low_pair, high_pair:left_high_pair, kicker:left_kicker}, Score::TwoPair{low_pair:right_low_pair, high_pair:right_high_pair, kicker:right_kicker}) => {
            match cmp_cards(left_high_pair, right_high_pair) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match cmp_cards(left_low_pair, right_low_pair) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => cmp_cards(left_kicker, right_kicker)
                }
            }
        },
        (Score::ThreeOfAKind{card:left_card, high_kicker:left_high_kicker, low_kicker:left_low_kicker}, 
            Score::ThreeOfAKind{card:right_card, high_kicker:right_high_kicker, low_kicker:right_low_kicker}) => {
            match cmp_cards(left_card, right_card) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => 
                    match cmp_cards(left_high_kicker, right_high_kicker) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => cmp_cards(left_low_kicker, right_low_kicker)
                    }
                
            }
        },
        (Score::Straight(left_card), Score::Straight(right_card)) => {
            cmp_cards(left_card, right_card)
        },
        (Score::FullHouse{three_of_a_kind:left_three_of_a_kind, pair:left_pair}, Score::FullHouse{three_of_a_kind:right_three_of_a_kind, pair: right_pair}) => {
            match cmp_cards(left_three_of_a_kind, right_three_of_a_kind) {
                Ordering::Equal => cmp_cards(left_pair, right_pair),
                less_or_greater => less_or_greater
            }
        },
        (Score::FourOfAKind{four_of_a_kind:left_four_of_a_kind, kicker:left_kicker}, Score::FourOfAKind{four_of_a_kind: right_four_of_a_kind, kicker: right_kicker}) => {
            match cmp_cards(left_four_of_a_kind, right_four_of_a_kind) {
                Ordering::Equal => cmp_cards(left_kicker, right_kicker),
                less_or_greater => less_or_greater
            }
        }

        // In the simple case then just compare the sequences of the scores themselves.
        (left, right) => { score_seq(&right).cmp(&score_seq(&left)) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_pair() {
        assert_eq!(PokerHand::from_str("23427").unwrap().score(), Score::Pair{ card:Card::Two, kickers:[Card::Three, Card::Four, Card::Seven]});
    }

    #[test]
    fn score_two_pair() {
        assert_eq!(PokerHand::from_str("3434A").unwrap().score(), Score::TwoPair{low_pair:Card::Three, high_pair:Card::Four, kicker:Card::Ace});
    }

    #[test]
    fn score_ace_high_straight() {
       assert_eq!(PokerHand::from_str("TJQKA").unwrap().score(), Score::Straight(Card::Ace));
    }

    #[test]
    fn score_five_high_straight() {
       assert_eq!(PokerHand::from_str("A2345").unwrap().score(), Score::Straight(Card::Five));
    }
    
    #[test]
    fn score_high_card() {
        assert_eq!(PokerHand::from_str("23457").unwrap().score(), Score::HighCard{card:Card::Seven, kickers:[Card::Two, Card::Three, Card::Four, Card::Five]});
    }

    #[test]
    fn score_high_card_other_order() {
        assert_eq!(PokerHand::from_str("75432").unwrap().score(), Score::HighCard{card:Card::Seven, kickers:[Card::Two, Card::Three, Card::Four, Card::Five]});
    }
    
    #[test]
    fn score_high_card_ace() {
        assert_eq!(PokerHand::from_str("A235K").unwrap().score(), Score::HighCard{card:Card::Ace, kickers:[Card::Two, Card::Three, Card::Five, Card::King]});
    }

    #[test]
    fn score_four_of_a_king() {
        assert_eq!(PokerHand::from_str("A2222").unwrap().score(), Score::FourOfAKind{four_of_a_kind:Card::Two, kicker:Card::Ace});
    }



    #[test]
    fn hand_from_char_wrong_length() {
        assert_eq!(PokerHand::from_str(&"TTTTTT"), Result::Err(String::from("Required 5 characters but found 6.")));
    }

    #[test]
    fn hand_from_char_bad_char() {
        assert_eq!(PokerHand::from_str(&"TTTTX"), Result::Err(String::from("Character 'X' is not valid.")));
    }

    #[test]
    fn hand_from_char_ok() {
        assert_eq!(PokerHand::from_str(&"23456"), 
            Result::Ok(
                PokerHand {
                    cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]
                }
            )
        );
    }

}