pub mod rand {
    use std::cmp::Ordering;
    use std::io;
    const COMPUTED_GUESS: i32 = 236;

    pub fn init() {
        loop {
            let mut guess = String::new();
            println!("Guess: ");
            io::stdin().read_line(&mut guess).unwrap();
            let guess: i32 = guess.trim().parse().unwrap();

            match guess.cmp(&COMPUTED_GUESS) {
                Ordering::Greater => println!("Mf's way too big! (Hint: less is more)"),
                Ordering::Equal => break println!("Correct!"),
                Ordering::Less => println!("It's less man...less!!!"),
            }
        }
    }
}
