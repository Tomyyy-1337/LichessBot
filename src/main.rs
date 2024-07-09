mod lichess_api;
mod engine;

use std::str::FromStr;

use chess::Board;

#[tokio::main]
async fn main() {
    let api = lichess_api::LichessAPI::default();
    let engine = engine::Engine::new();
    
    api.get_game("gGLBG8of").await;

    
    // let game = api.load_puzzle("OFtie").await.unwrap();
    // let board = game.current_position();
    
    // let best_move = engine.best_move(&board);
    // println!("Best move: {}", best_move.unwrap());   


}
