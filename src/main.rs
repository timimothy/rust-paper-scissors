use core::fmt;
use rand::Rng;
use std::io;
enum Move {
    RUST,
    PAPAER,
    SCISSORS,
    NONE,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::NONE => write!(f, "None"),
            Move::PAPAER => write!(f, "Paper"),
            Move::RUST => write!(f, "Rust"),
            Move::SCISSORS => write!(f, "Scissors"),
        }
    }
}

struct GameState {
    player1: Move,
    player2: Move,
}

const INPUTS: [&str; 4] = ["r", "p", "s", "c"];

fn main() {
    println!("Hello, world!");

    game();
}

fn game() {
    let mut game_state: GameState = GameState {
        player1: Move::NONE,
        player2: Move::NONE,
    };

    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input).expect("Opps");

    println!("input {} ", user_input);

    // This is the game loop.
    loop {
        println!("Your move bucko!:");

        while !valid_input(&user_input) {
            user_input = String::new();
            stdin.read_line(&mut user_input).expect("Opps");
            user_input = user_input.as_str().trim().to_string();
        }

        match user_input.as_str().trim() {
            "c" => break,
            "r" => game_state.player1 = Move::RUST,
            "p" => game_state.player1 = Move::PAPAER,
            "s" => game_state.player1 = Move::SCISSORS,
            _ => println!("{}", user_input),
        }

        user_input = String::new();

        game_state.player2 = get_random_move();

        println!("Player Move {}", game_state.player1);
        println!("Computer Move {}", game_state.player2);

        let winner = get_winner(&game_state);

        println!("Winner is {}", winner);
    }
}

fn valid_input(input: &str) -> bool {
    let result = INPUTS.contains(&input);
    println!("valid {}", result);
    return result;
}

fn get_random_move() -> Move {
    let mut rng = rand::thread_rng();
    match rng.gen_range(1..3) {
        1 => Move::PAPAER,
        2 => Move::RUST,
        3 => Move::SCISSORS,
        _ => Move::NONE,
    }
}

fn get_winner(game: &GameState) -> i32 {
    match game.player1 {
        Move::PAPAER => match game.player2 {
            Move::PAPAER => 0,
            Move::RUST => 1,
            Move::SCISSORS => 2,
            _ => 0,
        },
        Move::RUST => match game.player2 {
            Move::RUST => 0,
            Move::SCISSORS => 1,
            Move::PAPAER => 2,
            _ => 0,
        },
        Move::SCISSORS => match game.player2 {
            Move::SCISSORS => 0,
            Move::PAPAER => 1,
            Move::RUST => 2,
            _ => 0,
        },
        _ => panic!("Panic"),
    }
}
