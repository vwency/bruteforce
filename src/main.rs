use anyhow::Result;
use bruteforce::try_password;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const ZIP_PATH: &str = "attack_target/archive.zip";
const THREAD_RANGES: [(u32, u32); 4] = [(0, 2500), (2500, 5000), (5000, 7500), (7500, 10000)];

fn main() -> Result<()> {
    let zip_data = load_zip_file()?;
    let password_found = Arc::new(AtomicBool::new(false));

    println!("=== Многопоточный брутфорс ===");
    let start = Instant::now();
    run_bruteforce(&zip_data, &password_found);
    println!("Готово за: {:?}", start.elapsed());

    println!("\n=== Однопоточный брутфорс ===");
    let start_single = Instant::now();
    run_single_thread_bruteforce(&zip_data)?;
    println!("Готово за: {:?}", start_single.elapsed());

    Ok(())
}

fn load_zip_file() -> Result<Arc<Vec<u8>>> {
    let data = std::fs::read(ZIP_PATH)?;
    Ok(Arc::new(data))
}

fn run_bruteforce(zip_data: &Arc<Vec<u8>>, password_found: &Arc<AtomicBool>) {
    let handles = spawn_worker_threads(zip_data, password_found);
    wait_for_all_threads(handles);
}

fn spawn_worker_threads(
    zip_data: &Arc<Vec<u8>>,
    password_found: &Arc<AtomicBool>,
) -> Vec<thread::JoinHandle<()>> {
    THREAD_RANGES
        .iter()
        .enumerate()
        .map(|(thread_id, &(range_start, range_end))| {
            spawn_worker(thread_id, range_start, range_end, zip_data, password_found)
        })
        .collect()
}

fn spawn_worker(
    thread_id: usize,
    range_start: u32,
    range_end: u32,
    zip_data: &Arc<Vec<u8>>,
    password_found: &Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    let zip_data = Arc::clone(zip_data);
    let password_found = Arc::clone(password_found);
    thread::spawn(move || {
        bruteforce_password_range(
            thread_id,
            range_start,
            range_end,
            &zip_data,
            &password_found,
        )
    })
}

fn wait_for_all_threads(handles: Vec<thread::JoinHandle<()>>) {
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

fn bruteforce_password_range(
    thread_id: usize,
    range_start: u32,
    range_end: u32,
    zip_data: &[u8],
    password_found: &AtomicBool,
) {
    let start_time = Instant::now();
    for password_num in range_start..range_end {
        if should_stop(password_found) {
            log_thread_stopped(thread_id, start_time);
            return;
        }
        let password = format_password(password_num);
        if try_password(&password, zip_data) {
            mark_password_found(password_found);
            log_password_found(thread_id, &password, start_time);
            return;
        }
    }
    log_range_completed(thread_id, start_time);
}

fn should_stop(password_found: &AtomicBool) -> bool {
    password_found.load(Ordering::Relaxed)
}

fn mark_password_found(password_found: &AtomicBool) {
    password_found.store(true, Ordering::Relaxed);
}

fn format_password(num: u32) -> String {
    format!("{:04}", num)
}

fn log_thread_stopped(thread_id: usize, start_time: Instant) {
    println!(
        "Поток {} остановлен. Время: {:?}",
        thread_id,
        start_time.elapsed()
    );
}

fn log_password_found(thread_id: usize, password: &str, start_time: Instant) {
    println!(
        "Поток {} нашёл пароль: {}. Время: {:?}",
        thread_id,
        password,
        start_time.elapsed()
    );
}

fn log_range_completed(thread_id: usize, start_time: Instant) {
    println!(
        "Поток {} закончил перебор. Время: {:?}",
        thread_id,
        start_time.elapsed()
    );
}

fn run_single_thread_bruteforce(zip_data: &Arc<Vec<u8>>) -> Result<()> {
    for password_num in 0..10000 {
        let password = format!("{:04}", password_num);
        if try_password(&password, zip_data) {
            println!("Найден пароль: {}", password);
            return Ok(());
        }
    }
    println!("Пароль не найден");
    Ok(())
}
