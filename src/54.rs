/* Problem 54: Poker hands
 *
 * In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in
 * the following way:
 *
 *  High Card: Highest value card.
 *  One Pair: Two cards of the same value.
 *  Two Pairs: Two different pairs.
 *  Three of a Kind: Three cards of the same value.
 *  Straight: All cards are consecutive values.
 *  Flush: All cards of the same suit.
 *  Full House: Three of a kind and a pair.
 *  Four of a Kind: Four cards of the same value.
 *  Straight Flush: All cards are consecutive values of same suit.
 *  Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
 *
 * The cards are valued in the order:
 * 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
 *
 * If two players have the same ranked hands then the rank made up of the highest value wins; for
 * example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for
 * example, both players have a pair of queens, then highest cards in each hand are compared (see
 * example 4 below); if the highest cards tie then the next highest cards are compared, and so on.
 *
 * Consider the following five hands dealt to two players:
 *
 * | Hand | Player 1i                                           |  Player 2                                             | Winner
 * | 1    | 5H 5C 6S 7S KD (Pair of Fives)                      |  2C 3S 8S 8D TD (Pair of Eights)                      | Player 2
 * | 2    | 5D 8C 9S JS AC (Highest card Ace)                   |  2C 5C 7D 8S QH (Highest card Queen)                  | Player 1
 * | 3    | 2D 9C AS AH AC (Three Aces)                         |  3D 6D 7D TD QD (Flush with Diamonds)                 | Player 2
 * | 4    | 4D 6S 9H QH QC (Pair of Queens - Highest card Nine) |  3D 6D 7H QD QS (Pair of Queens - Highest card Seven) | Player 1
 * | 5    | 2H 2D 4C 4D 4S (Full House - With Three Fours)      |  3C 3D 3S 9S 9D (Full House - with Three Threes)      | Player 1
 *
 * The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the
 * file contains ten cards (separated by a single space): the first five are Player 1's cards and
 * the last five are Player 2's cards. You can assume that all hands are valid (no invalid
 * characters or repeated cards), each player's hand is in no specific order, and in each hand there
 * is a clear winner.
 *
 * How many hands does Player 1 win? */

#![feature(macro_rules)]

#[deriving(PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
  HighCard(CardValue),              // High Card: Highest value card.
  OnePair(CardValue),               // One Pair: Two cards of the same value.
  TwoPairs(CardValue, CardValue),   // Two Pairs: Two different pairs.
  ThreeOfAKind(CardValue),          // Three of a Kind: Three cards of the same value.
  Straight,                         // Straight: All cards are consecutive values.
  Flush,                            // Flush: All cards of the same suit.
  FullHouse(CardValue, CardValue),  // Full House: Three of a kind and a pair.
  FourOfAKind(CardValue),           // Four of a Kind: Four cards of the same value.
  StraightFlush,                    // Straight Flush: All cards are consecutive values of same suit.
  RoyalFlush,                       // Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
}

#[deriving(PartialEq, Eq, PartialOrd, Ord, Clone, FromPrimitive, Hash)]
enum CardValue {
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
  Ace
}

impl CardValue {
  fn parse(value: &str) -> Option<CardValue> {
    match value {
     "2" => Some(Two),
     "3" => Some(Three),
     "4" => Some(Four),
     "5" => Some(Five),
     "6" => Some(Six),
     "7" => Some(Seven),
     "8" => Some(Eight),
     "9" => Some(Nine),
     "T" => Some(Ten),
     "J" => Some(Jack),
     "Q" => Some(Queen),
     "K" => Some(King),
     "A" => Some(Ace),
     _   => None,
    }
  }
}

#[deriving(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum CardSuit {
  Spades,
  Hearts,
  Diamonds,
  Clubs,
}

