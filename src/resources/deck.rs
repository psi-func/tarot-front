use rand::{seq::SliceRandom, thread_rng};

use super::card::CardId;

const NUM_CARDS: u8 = 78;

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<CardId>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(NUM_CARDS as usize);
        for id in 1..=NUM_CARDS {
            cards.push(CardId::try_from(id).unwrap());
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Deck { cards }
    }
}

impl Deck {
    pub fn get_cards(&mut self, num_cards: usize) -> Vec<CardId> {
        self.cards.drain(0..num_cards).collect()
    }
}
