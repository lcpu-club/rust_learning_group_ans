use std::io::{Result, Write};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod datagen;

struct Model {
    s: String,
    n: i32,
}

impl Model {
    fn new(s: impl Into<String>, n: i32) -> Self {
        let s = s.into();
        Self { s, n }
    }
}

impl datagen::Model for Model {
    fn arbitrary(rng: &mut impl rand::Rng) -> Self {
        let mut s = datagen::rand_string(rng, 100, 10);
        while rng.gen_bool(0.5) {
            s.insert(rng.gen_range(0..=s.len()), 'y');
        }
        let n = rng.gen_range(1..=1000);
        Model { s, n }
    }

    fn write_input(&self, w: &mut impl Write) -> Result<()> {
        writeln!(w, "{}", self.s)?;
        writeln!(w, "{}", self.n)
    }

    fn write_ans(&self, w: &mut impl Write) -> Result<()> {
        let s: String = self.s.chars().filter(|&c| c != 'y').collect();
        let s = s.repeat(self.n as usize);
        writeln!(w, "{}", s)
    }
}

fn main() -> std::io::Result<()> {
    datagen::Datagen::new("./fixtures/data_types_2")
        .edge_cases(vec![
            Model::new("x", 1),
            Model::new("y", 2),
            Model::new("xy", 3),
            Model::new("你好", 4),
            Model::new("♿😅😅♿", 5),
            Model::new("yayayyayayay", 5),
            Model::new("Yayayyayayay", 5),
            Model::new("amd, yes!", 1000),
        ])
        .sample_cases(&mut ChaCha8Rng::seed_from_u64(42), 25)
        .generate()
}
