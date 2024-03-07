use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

struct Model {
    x: i32,
    y: f64,
}

impl Model {
    pub fn arbitrary(rng: &mut impl rand::Rng) -> Self {
        Model {
            x: rng.gen(),
            y: rng.gen_range(-1_000_000_000.0..1_000_000_000.0),
        }
    }

    pub fn write_input(&self, w: &mut impl Write) -> Result<()> {
        writeln!(w, "{}", self.x)?;
        writeln!(w, "{}", self.y)
    }

    pub fn write_ans(&self, w: &mut impl Write) -> Result<()> {
        let x = self.x as u8;
        writeln!(w, "{}", x)?;

        let y = self.y as i32;
        writeln!(w, "{}", y)?;

        let sum = x as i32 + y;
        writeln!(w, "{}", sum)
    }
}

const CASES: usize = 25;
const FIXTURE: &str = "./fixtures/data_types";

fn main() -> Result<()> {
    let mut cases = vec![
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
    ];

    let rng = &mut ChaCha8Rng::seed_from_u64(42);

    while cases.len() < CASES {
        cases.push(Model::arbitrary(rng));
    }

    let out_dir = Path::new(FIXTURE);
    if !out_dir.exists() {
        std::fs::create_dir(out_dir)?;
    }

    for (i, case) in cases.iter().enumerate() {
        let input = File::create(out_dir.join(format!("test_{}.in", i + 1)))?;
        case.write_input(&mut BufWriter::new(input))?;

        let ans = File::create(out_dir.join(format!("test_{}.ans", i + 1)))?;
        case.write_ans(&mut BufWriter::new(ans))?;
    }

    Ok(())
}
