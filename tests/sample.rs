use poker::PokerHand;
use poker::Score;
use poker::score;
use poker::cards::Card;
use poker::compare_hands;

use std::cmp::Ordering;

    #[test]
    fn test_sample1() {
        let hand1 = &PokerHand::from_str("AAKKK").unwrap();
        let hand2 = &PokerHand::from_str("23456").unwrap();

        assert_eq!(score(hand1), Score::FullHouse{pair:Card::Ace, three_of_a_kind:Card::King});
        assert_eq!(score(hand2), Score::Straight);
        assert_eq!(compare_hands(&score(hand1), &score(hand2)), Ordering::Less);
    }

    #[test]
    fn test_sample2() {
        let hand1 = &PokerHand::from_str("KA225").unwrap();
        let hand2 = &PokerHand::from_str("33A47").unwrap();
        
        assert_eq!(score(hand1), Score::Pair(Card::Two));
        assert_eq!(score(hand2), Score::Pair(Card::Three));
        // TODO assert_eq!(compare_hands(hand1, hand2), Ordering::Greater);
    }

    #[test]
    fn test_sample3() {
        let hand1 = &PokerHand::from_str("AA225").unwrap();
        let hand2 = &PokerHand::from_str("44465").unwrap();

        assert_eq!(score(hand1), Score::TwoPair{ low_pair:Card::Ace, high_pair:Card::Two});
        assert_eq!(score(hand2), Score::ThreeOfAKind(Card::Four));
        // TODO assert_eq!(compare_hands(hand1, hand2), Ordering::Greater);
    }

    #[test]
    fn test_sample4() {
        let hand1 = &PokerHand::from_str("TT8A9").unwrap();
        let hand1_score = score(hand1);

        let hand2 = &PokerHand::from_str("TTA89").unwrap();
        let hand2_score = score(hand2);

        assert_eq!(hand1_score, Score::Pair(Card::Ten));
        assert_eq!(hand2_score, Score::Pair(Card::Ten));
        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Equal);
    }

    #[test]
    fn test_sample5() {
        let hand1 = &PokerHand::from_str("A2345").unwrap();
        let hand2 = &PokerHand::from_str("23456").unwrap();

        assert_eq!(score(hand1), Score::Straight);
        assert_eq!(score(hand2), Score::Straight);
        // TODO assert_eq!(compare_hands(hand1, hand2), Ordering::Greater);
    }

    #[test]
    fn test_sample6() {
        let hand1 = &PokerHand::from_str("QQ2AT").unwrap();
        let hand2 = &PokerHand::from_str("QQT2J").unwrap();
        
        assert_eq!(score(hand1), Score::Pair(Card::Queen));
        assert_eq!(score(hand2), Score::Pair(Card::Queen));
        // TODO assert_eq!(compare_hands(hand1, hand2), Ordering::Less);// i wonder why
    }