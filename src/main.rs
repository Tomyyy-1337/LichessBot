use std::str::FromStr;

use chess::{Board, ChessMove};
use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap, AUTHORIZATION};

fn main() {
    let api_token = std::env::var("LICHESS_API_KEY").unwrap();

    let client = Client::new();
    
    load_puzzle(&client, &api_token);

    let board = Board::default();

    load_tablebase(&client, &api_token, &board);


    

    
}

fn load_tablebase(client: &Client, api_token: &str, board: &Board) {
    // let board = Board::from_str("4k3/6KP/8/8/8/8/7p/8 w - - 0 1").unwrap();
    let fen = board.to_string().replace(" ", "_"); 

    println!("{}", fen);


    let url = format!("http://tablebase.lichess.ovh/standard?fen={}", fen);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, api_token.parse().unwrap());

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let body = response.text().unwrap();
            println!("{}", body);
        }
        reqwest::StatusCode::BAD_REQUEST => {
            println!("Not in tablebase");
        }
        _ => {
            println!("Error: {}", response.status());
        }
    }
}   

fn load_puzzle(client: &Client, api_token: &str) {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, api_token.parse().unwrap());

    let response = client
        .get("https://lichess.org/api/puzzle/2z4YZ")
        .headers(headers)
        .send()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let body = response.text().unwrap();

            let pgn = body.split("\"pgn\":\"").skip(1).next().unwrap().split("\"").next().unwrap();
            
            let game = load_game_from_pgn(pgn);
            
            println!("{}", game.current_position());
        }
        _ => println!("Error: {}", response.status()),
    }
}

fn load_game_from_pgn(pgn: &str) -> chess::Game {
    let mut game = chess::Game::new();
    for mv in pgn.split_ascii_whitespace() {
        game.make_move(ChessMove::from_san(&game.current_position(), mv).unwrap());
    }
    game
}