impl CardSuit {
  fn parse(suit: &str) -> Option<CardSuit> {
    match suit {
      "H" => Some(Hearts),
      "S" => Some(Spades),
      "C" => Some(Clubs),
      "D" => Some(Diamonds),
      _   => None
    }
  }
}

#[deriving(PartialEq, Eq, Clone)]
struct Card {
  value: CardValue,
  suit: CardSuit,
}

impl Card {
  fn parse(s: &str) -> Option<Card> {
    let value = s.slice_to(1);
    let suit  = s.slice_from(1);

    match (CardValue::parse(value), CardSuit::parse(suit)) {
      (Some(card_value), Some(card_suit)) => {
        Some(Card { value: card_value, suit: card_suit })
      },

      _ => None,
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
    self.value.partial_cmp(& other.value)
  }
}

impl Ord for Card {
  fn cmp(&self, other: &Card) -> Ordering {
    self.value.cmp(& other.value)
  }
}

struct Hand {
  cards: [Card, ..5]
}

impl Hand {
  fn beats(&self, other: &Hand) -> bool {
    match self.rank().cmp(&other.rank()) {
      Less    => false,
      Greater => true,

      Equal => {
        let my_cards    = self.cards.as_slice();
        let other_cards = other.cards.as_slice();

        for (my, hers) in my_cards.iter().rev().zip(other_cards.iter().rev()) {
          match my.cmp(hers) {
            Less    => return false,
            Greater => return true,
            Equal   => continue,
          }
        }

        unreachable!()
      }
    }
  }

  fn parse(s: &str) -> Option<Hand> {
    let parse_cards = s.split(' ')
      .map(Card::parse)
      .collect::<Option<Vec<Card>>>();

    return parse_cards.and_then(|mut cards| {
      if cards.len() != 5 { return None }

      cards.sort();

      Some(Hand {
        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]]
      })
    });
  }
}

impl Hand {
  fn rank(&self) -> Rank {
    static ROYAL_FLUSH: &'static [CardValue] = &[Ten, Jack, Queen, King, Ace];

    let cards_in_order = self.cards.as_slice();

    let values_in_order: Vec<CardValue> = cards_in_order.iter().map(|card| {
      card.value
    }).collect();

    let consecutives = self.consecutive_card_count(cards_in_order.as_slice());

    let (suits, values) = self.card_counts();

    let all_same_suit   = suits[0].val1() == 5;
    let all_consecutive = consecutives == 5;

    if all_same_suit && all_consecutive {
      if values_in_order.as_slice() == ROYAL_FLUSH {
        return RoyalFlush;
      } else {
        return StraightFlush;
      }
    }

    let (first_value,  first_value_count)  = values[0];
    let (second_value, second_value_count) = values[1];

    match (first_value_count, second_value_count) {
      (4, _) => return FourOfAKind(first_value),
      (3, 2) => return FullHouse(first_value, second_value),
      _      => (),
    };

    if all_same_suit {
      return Flush
    } else if all_consecutive {
      return Straight;
    }

    return match (first_value_count, second_value_count) {
      (3, _) => ThreeOfAKind(first_value),
      (2, 2) => TwoPairs(first_value, second_value),
      (2, _) => OnePair(first_value),
      _      => HighCard(* values_in_order.last().unwrap()),
    };
  }

  fn consecutive_card_count(&self, in_order: &[Card]) -> uint {
    let mut max_consecutive = 0u;
    let mut current_consecutives: Option<uint> = None;

    for window in in_order.windows(2) {
      let value1 = window[0].value;
      let value2 = window[1].value;

      let consecutive_value = FromPrimitive::from_int((value1 as int) + 1);

      if consecutive_value != Some(value2) {
        current_consecutives = None;
        continue;
      }

      let new_consecutives = match current_consecutives {
        Some(value) => value + 1,
        None        => 2,
      };

      if new_consecutives > max_consecutive {
        max_consecutive = new_consecutives;
      }

      current_consecutives = Some(new_consecutives);
    }

    max_consecutive
  }

