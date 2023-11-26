use lib::board::Board;
use lib::agent::Agent;
use lib::opening_db::OpeningDatabase;
use once_cell::sync::Lazy;

static DB: Lazy<OpeningDatabase> = Lazy::new(|| { OpeningDatabase::load("compiled_db.bin".to_string()) });

fn use_benchmark_file(filename: &str) {
    let bench_file = std::fs::read_to_string(filename).unwrap();

    let mut runtime = 0;
    let mut total_visits = 0;
    for line in bench_file.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let expected = parts[1].parse::<i8>().unwrap();

        let mut agent = Agent::new(Some(&DB));
        let board = Board::from_position(parts[0]).unwrap();

        let start = std::time::Instant::now();
        let (actual, visited) = agent.best_score(board);
        let end = std::time::Instant::now();

        runtime += end.duration_since(start).as_micros();
        total_visits += visited;

        assert_eq!(expected, actual);
    }

    let avg = runtime as f64 / bench_file.lines().count() as f64;
    if avg < 1_000.0 {
        println!("Average time: {:.2} us", avg);
    } else if avg < 1_000_000.0 {
        println!("Average time: {:.2} ms", avg / 1_000.0);
    } else {
        println!("Average time: {:.2} s", avg / 1_000_000.0);
    }
    println!("Average visits: {}", total_visits / bench_file.lines().count() as u64);
}

#[test]
fn benchmark_end_easy() {
    use_benchmark_file("benchmarks/Test_L3_R1");
}

#[test]
fn benchmark_middle_easy() {
    use_benchmark_file("benchmarks/Test_L2_R1");
}

#[test]
fn benchmark_middle_medium() {
    use_benchmark_file("benchmarks/Test_L2_R2");
}

#[test]
fn benchmark_begin_easy() {
    use_benchmark_file("benchmarks/Test_L1_R1");
}

#[test]
fn benchmark_begin_medium() {
    use_benchmark_file("benchmarks/Test_L1_R2");
}

#[test]
#[ignore]
fn benchmark_begin_hard() {
    use_benchmark_file("benchmarks/Test_L1_R3");
}
