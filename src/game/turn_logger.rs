use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};
use crate::game::tictactoe_core::TurnState;

pub struct TurnLogger {
    start_time: Instant,
    end_time: Instant,
    pub player_n_turn: usize,
    pub latest_placed_coord: (usize, usize),
    pub game_state: TurnState,
    pub total_turns: usize
}

impl TurnLogger {
    pub fn start() -> TurnLogger {
        TurnLogger {
            start_time: Instant::now(),
            end_time: Instant::now(),
            player_n_turn: 0,
            latest_placed_coord: (0, 0),
            game_state: TurnState::Continue,
            total_turns: 0,
        }
    }
    pub fn restart_timer(&mut self) {
        self.start_time = Instant::now();
    }
    pub fn end_timer(&mut self) {
        self.end_time = Instant::now();
    }
    pub fn elapsed_time(&self) -> Duration {
        self.end_time - self.start_time
    }
}

impl Display for TurnLogger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Turn: {}\nPlaced coordinates: x={}; y={}\nGame State: {}\nElapsed Time: {}", self.total_turns, self.latest_placed_coord.0, self.latest_placed_coord.1, self.game_state, self.elapsed_time().as_secs_f64())
    }
}

