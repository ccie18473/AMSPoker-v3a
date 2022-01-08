use crate::prelude::*;

pub struct TDeck {
    pub cards: Vec<TCard>,
}
impl TDeck {
    pub fn new_sorted_deck() -> Self {
        let mut deck = TDeck { cards: Vec::new() };
        let mut index = 0;
        for suit in 0..SUIT_SPADES + 1 {
            for value in 0..VALUE_KING + 1 {
                deck.cards
                    .push(TCard::new_card(value, suit, false, false, index));
                index += 1;
            }
        }
        deck
    }
    pub fn shuffle(&mut self) {
        let mut seed = RandomNumberGenerator::new();
        for n in 0..self.cards.len() {
            let m = seed.range(0, self.cards.len());
            if m != n {
                self.cards.swap(m, n);
            }
        }
    }
    pub fn new_shuffled_deck() -> Self {
        let mut deck = TDeck::new_sorted_deck();
        TDeck::shuffle(&mut deck);
        deck
    }
    pub fn pop(&mut self) -> Option<TCard> {
        self.cards.pop()
    }
    pub fn new_empty_hand() -> Self {
        let hand = TDeck { cards: Vec::new() };
        hand
    }
}
