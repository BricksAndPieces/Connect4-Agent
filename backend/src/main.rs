use std::io::{BufReader, Write};
use std::net::TcpListener;

use lib::board::Board;
use lib::agent::Agent;
use lib::opening_db::OpeningDatabase;
use std::io::BufRead;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let db_file = if args.len() >= 2 { Some(args[1].clone()) } else { None };
    let db = db_file.as_ref().map(|db_file|  {
            println!("Loading database: {}", db_file);
            OpeningDatabase::load(db_file.to_string())
        }
    );
    
    let mut agent = Agent::new(db.as_ref());
    println!("Agent initialized!");
    
    // check for webserver flag
    if args.len() >= 3 && args[2] == "--webserver" {
        println!("Starting webserver...");
        webserver(&mut agent);
    } else {
        cli(&mut agent);
    }
}

fn webserver(agent: &mut Agent) {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let buf_reader = BufReader::new(&stream);
        let http_req: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_req);

        if http_req[0].split(" ").collect::<Vec<&str>>()[0] != "GET" {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            println!("Response: {:#?}\n", response);
            continue;
        }

        let get_req = http_req[0].split(" ").collect::<Vec<&str>>()[1].split("/").collect::<Vec<&str>>();
        if get_req.len() < 2 || get_req[1] != "api" {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            println!("Response: {:#?}\n", response);
            continue;
        }

        let pos = if get_req.len() < 3 { "" } else { get_req[2] };
        let board = Board::from_position(pos);
        match board {
            Ok(board) => {
                let (col, score) = agent.best_col(board);
                let response = format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\n\r\n{{\"col\": {}, \"score\": {}}}\n", col, score);
                stream.write_all(response.as_bytes()).unwrap();
                println!("Response: {:#?}\n", response);
            },
            Err(_) => {
                // invalid board position
                let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                println!("Response: {:#?}\n", response);
            }
        }
    }
}

fn cli(agent: &mut Agent) {
    let mut board = Board::new();

    loop {
        println!("Player turn:");
        board.print();
        println!("Enter your move: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let col: usize = (input.trim().parse::<usize>().unwrap()) - 1;
        if col > 6  || !board.is_valid_col(col) {
            println!("Invalid move");
            continue;
        }

        let win = board.is_winning_col(col);
        board.play_col(col);

        if win {
            println!("Player wins!");
            break;
        }

        if board.num_actions() == 42 {
            println!("Draw!");
            break;
        }

        println!();

        println!("Agent turn:");
        let (col, score) = agent.best_col(board);
        let win = board.is_winning_col(col as usize);
        board.play_col(col as usize);

        println!("Agent played column: {}", col + 1);
        println!("Score: {}", score);
        board.print();
        
        if win {
            println!("Agent wins!");
            break;
        }

        if board.num_actions() == 42 {
            println!("Draw!");
            break;
        }

        println!()
    }
}
