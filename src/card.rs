use crate::prelude::*;

type Suit = usize;
type SuitColor = usize;

pub const SUIT_CLUBS: Suit = 0;
//pub const SUIT_DIAMONDS: Suit = 1;
//pub const SUIT_HEARTS: Suit = 2;
pub const SUIT_SPADES: Suit = 3;

pub const SUIT_COLOR_BLACK: SuitColor = 0;
pub const SUIT_COLOR_RED: SuitColor = 1;

//pub const VALUE_JACK: usize = 10;
//pub const VALUE_QUEEN: usize = 11;
pub const VALUE_KING: usize = 12;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TCard {
    pub value: usize,
    pub suit: Suit,
    pub face_up: bool,
    pub selected: bool,
    pub index: usize,
}

impl TCard {
    pub fn new_card(value: usize, suit: Suit, face_up: bool, selected: bool, index: usize) -> Self {
        Self {
            value,
            suit,
            face_up,
            selected,
            index,
        }
    }
    pub fn card_selected(&mut self) {
        self.flip_card()
    }
    pub fn flip_card(&mut self) {
        self.face_up = !self.face_up;
        self.selected = !self.selected;
        if self.selected {
            self.index = VALUE_RUST;
        } else {
            self.index = self.value + self.suit * 13;
        }
    }
    pub fn color(&self) -> usize {
        if self.suit == SUIT_CLUBS || self.suit == SUIT_SPADES {
            return SUIT_COLOR_BLACK;
        }
        SUIT_COLOR_RED
    }
}
