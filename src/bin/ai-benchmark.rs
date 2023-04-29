//! Benchmark the effectiveness of the AI algorithm.
//!
//! This binary will get how many tries it took the
//! AI to guess the correct number from 1 to a given value.
//! The value ranges from the arguement `min` to argument `max`,
//! increasing each time by `step`. For each value, it will test
//! the algorithm for `iterations` number of iterations.

use clap::Parser;
use guess_my_number_rs::{AutoNumberGuesser, Statistics};
use serde_derive::Serialize;

/// Command line arguments for ai-benchmark.
#[derive(Parser, Debug)]
struct Arguments {
    /// Minimum the given value can be.
    #[arg(long, default_value_t = 100)]
    min: i128,

    /// Maximum the given value can be.
    #[arg(long, default_value_t = 5000)]
    max: i128,

    /// The amount the given value increases.
    #[arg(long, default_value_t = 100)]
    step: usize,

    /// The amount of iterations each given value is tested.
    #[arg(long, default_value_t = 500)]
    iterations: u128,

    /// Output csv file
    #[arg(long, short, default_value_t = String::from("data.csv"))]
    output: String,
}

/// Statistics to export to csv for AI benchmark.
#[derive(Serialize)]
pub struct BenchmarkStats {
    /// The maximum number.
    #[serde(rename = "Max Number")]
    max_value: i128,

    /// The mean.
    #[serde(rename = "Mean")]
    mean: f32,

    /// The standard deviation.
    #[serde(rename = "Standard Deviation")]
    std: f32,
}

impl BenchmarkStats {
    /// Constructor for BenchmarkStats
    pub fn new(max_value: i128, mean: f32, std: f32) -> Self {
        Self {
            max_value,
            mean,
            std,
        }
    }
}

fn main() {
    let args = Arguments::parse();

    // Checking for valid arguments
    if args.min > args.max {
        eprintln!("Argument `max` must be greater than the `min`.");
        std::process::exit(1);
    }

    if args.min < 1 {
        eprintln!("Argument `min` must be greater than 0.");
        std::process::exit(1);
    }

    // Intializing structs.
    let mut guesser = AutoNumberGuesser::new(1, args.min).unwrap();
    let mut writer = match csv::Writer::from_path(&args.output) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Looping over all given values
    for i in (args.min..=args.max).step_by(args.step) {
        // Setting max to current value
        guesser.game_mut().set_max(i).unwrap();
        let mut data = Vec::new();

        // Collecting data
        for _ in 0..args.iterations {
            data.push(guesser.start(false));
        }

        // Ouputting statistics
        let stats = Statistics::from(data);
        let stats = BenchmarkStats::new(i, stats.mean(), stats.std());
        match writer.serialize(stats) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}
