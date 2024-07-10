mod lichess_api;
mod engine;

use std::str::FromStr;

use chess::{Board, Game};

#[tokio::main]
async fn main() {
    let api = lichess_api::LichessAPI::default();
    let engine = engine::Engine::new();
    
    // api.get_game("gGLBG8of").await;

    
    // let game = api.load_puzzle("OFtie").await.unwrap();
    // let board = game.current_position();
    
    // let best_move = engine.best_move(&board);
    // println!("Best move: {}", best_move.unwrap());   


    // UCI protocol
    // read next line 
    // let mut line = String::new();
    // std::io::stdin().read_line(&mut line).unwrap();

    // if line.trim().to_ascii_lowercase() == "uci" {
    //     println!("id name Night Of NI");
    //     println!("id author Tomyyy");
    //     println!("uciok");
    // }

    // let game = Game::new();

    let game_id = api.start_game().await;

    println!("Game ID: {:?}", game_id);



}
