//! A number guessing game.
pub mod auto_guesser;
pub mod histogram;
pub mod number_game;

pub use auto_guesser::AutoNumberGuesser;
pub use number_game::NumberGuessingGame;

/// A struct storing common statistics of a given data.
#[derive(Debug)]
pub struct Statistics {
    /// Mean value of the data.
    mean: f32,
    /// Standard deviation of the data.
    std: f32,
    /// Variance of the data.
    variance: f32,
}

impl Statistics {
    /// Constructs a new Statistics.
    pub fn new(mean: f32, std: f32, variance: f32) -> Self {
        Self {
            mean,
            std,
            variance,
        }
    }

    /// Prints the mean, standard deviation and variance into standard output.
    pub fn print(&self) {
        println!("Mean: {}", self.mean);
        println!("Variance: {}", self.variance);
        println!("Standard Deviation: {}", self.std);
    }

    /// Gets the mean value of the data.
    pub fn mean(&self) -> f32 {
        self.mean
    }

    /// Gets the standard deviation of the data.
    pub fn std(&self) -> f32 {
        self.std
    }

    /// Gets the variance of the data.
    pub fn variance(&self) -> f32 {
        self.variance
    }
}

impl From<Vec<u128>> for Statistics {
    /// Creates a new Statistics using a `std::vec::Vec` containting all the data.
    ///
    /// ```
    /// use guess_my_number_rs::Statistics;
    ///
    /// let data: Vec<u128> = vec![1, 2, 3, 4];
    ///
    /// let stats = Statistics::from(data);
    /// assert_eq!(stats.mean(), 2.5);
    /// assert_eq!(stats.std(), 1.118034);
    /// assert_eq!(stats.variance(), 1.25);
    /// ```
    fn from(source: Vec<u128>) -> Self {
        // Calculating values
        let mean = source.iter().sum::<u128>() as f32 / source.len() as f32;
        let mean_square = source.iter().map(|x| x * x).sum::<u128>() as f32 / source.len() as f32;
        let variance = mean_square - mean * mean;
        let std = variance.sqrt();

        Self {
            mean,
            std,
            variance,
        }
    }
}
