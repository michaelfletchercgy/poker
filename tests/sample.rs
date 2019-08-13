use poker::PokerHand;
use poker::Score;
use poker::cards::Card;
use poker::compare_hands;

use std::cmp::Ordering;

    #[test]
    fn test_sample1() {
        let hand1 = &PokerHand::from_str("AAKKK").unwrap();
        let hand2 = &PokerHand::from_str("23456").unwrap();

        assert_eq!(hand1.score(), Score::FullHouse{pair:Card::Ace, three_of_a_kind:Card::King});
        assert_eq!(hand2.score(), Score::Straight(Card::Six));
        assert_eq!(compare_hands(&hand1.score(), &hand2.score()), Ordering::Less);
    }

    #[test]
    fn test_sample2() {
        let hand1 = &PokerHand::from_str("KA225").unwrap();
        let hand1_score = hand1.score();
        assert_eq!(hand1_score, Score::Pair{card:Card::Two, kickers:[Card::Five, Card::King, Card::Ace]});
        
        let hand2 = &PokerHand::from_str("33A47").unwrap();
        let hand2_score = hand2.score();
        assert_eq!(hand2_score, Score::Pair{card:Card::Three, kickers:[Card::Four, Card::Seven, Card::Ace]});
        
        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Greater);
    }

    #[test]
    fn test_sample3() {
        let hand1 = &PokerHand::from_str("AA225").unwrap();
        let hand1_score = hand1.score();
        assert_eq!(hand1_score, Score::TwoPair{ low_pair:Card::Two, high_pair:Card::Ace, kicker:Card::Five});
        
        let hand2 = &PokerHand::from_str("44465").unwrap();
        let hand2_score = hand2.score();
        assert_eq!(hand2_score, Score::ThreeOfAKind{card:Card::Four, high_kicker:Card::Six, low_kicker:Card::Five});

        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Greater);
    }

    #[test]
    fn test_sample4() {
        let hand1 = &PokerHand::from_str("TT8A9").unwrap();
        let hand1_score = hand1.score();
        assert_eq!(hand1_score, Score::Pair{card:Card::Ten, kickers:[Card::Eight, Card::Nine, Card::Ace]});

        let hand2 = &PokerHand::from_str("TTA89").unwrap();
        let hand2_score = hand2.score();
        assert_eq!(hand2_score, Score::Pair{card:Card::Ten, kickers:[Card::Eight, Card::Nine, Card::Ace]});

        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Equal);
    }

    #[test]
    fn test_sample5() {
        let hand1 = &PokerHand::from_str("A2345").unwrap();
        let hand1_score = hand1.score();
        let hand2 = &PokerHand::from_str("23456").unwrap();
        let hand2_score = hand2.score();

        assert_eq!(hand1_score, Score::Straight(Card::Five));
        assert_eq!(hand2_score, Score::Straight(Card::Six));
        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Greater);
    }

    #[test]
    fn test_sample6() {
        let hand1 = &PokerHand::from_str("QQ2AT").unwrap();
        let hand1_score = hand1.score();
        assert_eq!(hand1_score, Score::Pair{card:Card::Queen, kickers:[Card::Two, Card::Ten, Card::Ace]});

        let hand2 = &PokerHand::from_str("QQT2J").unwrap();
        let hand2_score = hand2.score();
        assert_eq!(hand2_score, Score::Pair{card:Card::Queen, kickers:[Card::Two, Card::Ten, Card::Jack]});
        
        assert_eq!(compare_hands(&hand1_score, &hand2_score), Ordering::Less);
    }