  fn card_counts(&self) -> (Vec<(CardSuit, uint)>, Vec<(CardValue, uint)>) {
    use std::collections::hashmap::{HashMap, Occupied, Vacant};
    use std::hash::Hash;

    let mut suit_map:  HashMap<CardSuit, uint>  = HashMap::with_capacity(5);
    let mut value_map: HashMap<CardValue, uint> = HashMap::with_capacity(5);

    for &Card { suit: suit, value: value} in self.cards.iter() {
      increment_by_one(&mut suit_map,  suit);
      increment_by_one(&mut value_map, value);
    }

    return (into_sorted_vec(suit_map), into_sorted_vec(value_map));

    fn increment_by_one<T: Eq + Hash>(map: &mut HashMap<T, uint>, key: T) {
      match map.entry(key) {
        Occupied(entry) => {
          *entry.into_mut() += 1;
        },

        Vacant(entry) => {
          entry.set(1);
        }
      }
    }

    fn into_sorted_vec<T: Eq + Hash + Ord>(map: HashMap<T, uint>) -> Vec<(T, uint)> {
      let mut vec =  map.into_iter().collect::<Vec<(T, uint)>>();
      vec.sort_by(|&(ref k1, ref v1), &(ref k2, ref v2)| {
        match v2.cmp(v1) {
          Equal => k2.cmp(k1),
          ord   => ord
        }
      });

      return vec;
    }
  }
}

#[cfg(not(test))]
fn main() {
  static HANDS: &'static str = include_str!("../data/54-poker.txt");

  let player_1_victories = HANDS.lines().filter(|line| {
    let maybe_hand1 = Hand::parse(line.slice_to(14));
    let maybe_hand2 = Hand::parse(line.slice_from(15));

    match (maybe_hand1, maybe_hand2) {
      (Some(hand1), Some(ref hand2)) => hand1.beats(hand2),

      _ => fail!("Parsing problem on line:\n{}", line)
    }
  }).count();

  println!("{}", player_1_victories);
}

#[cfg(test)]
mod tests {
  use super::{Rank, Hand};

  fn hand(s: &str) -> Hand {
    Hand::parse(s).unwrap()
  }

  fn rank(s: &str) -> Rank {
    hand(s).rank()
  }

  #[test]
  fn test_royal_flush() {
    use super::RoyalFlush;

    assert_eq!(RoyalFlush, rank("TH JH QH KH AH"));
    assert_eq!(RoyalFlush, rank("AH JH TH QH KH"));
  }

  #[test]
  fn test_straight_flush() {
    use super::StraightFlush;

    assert_eq!(StraightFlush, rank("5H 6H 7H 8H 9H"));
    assert_eq!(StraightFlush, rank("9H TH JH QH KH"));
    assert_eq!(StraightFlush, rank("7C 8C 9C TC JC"));
  }

  #[test]
  fn test_four_of_a_kind() {
    use super::{Five, Nine, FourOfAKind};

    assert_eq!(FourOfAKind(Five), rank("5H 5C 5D 5S 9H"));
    assert_eq!(FourOfAKind(Nine), rank("9H 9D KH 9C 9S"));
  }

  #[test]
  fn test_full_house() {
    use super::{FullHouse, Five, Nine, Four, Two, Three};

    assert_eq!(FullHouse(Five, Nine),  rank("5H 5C 5D 9S 9H"));
    assert_eq!(FullHouse(Nine, Five),  rank("9H 9D 5H 5C 9S"));
    assert_eq!(FullHouse(Four, Two),   rank("2H 2D 4C 4D 4S"));
    assert_eq!(FullHouse(Three, Nine), rank("3C 3D 3S 9S 9D"));
  }

  #[test]
  fn test_flush() {
    use super::Flush;

    assert_eq!(Flush, rank("3D 6D 7D TD QD"));
    assert_eq!(Flush, rank("9C TC 5C 2C AC"));
    assert_eq!(Flush, rank("3D 6D 7D TD QD"));
  }

