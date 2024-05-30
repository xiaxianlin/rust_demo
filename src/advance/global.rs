use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        OnceLock,
    },
    thread,
};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

struct Factory {
    factory_id: usize,
}

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
struct Logger;

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created...");
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}

pub fn test() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}
