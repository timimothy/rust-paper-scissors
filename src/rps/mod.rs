use std::io::{self, Stdin};

enum Move {
    RUST,
    PAPAER,
    SCISSORS,
    NONE,
    QUIT,
}

struct RoundState {
    p1: Move,
    p2: Move,
}

impl RoundState {
    pub fn new() -> Self {
        Self {
            p1: Move::NONE,
            p2: Move::NONE,
        }
    }

    fn reset(&mut self) {
        self.p1 = Move::NONE;
        self.p2 = Move::NONE;
    }

    pub fn get_winnder(&self) -> i8 {
        match self.p1 {
            Move::PAPAER => match self.p2 {
                Move::PAPAER => 0,
                Move::RUST => 1,
                Move::SCISSORS => 2,
                _ => 0,
            },
            Move::RUST => match self.p2 {
                Move::RUST => 0,
                Move::SCISSORS => 1,
                Move::PAPAER => 2,
                _ => 0,
            },
            Move::SCISSORS => match self.p2 {
                Move::SCISSORS => 0,
                Move::PAPAER => 1,
                Move::RUST => 2,
                _ => 0,
            },
            _ => -1,
        }
    }
}

pub struct RockPaperScissors {
    round_state: RoundState,
    round_count: i32,

    pl_name: String,
    p2_name: String,
}

impl RockPaperScissors {
    pub fn new() -> Self {
        Self {
            round_state: RoundState::new(),
            round_count: 0,

            pl_name: String::new(),
            p2_name: String::new(),
        }
    }

    pub fn fight(&mut self) {
        // Set up application for user input
        let stdin: Stdin = io::stdin();

        // Get the user name
        self.get_user_name(&stdin);
        // Get the computers name
        self.get_computer_name(&stdin);

      loop {
        self.play_round(&stdin);
        }
    }

    fn play_round(&mut self, stdin: &Stdin) {
        self.round_count = self.round_count + 1;

        self.round_state.reset();

        println!("Round {}, FIGHT!", self.round_count);

        self.round_state.p1 = self.get_user_move(stdin);
    }

    fn get_user_name(&mut self, stdin: &Stdin) {
        let mut name = String::new();

        println!("What is your name?");

        stdin.read_line(&mut name).expect("msg");
        self.pl_name = name;
    }

    fn get_computer_name(&mut self, stdin: &Stdin) {
        let mut name = String::new();

        println!("What is the name of your opponent?");

        stdin.read_line(&mut name).expect("msg");
        self.p2_name = name;
    }

    fn get_user_move(&mut self, stdin: &Stdin) -> Move {
        let mut user_input: String;
        loop {
            user_input = String::new();
            stdin.read_line(&mut user_input).expect("Opps");
            user_input = user_input.as_str().trim().to_string();

            if valid_input(&user_input) {
                break;
            }
        }

        match user_input.as_str().trim() {
            "q" => Move::QUIT,
            "r" => Move::RUST,
            "p" => Move::PAPAER,
            "s" => Move::SCISSORS,
            _ => panic!("Opps"),
        }
    }
}

const INPUTS: [&str; 4] = ["r", "p", "s", "q"];

fn valid_input(input: &str) -> bool {
    return INPUTS.contains(&input);
}