  #[test]
  fn test_straight() {
    use super::Straight;

    assert_eq!(Straight, rank("3D 4D 5H 6S 7D"));
    assert_eq!(Straight, rank("2C 3S 4H 5C 6H"));
  }

  #[test]
  fn test_three_of_a_kind() {
    use super::{ThreeOfAKind, Nine, Five, Ace};

    assert_eq!(ThreeOfAKind(Five), rank("5H 5C 5D 9S AH"));
    assert_eq!(ThreeOfAKind(Nine), rank("9H 2D 5H 9C 9S"));
    assert_eq!(ThreeOfAKind(Ace),  rank("2D 9C AS AH AC"));
  }

  #[test]
  fn test_two_pairs() {
    use super::{TwoPairs, Five, Nine};

    assert_eq!(TwoPairs(Nine, Five), rank("5H AC 5D 9S 9H"));
    assert_eq!(TwoPairs(Nine, Five), rank("9H 2D 5H 5C 9S"));
  }

  #[test]
  fn test_one_pair() {
    use super::{OnePair, Five, Nine, Eight, Queen};

    assert_eq!(OnePair(Five),  rank("2H AC 5D 9S 5H"));
    assert_eq!(OnePair(Nine),  rank("AH 2D 9H 5C 9S"));
    assert_eq!(OnePair(Five),  rank("5H 5C 6S 7S KD"));
    assert_eq!(OnePair(Eight), rank("2C 3S 8S 8D TD"));
    assert_eq!(OnePair(Queen), rank("4D 6S 9H QH QC"));
    assert_eq!(OnePair(Queen), rank("3D 6D 7H QD QS"));
  }

  #[test]
  fn test_high_card() {
    use super::{HighCard, Queen, King, Ace};

    assert_eq!(HighCard(Queen), rank("2H QC 5D 9S TH"));
    assert_eq!(HighCard(King),  rank("KH 2D 9H 5C 3S"));
    assert_eq!(HighCard(Ace),   rank("5D 8C 9S JS AC"));
    assert_eq!(HighCard(Queen), rank("2C 5C 7D 8S QH"));
  }

  #[test]
  fn test_rank_beats() {
    use super::{RoyalFlush, StraightFlush, FourOfAKind, FullHouse};
    use super::{King, Queen, Ace};

    assert!(RoyalFlush            > StraightFlush);
    assert!(StraightFlush         > FourOfAKind(Ace));
    assert!(FourOfAKind(Ace)      > FourOfAKind(King));
    assert!(FullHouse(Queen, Ace) > FullHouse(Queen, King));
  }

  fn beats(hand1_str: &str, hand2_str: &str) -> bool {
    let hand1 = hand(hand1_str);
    let hand2 = hand(hand2_str);

    hand1.beats(& hand2)
  }

  #[test]
  fn test_hand_beats() {
    assert_eq!(beats("5H 5C 6H 6C QH", "5S 5D 6D 6S KH"), false);
    assert_eq!(beats("4D 6S 9H QH QC", "3C 6H 9C QD QS"), true);
    assert_eq!(beats("6D 7C 5D 5H 3S", "5C JC 2H 5S 3D"), false);

    // from the problem statement:
    assert_eq!(beats("5H 5C 6S 7S KD", "2C 3S 8S 8D TD"), false);
    assert_eq!(beats("5D 8C 9S JS AC", "2C 5C 7D 8S QH"), true);
    assert_eq!(beats("2D 9C AS AH AC", "3D 6D 7D TD QD"), false);
    assert_eq!(beats("4D 6S 9H QH QC", "3D 6D 7H QD QS"), true);
    assert_eq!(beats("2H 2D 4C 4D 4S", "3C 3D 3S 9S 9D"), true);
  }
}
