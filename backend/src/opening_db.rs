use std::collections::HashSet;
use std::fs::{OpenOptions, File};
use std::io::{stdout, Write, Read, BufWriter, BufReader};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

use crate::board::Board;
use crate::agent::Agent;
use crate::transposition::{TranspositionTable, TABLE_SIZE};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OpeningDatabase {
    table: TranspositionTable,
    depth: usize,
}

impl OpeningDatabase {
    pub fn generate(path: &str, depth: u32) {
        println!("Generating opening database for depth: {}", depth);
        let mut boards: HashSet<Board> = HashSet::new();
        OpeningDatabase::gen_positions(Board::new(), depth, &mut boards);
        println!("Found {} board states", boards.len());

        let cur = AtomicUsize::new(0);
        let total = boards.len();

        let stdout = Arc::new(Mutex::new(stdout()));
        let start_time = std::time::Instant::now();

        let db = Self::load("compiled_db.bin".to_string());

        print!("Calculating board scores [0.0%]  ETA: --:--  0/{}", boards.len());
        stdout.lock().unwrap().flush().unwrap();

        let map_func = |board: &Board| {
            let mut agent = Agent::new(Some(&db));
            let (score, num_visited) = agent.best_score(*board);
            let entry = board.hash() << 8 | (score as u64);

            cur.fetch_add(1, Ordering::Relaxed);
            let cur_ = cur.load(Ordering::Relaxed);

            let cur_time = std::time::Instant::now();
            let eta_secs = cur_time.duration_since(start_time).as_secs_f32() / (cur_ as f32) * (total as f32 - cur_ as f32);
            let eta = format!("{:.0}:{:0>2.0}", eta_secs / 3600.0, eta_secs / 60.0 % 60.0);
            print!("\rCalculating board scores [{:.1}%]  ETA: {}  {}/{}", (cur_ as f32)/(total as f32)*100.0, eta, cur_, total);
            stdout.lock().unwrap().flush().unwrap();

            (entry, num_visited)
        };

        let mut board_scores = boards.par_iter().map(map_func).collect::<Vec<(u64, u64)>>();
        board_scores.sort_by(|a, b| a.1.cmp(&b.1));
        println!();

        println!("Writing to file: {}", path);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        // write depth to file
        file.write_all(&depth.to_be_bytes()).unwrap();

        // write book_vec to file
        for i in 0..board_scores.len() {
            file.write_all(&board_scores[i].0.to_be_bytes()).unwrap();
        }
    }

    pub fn compile(dest_file: String, src_files: Vec<String>) {
        // let db = Self::load_raw(src_files);

        let mut table = TranspositionTable::new();
        let mut depth = 0;
        let mut entries: Vec<u64> = Vec::new();

        for path in src_files.iter().rev() {
            println!("Loading opening db from file: {}", path);

            // get file
            let mut file = File::open(path).unwrap();

            // get depth from file
            let mut depth_bytes = [0; 4];
            file.read_exact(&mut depth_bytes).unwrap();
            let file_depth: usize = u32::from_be_bytes(depth_bytes) as usize;
            if file_depth > depth { depth = file_depth; }

            // create table from file entries
            let mut buf = [0; 8];
            while file.read_exact(&mut buf).is_ok() {
                let entry = u64::from_be_bytes(buf);
                entries.push(entry >> 8);
                table.set(entry >> 8, (entry & 0xFF) as i8);
            }
        }

        let table_entries = entries.iter().filter(|&entry| table.get(*entry).is_some()).count();
        let table_fullness = table_entries as f64 / entries.len() as f64 * 100.0;
        println!("Loaded db to depth {} with {} entries", depth, table_entries);
        println!("Table space filled: {:.2}%", table_entries as f64 / TABLE_SIZE as f64 * 100.0);
        println!("Table unique hashes: {:.2}%", table_fullness);

        println!("Writing to file: {}", dest_file);
        let db = Self { table: table, depth: depth };
        let f = BufWriter::new(File::create(dest_file).unwrap());
        bincode::serialize_into(f, &db).unwrap();
        println!("Done");
    }

    pub fn load(src_file: String) -> Self {
        let f = BufReader::new(File::open(src_file).unwrap());
        bincode::deserialize_from(f).unwrap()
    }

    pub fn get(&self, key: u64, depth: usize) -> Option<i8> {
        if depth <= self.depth { self.table.get(key) } else { None }
    }

    fn gen_positions(board: Board, depth: u32, boards: &mut HashSet<Board>) {
        if board.has_winning_action() { return; }
        if board.num_actions() >= 42 { return; }

        if board.num_actions() == depth as usize { 
            boards.insert(board);
            return;
        }

        for col in 0..7 {
            if board.is_valid_col(col) {
                let mut child = board.clone();
                child.play_col(col);
                OpeningDatabase::gen_positions(child, depth, boards);
            }
        }
    }
}
