use crate::game::tictactoe_game::{TicTacToeGame};
mod game;

fn main() {
    /*
    let mut game: TicTacToeGame;
    //game: TicTacToeGame = TicTacToeGame::load_default_2player_game();
    game = TicTacToeGame::load_default_1player_game(AiDifficulties::Hard);
    //game = TicTacToeGame::load_default_ai_game(AiDifficulties::Hard, AiDifficulties::Hard);
    game.change_size(4, 4);
    game.set_first_player(2);
    game.show_turn_info_mode = true;

    loop {
        game.start_game();
        thread::sleep(Duration::from_millis(5000));
        println!("\n\nRestarting game\n\n");
        thread::sleep(Duration::from_millis(5000));
        game.reload_game();
    }
    */
    TicTacToeGame::start_new_game_prompts()
}




