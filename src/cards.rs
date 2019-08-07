// TODO Module level docs
//
// Various functions and structures related to 52 deck cards.  This is generally
// useful and not specific to any particular card game.
//
// You would find ordering, comparing cards and ranking in a module for a more 
// particular card game.
//
// TODO this breakdown didn't really work because a a five card hand is a particularily
// of poker.


#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum Card {
    Ace,
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
    King
}

impl Card {
    pub fn from_char(c:&char) -> Result<Card, String> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(format!("Character '{}' is not valid.", c))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_from_char_err() {
        assert_eq!(Card::from_char(&'$'), Result::Err(String::from("Character '$' is not valid.")));
    }

    #[test]
    fn card_from_char_ok() {
        let rules = [
            (&'2', Card::Two),
            (&'3', Card::Three),
            (&'4', Card::Four),
            (&'5', Card::Five),
            (&'6', Card::Six),
            (&'7', Card::Seven),
            (&'8', Card::Eight),
            (&'9', Card::Nine),
            (&'T', Card::Ten),
            (&'J', Card::Jack),
            (&'Q', Card::Queen),
            (&'K', Card::King),
            (&'A', Card::Ace),

        ];

        for (character, card) in rules.iter() {
            assert_eq!(Card::from_char(character), Result::Ok(card.clone()));
        }
    }
}