use std::io::{self, Write};

use clap::{Parser, *};
use guess_my_number_rs::{
    histogram::{Direction, Histogram},
    AutoNumberGuesser, NumberGuessingGame,
};

/// Enum for who is playing the game.
enum Player {
    /// AI player.
    AI,
    /// Human player.
    Human,
}

/// Argument parser for NumberGuessingGame.
#[derive(Parser)]
struct Arguments {
    /// Get game options interactively.
    #[arg(long, short, exclusive = true)]
    interactive: bool,

    /// Get game options from arguments.
    #[command(flatten)]
    manual: Manual,
}

#[derive(Debug, Args)]
/// Arguments to set the game options manually.
struct Manual {
    /// Sets the range of what the number can be.
    ///
    /// The range must be in the form "minimum-maximum".
    #[arg(long, value_parser = validate_range)]
    range: Option<String>,

    /// Sets the range seperately.
    #[command(flatten)]
    min_max: Range,

    /// A human player is playing the game.
    #[arg(long, group = "game-player")]
    human: bool,

    /// An AI is playing the game.
    #[arg(long, group = "game-player")]
    ai: bool,

    /// Number of iterations the AI runs
    #[arg(long, default_value_t = 200)]
    iteration: u32,
}

/// Arguments to set the range seperately.
#[derive(Debug, Args)]
#[group(conflicts_with = "range")]
struct Range {
    /// Sets the minimum the number can be.
    #[arg(long, default_value_t = 1)]
    min: i128,

    /// Sets the maxiumum the number can be.
    #[arg(long, default_value_t = 10)]
    max: i128,
}

fn main() {
    let args = Arguments::parse();

    // Getting Options
    let (player, min, max, iteration): (Player, i128, i128, u32) = if args.interactive {
        // Interactive
        get_interactive()
    } else {
        // Manual
        let args = args.manual;

        // Player
        let player = if args.human {
            Player::Human
        } else if args.ai {
            Player::AI
        } else {
            // Shouldn't be needed (redundancy)
            // Exit as no player is specified
            eprintln!("No player specified.");
            std::process::exit(1);
        };

        // Range
        let (min, max): (i128, i128) = if args.range.is_some() {
            let args_str: String = args.range.unwrap();
            let min_max: Vec<&str> = args_str.split('-').collect();

            // Shouldn't happen (redundancy)
            if min_max.len() != 2 {
                eprintln!("Invalid Range.");
                std::process::exit(1);
            }

            let min: i128 = min_max[0].parse().unwrap_or_else(|_| {
                // Shouldn't happen (redundancy)
                eprintln!("Invalid Range");
                std::process::exit(1);
            });
            let max: i128 = min_max[1].parse().unwrap_or_else(|_| {
                // Shouldn't happen (redundancy)
                eprintln!("Invalid Range");
                std::process::exit(1);
            });

            (min, max)
        } else {
            let min_max = args.min_max;
            (min_max.min, min_max.max)
        };

        (player, min, max, args.iteration)
    };

    // Checking Range
    if max < min {
        eprintln!("Maximum must be atleast the minium");
        std::process::exit(1);
    }

    // Main logic
    match player {
        Player::Human => {
            let mut game = NumberGuessingGame::new(min, max).unwrap();
            while !game.get_guess() {}
        }
        Player::AI => {
            let mut game = AutoNumberGuesser::new(min, max).unwrap();
            let mut data: Vec<u128> = Vec::new();

            for _ in 0..iteration {
                game.start(false);
                data.push(game.game().get_n_guesses());
            }

            let histogram = Histogram::from_vec(data.clone());
            histogram.print(Direction::Horizontal);

            let mean = data.iter().sum::<u128>() as f32 / data.len() as f32;
            let mean_square = data.iter().map(|x| x * x).sum::<u128>() as f32 / data.len() as f32;
            let variance = mean_square - mean * mean;
            let std = variance.sqrt();

            println!("Mean: {mean}");
            println!("Variance: {variance}");
            println!("Standard Deviation: {std}");
        }
    }
}

fn validate_range(s: &str) -> Result<String, String> {
    let min_max: Vec<&str> = s.split('-').collect();

    if min_max.len() != 2 {
        // Checking if it has a "-"
        Err(String::from("Range must be in the form minimum-maximum"))
    } else {
        // Checking if range is an integer
        min_max[0]
            .parse::<i128>()
            .map_err(|_| String::from("Minimum and Maximum must be a 128-bit signed integer"))?;
        min_max[1]
            .parse::<i128>()
            .map_err(|_| String::from("Minimum and Maximum must be a 128-bit signed integer"))?;

        // Valid Range
        Ok(String::from(s))
    }
}

fn get_interactive() -> (Player, i128, i128, u32) {
    let stdin = io::stdin();

    // Player
    let player: Player = loop {
        let mut buffer = String::new();

        // Printing prompt
        print!("Who is playing the game (AI, Player): ");
        io::stdout().flush().expect("Failed to flush stdout.");

        // Reading input
        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let player = buffer.to_ascii_lowercase();
        let player = player.trim();

        if player == "ai" {
            break Player::AI;
        } else if player == "player" {
            break Player::Human;
        }

        println!("Invalid Player.");
    };

    // Iterations
    let iteration: u32 = match player {
        Player::Human => 0,
        Player::AI => loop {
            let mut buffer = String::new();

            // Printing prompt
            print!("What the number of iteration the AI to run: ");
            io::stdout().flush().expect("Failed to flush stdout.");

            // Reading input
            stdin.read_line(&mut buffer).expect("Failed to read line.");
            let buffer = buffer.trim();

            match buffer.parse::<u32>() {
                Ok(iteration) => break iteration,
                Err(_) => println!("Invalid number of iterations."),
            }
        },
    };

    // Minimum
    let min: i128 = loop {
        let mut buffer = String::new();

        // Printing prompt
        print!("What is the minimum the random number can be: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        // Reading input
        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let buffer = buffer.trim();

        match buffer.parse::<i128>() {
            Ok(min) => break min,
            Err(_) => println!("Minimum must be a 128-bit integer."),
        }
    };

    // Maximum
    let max: i128 = loop {
        let mut buffer = String::new();

        // Printing prompt
        print!("What is the maximum the random number can be: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        // Reading input
        stdin.read_line(&mut buffer).expect("Failed to read line.");
        let buffer = buffer.trim();

        match buffer.parse::<i128>() {
            Ok(min) => break min,
            Err(_) => println!("Maximum must be a 128-bit integer."),
        }
    };

    (player, min, max, iteration)
}
