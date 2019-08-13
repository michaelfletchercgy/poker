
use poker::PokerHand;
use poker::Score;

use std::io;
use std::io::stdin;
use std::io::BufRead;

use std::cmp::Ordering;

#[derive(Debug)]
enum ParseError {
    IoError(io::Error),
    LineParse(std::num::ParseIntError),
    FewLines,
    MissingLeftHand,
    MissingRightHand,
    LeftHandInvalid(usize, String),
    RightHandInvalid(usize, String)
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::IoError(error)
    }
}

fn parse(reader:&mut dyn BufRead) -> Result<Vec<(PokerHand, PokerHand)>, ParseError> {
    let mut buf = String::new();

    reader.read_line(&mut buf)?;
    let lines = match buf.trim().parse::<usize>() {
        Ok(number) => number,
        Err(err) => return Err(ParseError::LineParse(err))
    };

    let mut result = Vec::new();
    for line_idx in 0..lines {
        buf.clear();

        let bytes_read = reader.read_line(&mut buf)?;
        if bytes_read == 0 {
            return Err(ParseError::FewLines)
        }

        let mut parts = buf.split_ascii_whitespace();
        let left_hand_str = match parts.next() {
            Some(x) => x,
            None => return Err(ParseError::MissingLeftHand)
        };

        let right_hand_str = match parts.next() {
            Some(x) => x,
            None => return Err(ParseError::MissingRightHand)
        };
        
        let left_hand = match PokerHand::from_str(left_hand_str) {
            Ok(x) => x,
            Err(err) => return Err(ParseError::LeftHandInvalid(line_idx + 1, err))
        };

        let right_hand = match PokerHand::from_str(right_hand_str) {
            Ok(x) => x,
            Err(err) => return Err(ParseError::RightHandInvalid(line_idx + 1, err))
        };

        result.push((left_hand, right_hand));
    }
    
    Ok(result)
}


fn print_score(score:&Score) {
    match score {
        Score::FullHouse{pair: _, three_of_a_kind: _} => print!("FULLHOUSE"),
        Score::Straight(_) => print!("STRAIGHT"), 
        Score::FourOfAKind{..} => print!("FOUROFAKIND"),
        Score::TwoPair{..} => print!("TWOPAIR"),
        Score::ThreeOfAKind{..} => print!("THREEOFAKIND"),
        Score::Pair{..} => print!("PAIR"),
        Score::HighCard{..} => print!("HIGHCARD"),
    }
}

fn print_ordering(ordering:Ordering) {
    match ordering {
        Ordering::Less => { print!("a"); },
        Ordering::Equal => { print!("ab"); },
        Ordering::Greater => { print!("b") }
    }
}

fn main() {
    let hands = match parse(&mut stdin().lock()) {
        Ok(x) => x,
        Err(err) => { 
            match err {
                ParseError::IoError(io_err) => eprintln!("io error: {}", io_err),
                ParseError::LineParse(parse_err) => eprintln!("The first line of input should be an number and it was not: {}", parse_err),
                ParseError::FewLines => eprintln!("Missing lines"),
                ParseError::MissingLeftHand => eprintln!("Left hand missing."),
                ParseError::LeftHandInvalid(l, msg) => eprintln!("The left hand of line {} is invalid: {}", l, msg),
                ParseError::MissingRightHand => eprintln!("Right hand missing."),
                ParseError::RightHandInvalid(l, msg) => eprintln!("The right hand of line {} is invalid: {}", l, msg)
            }

            return;
        }
    };

    for (left, right) in hands {
        let left_score = left.score();
        let right_score = right.score();
        let ordering = poker::compare_hands(&left_score, &right_score);
        
        print_score(&left_score);
        print!(" ");
        print_score(&right_score);
        print!(" ");
        print_ordering(ordering);
        println!("");

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker::cards::Card;
    use std::io::BufReader;

    #[test]
    fn test_parse() {
        let example = "1\n23456 34567\n".as_bytes();
        let mut buf = BufReader::new(example);
        let result = parse(&mut buf).unwrap();
        assert_eq!(result.len(), 1);

        let (left, right) = &result[0];
        assert_eq!(left.cards(), &[Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]);
        assert_eq!(right.cards(), &[Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven]);
    }

}