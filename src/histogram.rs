use std::{collections::BTreeMap, fmt::Display, hash::Hash};

/// Direction of the histogram.
pub enum Direction {
    /// Histogram goes up vertically.
    Vertical,
    /// Histogram goes up horizontally.
    Horizontal,
}

/// A structure that represents a Histogram.
pub struct Histogram<T: Eq + Hash> {
    data: BTreeMap<T, u128>,
}

impl<T: Eq + Hash + Display + Ord> Histogram<T> {
    /// Constructs a new Histogram using a BTreeMap.
    ///
    /// Each of the entry represents the number of occurances
    /// of the key.
    pub fn new(data: BTreeMap<T, u128>) -> Self {
        Self { data }
    }

    /// Constructs a new Histogram from a vector of data.
    pub fn from_vec(data: Vec<T>) -> Self {
        let mut map = BTreeMap::<T, u128>::new();
        for i in data {
            let stat = map.entry(i).or_insert(0);
            *stat += 1;
        }

        Self::new(map)
    }

    /// Prints the histogram to stdout.
    pub fn print(&self, direction: Direction) {
        match direction {
            Direction::Horizontal => self.print_horizontal(),
            Direction::Vertical => self.print_vertical(),
        }
    }

    /// Prints a horizontal histogram to stdout.
    fn print_horizontal(&self) {
        for (key, value) in self.data.iter() {
            print!("{key:>5}/{value:<5}");

            for _ in 0..*value {
                print!("*");
            }
            println!();
        }
    }

    /// Prints a vertical histogram to stdout.
    fn print_vertical(&self) {
        let height = *self.get_highest().1;

        // Printing stars
        for i in (0..height).rev() {
            // Looping over all keys to check if the count is greater
            for value in self.data.values() {
                if *value > i {
                    print!("{:^5}", "*")
                } else {
                    print!("{:^5}", " ")
                }
            }
            println!()
        }

        // Printing labels
        for key in self.data.keys() {
            print!("{:^5}", key)
        }
        println!();
        // Printing Value
        for value in self.data.values() {
            print!("{:^5}", value)
        }
        println!();
    }

    /// Gets the current entry with the highest count.
    pub fn get_highest(&self) -> (&T, &u128) {
        self.data.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap()
    }

    /// Gets the current entry with the lowest count.
    pub fn get_lowest(&self) -> (&T, &u128) {
        self.data.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap()
    }

    /// Gets the data currently stored.
    pub fn data(&self) -> &BTreeMap<T, u128> {
        &self.data
    }

    /// Gets the mutable data currently stored.
    pub fn data_mut(&mut self) -> &mut BTreeMap<T, u128> {
        &mut self.data
    }

    /// Sets the data currently stored.
    pub fn set_data(&mut self, data: BTreeMap<T, u128>) {
        self.data = data;
    }

    /// Sets the data currently stored.
    pub fn set_data_list(&mut self, data: Vec<T>) {
        let mut map = BTreeMap::<T, u128>::new();
        for i in data {
            let stat = map.entry(i).or_insert(0);
            *stat += 1;
        }

        self.data = map;
    }
}
