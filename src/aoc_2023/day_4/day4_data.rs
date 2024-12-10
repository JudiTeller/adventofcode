use std::collections::HashMap;

pub struct Card {
    pub _card_id: i64,
    pub _winning_numbers: Vec<i64>,
    pub _guesses: Vec<i64>,
    pub _points: i64,
    pub matches: i64,
    pub amount: i64,
}

// struct cardgame containing a hashmap of cards and their amount
// implements a function to add cards

pub struct CardGame {
    pub cards: HashMap<i64, Card>,
}

impl CardGame {
    pub fn new() -> CardGame {
        CardGame {
            cards: HashMap::new(),
        }
    }

    pub fn add_card(
        &mut self,
        card_id: i64,
        winning_numbers: Vec<i64>,
        guesses: Vec<i64>,
        points: i64,
        matches: i64,
    ) {
        let card = Card {
            _card_id: card_id,
            _winning_numbers: winning_numbers,
            _guesses: guesses,
            _points: points,
            matches,
            amount: 1,
        };
        self.cards.insert(card_id, card);
    }
}
