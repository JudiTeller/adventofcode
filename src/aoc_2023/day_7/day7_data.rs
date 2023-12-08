

// enum for different kind of cards 2 to ace
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // 11
    Queen, // 12
    King, // 13
    Ace, // 14
}

// enum for different kind of cards 2 to ace
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card2 {
    Joker, // 11
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // 12
    King, // 13
    Ace, // 14
}

// struct for a hand with 5 cards
#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub comb: Combination,
    pub bet: i64,
}

impl Hand {
    pub fn new(card_string: &str, bet: i64, version: i64) -> Hand {
        let cards = parse_hand(card_string);

        Hand {
            cards: cards.clone(),
            comb: get_combination(&cards),
            bet,
        }
    }

    // function to compare two hands and return an Ordering
    pub fn compare(&self, other: &Hand) -> std::cmp::Ordering {
        // compare combinations
        let comb_cmp = self.comb.cmp(&other.comb);

        // if combinations are equal, compare cards
        if comb_cmp == std::cmp::Ordering::Equal {
            // compare cards
            for i in 0..self.cards.len() {
                let card_cmp = self.cards[i].cmp(&other.cards[i]);

                if card_cmp != std::cmp::Ordering::Equal {
                    return card_cmp;
                }
            }
        }

        return comb_cmp;
    }
}


// struct for a hand with 5 cards
#[derive(Debug, Clone)]
pub struct Hand2 {
    pub cards: Vec<Card2>,
    pub comb: Combination,
    pub bet: i64,
}

impl Hand2 {
    pub fn new(card_string: &str, bet: i64, version: i64) -> Hand2 {
        let cards = parse_hand2(card_string);

        Hand2 {
            cards: cards.clone(),
            comb: get_combination2(&cards),
            bet,
        }
    }

    // function to compare two hands and return an Ordering
    pub fn compare(&self, other: &Hand2) -> std::cmp::Ordering {
        // compare combinations
        let comb_cmp = self.comb.cmp(&other.comb);

        // if combinations are equal, compare cards
        if comb_cmp == std::cmp::Ordering::Equal {
            // compare cards
            for i in 0..self.cards.len() {
                let card_cmp = self.cards[i].cmp(&other.cards[i]);

                if card_cmp != std::cmp::Ordering::Equal {
                    return card_cmp;
                }
            }
        }

        return comb_cmp;
    }
}



// function to map a string to a card
pub fn map_card(card: &str) -> Card {
    match card {
        "2" => Card::Two,
        "3" => Card::Three,
        "4" => Card::Four,
        "5" => Card::Five,
        "6" => Card::Six,
        "7" => Card::Seven,
        "8" => Card::Eight,
        "9" => Card::Nine,
        "T" => Card::Ten,
        "J" => Card::Jack,
        "Q" => Card::Queen,
        "K" => Card::King,
        "A" => Card::Ace,
        _ => panic!("Unknown card: {}", card),
    }
}

// function to map a string to a card
pub fn map_card_2(card: &str) -> Card2 {
    match card {
        "J" => Card2::Joker,
        "2" => Card2::Two,
        "3" => Card2::Three,
        "4" => Card2::Four,
        "5" => Card2::Five,
        "6" => Card2::Six,
        "7" => Card2::Seven,
        "8" => Card2::Eight,
        "9" => Card2::Nine,
        "T" => Card2::Ten,
        "Q" => Card2::Queen,
        "K" => Card2::King,
        "A" => Card2::Ace,
        _ => panic!("Unknown card: {}", card),
    }
}

// datastruct for the possible combination type
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Combination {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


// function to parse string into Vector of Cards
pub fn parse_hand(hand: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    // iterate over string by chars
    for card in hand.chars() {
        // if char is a number, add it to the current card
        cards.push(map_card(&card.to_string()));
    }

    return cards;
}

// function to parse string into Vector of Cards
pub fn parse_hand2(hand: &str) -> Vec<Card2> {
    let mut cards = Vec::new();

    // iterate over string by chars
    for card in hand.chars() {
        // if char is a number, add it to the current card
        cards.push(map_card_2(&card.to_string()));
    }

    return cards;
}

// function that take &Vec<Card> and return a Combination
pub fn get_combination(cards_original: &Vec<Card>) -> Combination {
    // save original order of cards
    let mut cards = cards_original.clone();

    // sort cards by value
    cards.sort_by(|a, b| a.cmp(&b));

    // check for five of a kind
    if cards[0] == cards[4] {
        return Combination::FiveOfAKind;
    }

    // check for four of a kind
    if cards[0] == cards[3] || cards[1] == cards[4] {
        return Combination::FourOfAKind;
    }

    // check for full house
    if cards[0] == cards[2] && cards[3] == cards[4] {
        return Combination::FullHouse;
    }

    if cards[0] == cards[1] && cards[2] == cards[4] {
        return Combination::FullHouse;
    }

    // check for three of a kind
    if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
        return Combination::ThreeOfAKind;
    }

    // check for two pair
    if cards[0] == cards[1] && cards[2] == cards[3] {
        return Combination::TwoPair;
    }

    if cards[0] == cards[1] && cards[3] == cards[4] {
        return Combination::TwoPair;
    }

    if cards[1] == cards[2] && cards[3] == cards[4] {
        return Combination::TwoPair;
    }

    // check for pair
    if cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4] {
        return Combination::Pair;
    }

    // if no other combination is found, return high card
    return Combination::HighCard;

}


// function that takes Vec<Card2> and returns a Combination
// a Joker is a wild card and can be used as any card
pub fn get_combination2(cards_original: &Vec<Card2>) -> Combination {
    // save original order of cards
    let mut cards = cards_original.clone();

    // sort cards by value
    cards.sort_by(|a, b| a.cmp(&b));

    // check for five of a kind
    if cards[0] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker && cards[2] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker && cards[2] == Card2::Joker && cards[3] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker && cards[2] == Card2::Joker && cards[3] == Card2::Joker{
        return Combination::FiveOfAKind;
    }

    // check for four of a kind
    if cards[0] == cards[3] || cards[1] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == cards[3]
        || cards[0] == Card2::Joker && cards[2] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker && (cards[2] == cards[3] || cards[3] == cards[4])
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker && cards[2] == Card2::Joker
{
        return Combination::FourOfAKind;
    }

    // check for full house
    if cards[0] == cards[2] && cards[3] == cards[4] || cards[0] == cards[1] && cards[2] == cards[4]
        || cards[0] == Card2::Joker && cards[1] == cards[2] && cards[3] == cards[4] {
        return Combination::FullHouse;
    }


    // check for three of a kind
    if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4]
        || cards[0] == Card2::Joker && (cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4])
        || cards[0] == Card2::Joker && cards[1] == Card2::Joker {
        return Combination::ThreeOfAKind;
    }

    // check for two pair
    if cards[0] == cards[1] && cards[2] == cards[3]
        || cards[0] == cards[1] && cards[3] == cards[4]
        || cards[1] == cards[2] && cards[3] == cards[4] {
        return Combination::TwoPair;
    }

    // check for pair
    if cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4]
        || cards[0] == Card2::Joker {
        return Combination::Pair;
    }

    // if no other combination is found, return high card
    return Combination::HighCard;

}


