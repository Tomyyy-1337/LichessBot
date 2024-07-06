use chess::ChessMove;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION};

fn main() {
    let api_token = std::env::var("LICHESS_API_KEY").unwrap();

    let client = Client::new();
    let api_token = format!("Bearer {}", api_token);

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