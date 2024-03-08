#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use rand_distr::{Alphanumeric, Binomial};

pub trait Model {
    fn arbitrary(rng: &mut impl rand::Rng) -> Self;
    fn write_input(&self, w: &mut impl Write) -> Result<()>;
    fn write_ans(&self, w: &mut impl Write) -> Result<()>;
}

pub struct Datagen<M: Model> {
    cases: Vec<M>,
    fixture: String,
}

impl<M: Model> Datagen<M> {
    pub fn new(fixture: &str) -> Self {
        Self {
            cases: Vec::new(),
            fixture: fixture.to_string(),
        }
    }

    pub fn edge_cases(self, edge_cases: Vec<M>) -> Self {
        let mut cases = self.cases;
        cases.extend(edge_cases);
        Self { cases, ..self }
    }

    pub fn sample_cases(self, rng: &mut impl rand::Rng, count: usize) -> Self {
        let mut cases = self.cases;
        while cases.len() < count {
            println!("Generating case {}", cases.len() + 1);
            cases.push(M::arbitrary(rng));
        }
        Self { cases, ..self }
    }

    pub fn generate(self) -> Result<()> {
        let out_dir = Path::new(&self.fixture);
        if !out_dir.exists() {
            std::fs::create_dir(out_dir)?;
        }
        for (i, case) in self.cases.into_iter().enumerate() {
            let input = File::create(out_dir.join(format!("test_{}.in", i + 1)))?;
            case.write_input(&mut BufWriter::new(input))?;

            let ans = File::create(out_dir.join(format!("test_{}.ans", i + 1)))?;
            case.write_ans(&mut BufWriter::new(ans))?;
        }

        Ok(())
    }
}

pub fn rand_string(rng: &mut impl rand::Rng, max_len: u64, expected_len: u64) -> String {
    assert!(max_len > 0);
    let max_len = max_len - 1;
    let len = rng.sample(Binomial::new(max_len, expected_len as f64 / max_len as f64).unwrap()) + 1;
    (0..len).map(|_| rng.sample(Alphanumeric) as char).collect()
}
