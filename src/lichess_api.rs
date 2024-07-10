use std::{io::Read, str::FromStr};

use arrow::ipc::reader::StreamReader;
use rayon::{option, result};
use reqwest::{Client, Method, Request, RequestBuilder};
use chess::{ChessMove, Game};
use serde_json::{Deserializer, Value};
use futures_util::StreamExt;

pub struct LichessAPI {
    client: Client,
    api_token: String,
}

impl LichessAPI {
    pub fn default() -> Self {
        let api_token = std::env::var("LICHESS_API_KEY").unwrap();
        let client = Client::new();

        Self { client, api_token }
    }

    pub async fn load_puzzle(&self, puzzle_id: &str) -> Option<Game> {
        let url = format!("https://lichess.org/api/puzzle/{}", puzzle_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .unwrap();
    
        match response.text().await {
            Ok(body) => {
                let puzzle = body.parse::<serde_json::Value>().unwrap();
                let pgn = puzzle["game"]["pgn"].as_str().unwrap();
                let game = Self::load_game_from_pgn(pgn);
                Some(game)
            }
            Err(_) => None,
        }
    }

    fn load_game_from_pgn(pgn: &str) -> chess::Game {
        let mut game = chess::Game::new();
        for mv in pgn.split_ascii_whitespace() {
            game.make_move(ChessMove::from_san(&game.current_position(), mv).unwrap());
        }
        game
    }

    // Returns a vector of tuples with the possible moves, the DTZ and if the move is a zeroing move
    pub async fn load_tablebase_dtz(&self, board: &chess::Board) -> Option<Vec<(ChessMove, i64, bool)>> {
        let fen = board.to_string().replace(" ", "_");
        let url = format!("http://tablebase.lichess.ovh/standard?fen={}", fen);
        println!("URL: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                let body = response.text().await.unwrap().parse::<serde_json::Value>().unwrap();
                
                let moves = body["moves"].as_array().unwrap().iter()
                    .filter(|m| m["category"] != "unknown")
                    .map(|m| {
                        let uci = m["uci"].as_str().unwrap();
                        let dtz = m["dtz"].as_i64().unwrap();
                        let zeroing = m["zeroing"].as_bool().unwrap();
                        (ChessMove::from_str(uci).unwrap(), dtz, zeroing)
                    }
                ).collect();

                Some(moves)
            }   
            _ => {
                None
            }
        }
    }

    pub async fn start_game(&self) -> Option<String> {
        let url = "https://lichess.org/api/challenge/ai";
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.api_token).parse().unwrap());

        let params = [
            ("level", "2"), // Level von Stockfish (1-8)
            ("color", "white"), // Ihre Farbe ("white" oder "black")
            ("variant", "standard"), // Variante ("standard", "chess960", "crazyhouse", "antichess", "atomic", "horde", "kingOfTheHill", "racingKings", "threeCheck")/ Wer spielt gegen den Bot? ("bot" oder "user
        ];

        let response = self.client
            .post(url)
            .headers(headers)
            .form(&params)
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let game_info: serde_json::Value = response.json().await.unwrap();

            return Some(game_info["game"]["id"].as_str().unwrap().to_string())
        }
        println!("Failed to start game: {:?}", response.text().await.unwrap());
        None
        
        
    }

    pub async fn get_game(&self, game_id: &str) -> Game {
        let url = format!("https://lichess.org/api/bot/game/stream/{}", game_id);
        println!("URL: {}", url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.api_token).parse().unwrap());
        headers.insert(reqwest::header::ACCEPT, "application/x-ndjson".parse().unwrap());


        let mut response = self.client.get(&url)
            .headers(headers)
            .send()
            .await
            .unwrap()
            .bytes_stream();            

        while let Some(item) = response.next().await {
            println!("{:?}", item.unwrap());
        }

       
        Game::new()
    }

}