use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn main() {
    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();

    for i in 0..4 {
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            search_for_solution(i, sender_n, is_solution_found);
        });
    }

    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution.");
            println!("The number is : {}.", i);
            println!("Result hash: {}.", hash);
        }
        Err(_) => panic!("Worker threads disconnected before the solution was found!"),
    }
}

fn search_for_solution(
    start_at: usize,
    sender: mpsc::Sender<Solution>,
    is_solution_found: Arc<AtomicBool>,
) {
    let mut iteration_no = 0;
    for number in (start_at..).step_by(4) {
        if let Some(solution) = verify_number(number) {
            is_solution_found.store(true, Ordering::Relaxed);
            match sender.send(solution) {
                Ok(_) => {}
                Err(_) => println!(
                    "Receiver has stopped listening, dropping worker number {}.",
                    start_at
                ),
            }

            return;
        } else if iteration_no % 1000 == 0 && is_solution_found.load(Ordering::Relaxed) {
            return;
        }

        iteration_no += 1;
    }
}

const BASE: usize = 42;
static DIFFICULTY: &'static str = "000000";

struct Solution(usize, String);

fn verify_number(number: usize) -> Option<Solution> {
    let hash: String = calculate_hash(&(number * BASE)).to_string();
    if hash.ends_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
