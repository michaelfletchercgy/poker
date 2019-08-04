pub mod cards;

use cards::Card;

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Score {
    HighCard(Card),
    Pair(Card),
    TwoPair{
        low_pair: Card,
        high_pair: Card
    },
    ThreeOfAKind(Card),
    Straight,
    FullHouse{
        pair: Card,
        three_of_a_kind: Card
    },
    FourOfAKind(Card)
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
        Card::Ace => 1,
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
        Card::King => 13
    }
}

/**
 * Score this hand.
 */
pub fn score(hand: &PokerHand) -> Score {
    let mut cards: Vec<&Card> = Vec::new();
    cards.extend(hand.cards().iter());
    cards.sort_by_key(|card| card_seq(card));

    // Count the number of each card.  
    let mut counts: HashMap<&Card, usize> = HashMap::new();
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
        return Score::FourOfAKind(four_of_a_kind.unwrap());
    }

    // What's a full house?
    if !pairs.is_empty() && three_of_a_kind.is_some() {
        return Score::FullHouse{pair:pairs.last().unwrap().clone(), three_of_a_kind: three_of_a_kind.unwrap()};
    }

    // Straight
    let first_card = card_seq(cards[0]);
    if card_seq(cards[1]) == first_card + 1 &&
        card_seq(cards[2]) == first_card + 2 &&
        card_seq(cards[3]) == first_card + 3 &&
        card_seq(cards[4]) == first_card + 4 {
        return Score::Straight;
    }

    // Two pair
    if pairs.len() == 2 {
        return Score::TwoPair{ low_pair:pairs[0].clone(), high_pair:pairs[1].clone()};
    }


    // Three of a kind
    if three_of_a_kind.is_some() {
        return Score::ThreeOfAKind(three_of_a_kind.unwrap());
    }

    // Pairs
    if !pairs.is_empty() {
        return Score::Pair(pairs.last().unwrap().clone());
    }

    // Final score, just the highest card.
    Score::HighCard(cards[4].clone())
}

fn score_seq(score:&Score) -> usize {
    match score {
        Score::FullHouse{pair: _, three_of_a_kind: _} => 1,
        Score::Straight{} => 2, 
        Score::FourOfAKind(_) => 20,
        Score::TwoPair{low_pair: _, high_pair: _} => 20,
        Score::ThreeOfAKind(_) => 20,
        Score::Pair(_) => 20,
        Score::HighCard(_) => 10,
    }
}

/**
 * 
 */
pub fn compare_hands(left_score:&Score, right_score:&Score) -> Ordering {
    score_seq(left_score).cmp(&score_seq(right_score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_pair() {
        assert_eq!(score(&PokerHand::from_str("23427").unwrap()), Score::Pair(Card::Two));
    }

    #[test]
    fn score_two_pair() {
        assert_eq!(score(&PokerHand::from_str("3434A").unwrap()), Score::TwoPair{low_pair:Card::Three, high_pair:Card::Four});
    }

    #[test]
    fn score_high_card() {
        assert_eq!(score(&PokerHand::from_str("23457").unwrap()), Score::HighCard(Card::Seven));
    }

    #[test]
    fn score_high_card_other_order() {
        assert_eq!(score(&PokerHand::from_str("75432").unwrap()), Score::HighCard(Card::Seven));
    }
    
    #[test]
    fn score_high_card_ace() {
        assert_eq!(score(&PokerHand::from_str("A235K").unwrap()), Score::HighCard(Card::King));
    }

    #[test]
    fn score_four_of_a_king() {
        assert_eq!(score(&PokerHand::from_str("A2222").unwrap()), Score::FourOfAKind(Card::Two));
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