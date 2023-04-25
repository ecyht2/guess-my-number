use std::cmp::Ordering;

use guess_my_number_rs::NumberGuessingGame;
use rand::Rng;

#[test]
fn constructor() {
    // Should Not Panic
    let game = NumberGuessingGame::new(0, 69).unwrap();
    let (min, max) = game.get_range();
    assert_eq!(min, 0);
    assert_eq!(max, 69);

    // Should return Result
    let game = NumberGuessingGame::new(0, -69);
    match game {
        Ok(_) => panic!("Maximum should be greater than minimum"),
        Err(_) => (),
    }
}

#[test]
fn make_guess() {
    let mut game = NumberGuessingGame::new(0, 0).unwrap();
    let mut handler: i128 = 0;

    // Test greater guess
    let status = game.make_guess(69, |_, _, _| handler = 69);
    assert_eq!(status, Ordering::Greater);
    assert_eq!(handler, 69);

    // Test lesser guess
    let status = game.make_guess(-69, |_, guess, _| handler = guess as i128);
    assert_eq!(status, Ordering::Less);
    assert_eq!(handler, 2);

    // Test lesser guess
    let status = game.make_guess(0, |_, _, num| handler = num);
    assert_eq!(status, Ordering::Equal);
    assert_eq!(handler, 0);
}

#[test]
fn get_n_guesses() {
    for _ in 0..100 {
        let mut game = NumberGuessingGame::new(1, 10).unwrap();

        // Getting the amount of times to guess
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(0..=100);

        // Making number wrong guesses
        for _ in 0..number {
            game.make_guess(100, |_, _, _| ());
        }

        // Testing function
        assert_eq!(game.get_n_guesses(), number);
    }
}

#[test]
fn min() {
    let mut game = NumberGuessingGame::new(1, 10).unwrap();
    assert_eq!(game.min(), 1);

    // Should not panic
    game.set_min(2).expect("Valid Range shouldn't panic");
    assert_eq!(game.min(), 2);
}

#[test]
fn max() {
    let mut game = NumberGuessingGame::new(1, 10).unwrap();
    assert_eq!(game.max(), 10);

    // Should not panic
    game.set_max(69).expect("Valid Range shouldn't panic");
    assert_eq!(game.max(), 69);
}

#[test]
fn range() {
    let mut game = NumberGuessingGame::new(1, 10).unwrap();
    assert_eq!(game.get_range(), (1, 10));

    // Changing range
    game.set_range(69, 420)
        .expect("Valid Range shouldn't panic");
    assert_eq!(game.get_range(), (69, 420));

    // Trying to set invalid range
    match game.set_range(420, 69) {
        Ok(()) => panic!("Invalid range should panic."),
        Err(()) => (),
    };
    // Range should stay the same
    assert_eq!(game.get_range(), (69, 420));
}
