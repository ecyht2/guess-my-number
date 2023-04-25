//! Number guessing game.
use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_n_guesses() {
        let mut game = NumberGuessingGame::new(1, 10).unwrap();
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let number = rng.gen_range(0..=100);

            // Making number wrong guesses
            for _ in 0..number {
                game.make_guess(100, |_, _, _| ());
            }

            // Testing function
            assert_eq!(game.get_n_guesses(), number);
            // Resetting
            game.reset_n_guesses();
        }
    }
}

/// A number guessing game.
pub struct NumberGuessingGame {
    /// Minimum the number can be.
    min: i128,
    /// Maximum the number can be.
    max: i128,
    /// The number of guesses made.
    guesses: u128,
    /// The current number.
    number: i128,
}

impl NumberGuessingGame {
    /// Creates a number guessing game struct.
    ///
    /// An Err would be returned if min > max.
    pub fn new(min: i128, max: i128) -> Result<Self, ()> {
        let mut output = Self {
            min: 0,
            max: 0,
            guesses: 0,
            number: 0,
        };

        output.set_range(min, max)?;

        Ok(output)
    }

    /// Makes a guess of the generated number.
    pub fn make_guess<T>(&mut self, guess: i128, guess_handler: T) -> Ordering
    where
        T: FnOnce(Ordering, u128, i128),
    {
        // Increasing the amount of guesses
        self.guesses += 1;

        let result = guess.cmp(&self.number);
        guess_handler(result, self.guesses, self.number);

        result
    }

    /// Guess the number from standard input.
    ///
    /// `true` would be returned if the guess is correct, `false` would be returned otherwise.
    pub fn get_guess(&mut self) -> bool {
        loop {
            // Getting the guess from stdin
            let mut guess = String::new();

            // Printing prompt
            print!("Guess a number: ");
            io::stdout().flush().expect("Failed to flush stdout.");

            io::stdin()
                .read_line(&mut guess)
                .expect("Unable to read line.");

            // Parsing Guess
            let guess: i128 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid Number");
                    continue;
                }
            };

            let status = self.make_guess(guess, Self::handle_guess);

            // Returning Guess result
            return match status {
                Ordering::Equal => true,
                _ => false,
            };
        }
    }

    /// Resets the number currently stored.
    pub fn reset_number(&mut self) {
        let mut rng = rand::thread_rng();
        // Reseting Number
        self.number = rng.gen_range(self.min..=self.max);
        // Resetting guesses
        self.reset_n_guesses();
    }

    /// Resets the number of guesses made.
    fn reset_n_guesses(&mut self) {
        self.guesses = 0;
    }

    /// Gets the number of guesses made.
    pub fn get_n_guesses(&self) -> u128 {
        self.guesses
    }

    /// Prints the result of make_guess() in a human readable form.
    fn handle_guess(result: Ordering, guesses: u128, number: i128) {
        match result {
            Ordering::Greater => println!("Your guess is too high."),
            Ordering::Less => println!("Your guess is too low."),
            Ordering::Equal => println!("Congratulations!! Your guess is correct. The number is {number}. You took {guesses} guesses."),
        }
    }

    /// Gets the minimum the number can be.
    pub fn min(&self) -> i128 {
        self.min
    }

    /// Sets the minimum the number can be.
    ///
    /// An Err would be returned if min is greater than max. If this happens
    /// the number would not change.
    pub fn set_min(&mut self, min: i128) -> Result<(), ()> {
        self.set_range(min, self.max)
    }

    /// Gets the maxiumum the number can be.
    pub fn max(&self) -> i128 {
        self.max
    }

    /// Sets the maximum the number can be.
    ///
    /// An Err would be returned if min is greater than max. If this happens
    /// the number would not change.
    pub fn set_max(&mut self, max: i128) -> Result<(), ()> {
        self.set_range(self.min, max)
    }

    /// Gets the range the number can be in.
    ///
    /// This function returns a tuple in the form (miniumum, maximum) inclusive of the number.
    pub fn get_range(&self) -> (i128, i128) {
        (self.min(), self.max())
    }

    /// Sets the range the number can be in.
    ///
    /// An Err would be returned if min is greater than max. If this happens
    /// the number would not change.
    pub fn set_range(&mut self, min: i128, max: i128) -> Result<(), ()> {
        // Checking Range
        if min > max {
            return Err(());
        }

        // Setting min and max
        self.min = min;
        self.max = max;
        self.reset_number();

        Ok(())
    }
}
