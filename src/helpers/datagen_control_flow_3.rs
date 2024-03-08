use std::io::Write;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::WeightedAliasIndex;

mod datagen;

#[derive(Clone, Copy)]
enum Card {
    A2,
    A4,
    N(usize),
}

struct Model {
    cards: Vec<Card>,
}

impl Model {
    pub fn new(cards: impl IntoIterator<Item = Card>) -> Self {
        Self {
            cards: cards.into_iter().collect(),
        }
    }
}

impl datagen::Model for Model {
    fn arbitrary(rng: &mut impl rand::Rng) -> Self {
        let bound = rng.gen_range(2..=5);
        let mut cards = Vec::new();
        let mut current = [0; 10];

        let mut deck = Deck::new();

        const PROBS: [f64; 10] = [
            0.1725, 0.0863, 0.0575, 0.0431, 0.0345, 0.0287, 0.0246, 0.0216, 0.0192, 0.0173,
        ];
        let distr = WeightedAliasIndex::new(PROBS.to_vec()).unwrap();
        const ALPHA: f64 = 0.05;
        const BETA: f64 = ALPHA / 2.0;
        let p: f64 = PROBS.iter().sum();
        let q: f64 = 1.0 - p - ALPHA - BETA;

        let mut sum: i32 = current.iter().sum();
        while sum != 1 {
            match rng.sample(&WeightedAliasIndex::new(vec![ALPHA, BETA, p, q]).unwrap()) {
                0 => {
                    cards.push(Card::A2);
                    for _ in 0..2 {
                        let card = deck.draw();
                        current[card] += 1;
                    }
                }
                1 => {
                    cards.push(Card::A4);
                    for _ in 0..4 {
                        let card = deck.draw();
                        current[card] += 1;
                    }
                }
                2 => {
                    let step = rng.sample(&distr);
                    if (sum + step as i32) < bound {
                        continue;
                    }
                    let card = deck.find(step);
                    cards.push(Card::N(card));
                    while current[card] == 0 {
                        let card = deck.draw();
                        current[card] += 1;
                    }
                    current[card] -= 1;
                }
                3 => {
                    if sum < bound {
                        continue;
                    }
                    let mut card = rng.gen_range(0..10);
                    while current[card] == 0 {
                        card = rng.gen_range(0..10);
                    }
                    cards.push(Card::N(card));
                    current[card] -= 1;
                }
                _ => unreachable!(),
            }
            sum = current.iter().sum();
        }

        Self { cards }
    }

    fn write_input(&self, w: &mut impl Write) -> std::io::Result<()> {
        for card in &self.cards {
            match card {
                Card::A2 => writeln!(w, "+2")?,
                Card::A4 => writeln!(w, "+4")?,
                Card::N(n) => writeln!(w, "{}", n)?,
            }
        }
        Ok(())
    }

    fn write_ans(&self, w: &mut impl Write) -> std::io::Result<()> {
        let mut deck = Deck::new();
        let mut draw_card = || deck.draw();

        let mut total = 0;

        let mut cards = [0; 13];
        for &card in &self.cards {
            match card {
                Card::A2 => {
                    for _ in 0..2 {
                        cards[draw_card()] += 1;
                    }
                    total += 2;
                }
                Card::A4 => {
                    for _ in 0..4 {
                        cards[draw_card()] += 1;
                    }
                    total += 4;
                }
                Card::N(n) => {
                    while cards[n] == 0 {
                        cards[draw_card()] += 1;
                        total += 1;
                    }
                    cards[n] -= 1;
                }
            }

            let mut sum = 0;
            for c in cards.iter() {
                sum += c;
            }
            if sum == 1 {
                writeln!(w, "UNO!")?;
                break;
            }
        }
        writeln!(w, "{}", total)
    }
}

struct Deck {
    cursor: usize,
}

impl Deck {
    const MAGIC: [usize; 10] = [3, 6, 9, 2, 5, 8, 1, 4, 7, 0];

    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn draw(&mut self) -> usize {
        let card = Self::MAGIC[self.cursor];
        self.cursor = (self.cursor + 1) % 10;
        card
    }

    pub fn find(&self, c: usize) -> usize {
        let idx = Self::MAGIC.iter().position(|&x| x == c).unwrap();
        (idx + 10 - self.cursor) % 10
    }
}

fn main() -> std::io::Result<()> {
    datagen::Datagen::new("./fixtures/control_flow_3")
        .edge_cases(vec![
            Model::new([Card::N(6)]),
            Model::new([Card::A2, Card::N(2), Card::N(3), Card::N(6)]),
        ])
        .sample_cases(&mut ChaCha8Rng::seed_from_u64(42), 25)
        .generate()
}
