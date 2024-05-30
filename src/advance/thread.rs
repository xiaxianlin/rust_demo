use std::{
    cell::RefCell,
    ops::Sub,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, Barrier, Condvar, Mutex, Once,
    },
    thread::{self, spawn, JoinHandle, LocalKey},
    time::{Duration, Instant},
};

pub fn test10() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn test9() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn test8() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

struct Foo;
impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}

thread_local! {
    static FOO: RefCell<u32> = RefCell::new(1);
}

struct Bar {
    foo: &'static LocalKey<RefCell<u32>>,
}

impl Bar {
    fn new() -> Self {
        Self { foo: &FOO }
    }
}

pub fn test7() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

pub fn test6() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !&*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

static mut VAL: usize = 0;
static INIT: Once = Once::new();

pub fn test5() {
    let handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

pub fn test4() {
    let (send, recv) = mpsc::channel();
    for i in 0..3 {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

const N_TIMES: u64 = 10_000_000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

pub fn test3() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}", Instant::now().sub(s));
}

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100; // A
        }
        READY.store(true, Ordering::Release); // B: 内存屏障 ↑
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {} // C: 内存屏障 ↓

        assert_eq!(100, unsafe { DATA }); // D
    })
}

pub fn test() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
