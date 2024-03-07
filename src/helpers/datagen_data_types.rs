use std::io::{Result, Write};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod datagen;

struct Model {
    x: i32,
    y: f64,
}

impl datagen::Model for Model {
    fn arbitrary(rng: &mut impl rand::Rng) -> Self {
        Model {
            x: rng.gen(),
            y: rng.gen_range(-1_000_000_000.0..1_000_000_000.0),
        }
    }

    fn write_input(&self, w: &mut impl Write) -> Result<()> {
        writeln!(w, "{}", self.x)?;
        writeln!(w, "{}", self.y)
    }

    fn write_ans(&self, w: &mut impl Write) -> Result<()> {
        let x = self.x as u8;
        writeln!(w, "{}", x)?;

        let y = self.y as i32;
        writeln!(w, "{}", y)?;

        let sum = x as i32 + y;
        writeln!(w, "{}", sum)
    }
}

fn main() -> Result<()> {
    datagen::Datagen::new("./fixtures/data_types")
        .edge_cases(vec![
            Model { x: 0, y: 0.0 },
            Model { x: 1, y: 1.0 },
            Model { x: -1, y: 0.1 },
            Model { x: 256, y: 256.5 },
            Model { x: -256, y: -256.5 },
            Model {
                x: 1_000_000_000,
                y: 1_000_000_000.0,
            },
            Model {
                x: i32::MAX,
                y: -1_000_000_000.0,
            },
            Model {
                x: i32::MIN,
                y: 999_999_999.9,
            },
        ])
        .sample_cases(&mut ChaCha8Rng::seed_from_u64(42), 25)
        .generate()
}
