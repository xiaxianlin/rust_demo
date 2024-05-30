use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;

#[derive(Clone)]
struct NInfo {
    n_times: u64,
    n_threads: usize,
}

fn add_n_times(n: u64, counter: Arc<Mutex<u64>>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }
    })
}

fn add_n_times_atomic(n: u64, counter: Arc<AtomicU64>) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            counter.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn mutex_test(test_info: NInfo) -> std::time::Duration {
    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let mut threads = Vec::with_capacity(test_info.n_threads);
    let s = Instant::now();
    for _ in 0..test_info.n_threads {
        threads.push(add_n_times(test_info.n_times, counter.clone()))
    }

    for thread in threads {
        thread.join().unwrap();
    }
    let use_time: std::time::Duration = Instant::now().sub(s);
    assert_eq!(
        test_info.n_times * test_info.n_threads as u64,
        *counter.lock().unwrap()
    );
    use_time
}

fn atomic_test(test_info: NInfo) -> std::time::Duration {
    let counter: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let mut threads = Vec::with_capacity(test_info.n_threads);
    let s = Instant::now();
    for _ in 0..test_info.n_threads {
        threads.push(add_n_times_atomic(test_info.n_times, counter.clone()))
    }

    for thread in threads {
        thread.join().unwrap();
    }
    let use_time = Instant::now().sub(s);
    assert_eq!(
        test_info.n_times * test_info.n_threads as u64,
        counter.load(Ordering::Relaxed)
    );
    use_time
}

fn test(test_info: NInfo) {
    let mut d_time = 0f64;
    for _ in 0..10 {
        let time_mutex = mutex_test(test_info.clone());
        let time_atomic = atomic_test(test_info.clone());
        d_time += (time_mutex.as_micros() as f64) / (time_atomic.as_micros() as f64);
    }
    d_time /= 10 as f64;
    println!(
        "start test times: {}, threads: {}, time_mutex/time_atomic: {d_time:?}",
        test_info.n_times, test_info.n_threads
    );
}

pub fn run() {
    test(NInfo {
        n_times: 100,
        n_threads: 10,
    });
    test(NInfo {
        n_times: 100,
        n_threads: 1_000,
    });
    test(NInfo {
        n_times: 500,
        n_threads: 10,
    });
    test(NInfo {
        n_times: 500,
        n_threads: 1_000,
    });
    test(NInfo {
        n_times: 800,
        n_threads: 10,
    });
    test(NInfo {
        n_times: 800,
        n_threads: 1_000,
    });
    test(NInfo {
        n_times: 1000,
        n_threads: 10,
    });
    test(NInfo {
        n_times: 1000,
        n_threads: 1_000,
    });
    test(NInfo {
        n_times: 1500,
        n_threads: 10,
    });
    test(NInfo {
        n_times: 1500,
        n_threads: 1_000,
    });
}
