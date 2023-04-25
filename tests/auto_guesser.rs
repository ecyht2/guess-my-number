use guess_my_number_rs::AutoNumberGuesser;

#[test]
fn constructor() {
    // Should Not Panic
    let game = AutoNumberGuesser::new(0, 69).unwrap();
    let (min, max) = game.game().get_range();
    assert_eq!(min, 0);
    assert_eq!(max, 69);

    // Should return Result
    let game = AutoNumberGuesser::new(0, -69);
    match game {
        Ok(_) => panic!("Maximum should be greater than minimum"),
        Err(_) => (),
    }
}

#[test]
fn test_auto_guesser() {
    let mut game = AutoNumberGuesser::new(0, 7).unwrap();

    for _ in 0..100 {
        game.start(true);
        // The number of guesses must be less than or equal to log2(8) + 1
        assert!(game.game().get_n_guesses() <= (8.0_f32.log2() as u128 + 1))
    }
}
