//! Module containing algorithm that plays number guessing game optimally.
use std::cmp::Ordering;

use super::NumberGuessingGame;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_n_guesses() {
        // Positive Numbers
        assert_eq!(AutoNumberGuesser::get_guess(0, 10), 5);
        // Negative Numbers
        assert_eq!(AutoNumberGuesser::get_guess(-5, 5), 0);
        assert_eq!(AutoNumberGuesser::get_guess(-11, -5), -8);
        // Rounding Down
        assert_eq!(AutoNumberGuesser::get_guess(1, 10), 5);
    }
}

/// An AI that plays the number guessing game in the optimal way.
pub struct AutoNumberGuesser {
    game: NumberGuessingGame,
}

impl AutoNumberGuesser {
    /// Creates an AI guesser for a Number Guessing Game
    pub fn new(min: i128, max: i128) -> Result<Self, ()> {
        let game = NumberGuessingGame::new(min, max)?;

        Ok(Self { game })
    }

    /// Returns a reference to the NumberGuessingGame.
    pub fn game(&self) -> &NumberGuessingGame {
        &self.game
    }

    /// Returns a mutable reference to the NumberGuessingGame.
    pub fn game_mut(&mut self) -> &mut NumberGuessingGame {
        &mut self.game
    }

    /// Starts the number guessing algorithm.
    pub fn start(&mut self, verbose: bool) -> u128 {
        // Resetting Variables
        self.game.reset_number();
        // Initializing initial condition
        let mut current_min = self.game.min();
        let mut current_max = self.game.max();
        let mut guess = AutoNumberGuesser::get_guess(current_min, current_max);

        loop {
            // Updating Guess
            let status = match verbose {
                true => self.game.make_guess(guess, |res, guesses, number| {
                    AutoNumberGuesser::handle_guess(res, guesses, number, guess)
                }),
                false => self.game.make_guess(guess, |_, _, _| ()),
            };

            // Changing current range
            match status {
                // Too Big
                Ordering::Greater => {
                    current_max = guess;
                    if current_max == (current_min + 1) {
                        current_max = current_min
                    }
                }
                // Too Small
                Ordering::Less => {
                    current_min = guess;
                    if current_min == (current_max - 1) {
                        current_min = current_max;
                    }
                }
                // Correct Guess
                Ordering::Equal => break,
            }

            guess = AutoNumberGuesser::get_guess(current_min, current_max);
        }

        self.game.get_n_guesses()
    }

    /// Get the next best g
    fn get_guess(min: i128, max: i128) -> i128 {
        (min + max) / 2
    }

    /// Guess handler function when the AI is set to verbose.
    fn handle_guess(result: Ordering, guesses: u128, number: i128, guess: i128) {
        println!("Alogorithm Guessed: {guess}");

        match result {
            Ordering::Greater => println!("Guess is too high."),
            Ordering::Less => println!("Guess is too low."),
            Ordering::Equal => {
                println!("It took the algorithm {guesses} guesses. To guess the number {number}.")
            }
        }
    }
}
