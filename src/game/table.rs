#![allow(dead_code)]

use super::card::{self, Monad, Card, Deck};
use std::iter::repeat_with;

pub struct Table {
    pub discard: Deck,
    pub common:  Deck,
    pub bi:      Deck,
    pub tri:     Deck,
    pub quad:    Deck,
    pub quint:   Deck,
    pub monad:   Vec<Monad>,
}

impl Table {
    pub fn new(players: usize) -> Self {
        let mut table = Table {
            discard: Deck::multiple(players),
            common:  Deck::multiple(players),
            bi:      Deck::multiple(1),
            tri:     Deck::multiple(1),
            quad:    Deck::multiple(1),
            quint:   Deck::multiple(1),
            monad:   repeat_with(|| Monad).take(12).collect(),
        };

        for color in &card::COLORS {
            use self::card::Value::*;

            table.common.extend(
                repeat_with(|| Card(Common, *color)).take(players)
            );
            table.bi   .push(Card(Bi   , *color));
            table.tri  .push(Card(Tri  , *color));
            table.quad .push(Card(Quad , *color));
            table.quint.push(Card(Quint, *color));
        }

        table.shuffle_decks();

        table
    }

    fn shuffle_decks(&mut self) {
        self.bi    .shuffle();
        self.tri   .shuffle();
        self.quad  .shuffle();
        self.quint .shuffle();
        self.common.shuffle();
    }
}
