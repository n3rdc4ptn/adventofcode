use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let final_state = solve1(lines.clone());

    println!("Final state: {:?}", final_state);

    let final_state2 = solve2(lines);

    println!("Final state 2: {:?}", final_state2);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Moves {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None,
        }
    }

    fn effective_move(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Lost) => Self::Scissors,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Lost) => Self::Rock,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Lost) => Self::Paper,
            (a, Outcome::Draw) => *a,
        }
    }

    fn play(&self, against: &Moves) -> Outcome {
        match (self, against) {
            // Equal
            (a, b) if a == b => Outcome::Draw,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Lost,
        }
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "X" => Some(Outcome::Lost),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    fn as_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug)]
struct GameState {
    your_points: u32,
    opponent_points: u32,
}

fn solve1(lines: Vec<String>) -> GameState {
    let final_state: GameState = lines.iter().fold(
        GameState {
            opponent_points: 0,
            your_points: 0,
        },
        |state, round| {
            let moves: Vec<&str> = round.split(" ").collect();
            if let (Some(opp_move), Some(your_move)) = (
                moves.get(0).and_then(|x| Moves::from_str(x)),
                moves.get(1).and_then(|x| Moves::from_str(x)),
            ) {
                return match your_move.play(&opp_move) {
                    Outcome::Win => GameState {
                        your_points: state.your_points + your_move.as_u32() + Outcome::Win.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Lost.as_u32(),
                    },
                    Outcome::Draw => GameState {
                        your_points: state.your_points
                            + your_move.as_u32()
                            + Outcome::Draw.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Draw.as_u32(),
                    },
                    Outcome::Lost => GameState {
                        your_points: state.your_points
                            + your_move.as_u32()
                            + Outcome::Lost.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Win.as_u32(),
                    },
                };
            }

            state
        },
    );

    final_state
}

fn solve2(lines: Vec<String>) -> GameState {
    let final_state = lines.iter().fold(
        GameState {
            your_points: 0,
            opponent_points: 0,
        },
        |state, round| {
            let moves: Vec<&str> = round.split(" ").collect();

            if let (Some(opp_move), Some(pref_outcome)) = (
                moves.get(0).and_then(|x| Moves::from_str(x)),
                moves.get(1).and_then(|x| Outcome::from_str(x)),
            ) {
                let your_move = opp_move.effective_move(&pref_outcome);

                return match your_move.play(&opp_move) {
                    Outcome::Win => GameState {
                        your_points: state.your_points + your_move.as_u32() + Outcome::Win.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Lost.as_u32(),
                    },
                    Outcome::Draw => GameState {
                        your_points: state.your_points
                            + your_move.as_u32()
                            + Outcome::Draw.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Draw.as_u32(),
                    },
                    Outcome::Lost => GameState {
                        your_points: state.your_points
                            + your_move.as_u32()
                            + Outcome::Lost.as_u32(),
                        opponent_points: state.opponent_points
                            + opp_move.as_u32()
                            + Outcome::Win.as_u32(),
                    },
                };
            }

            state
        },
    );

    final_state
}
