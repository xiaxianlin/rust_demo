use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn show(pt: &Point) {
    println!("({}, {})", pt.x, pt.y);
}

fn show_as_ref<T: AsRef<Point>>(pt: T) {
    let pt: &Point = pt.as_ref();
    println!("({}, {})", pt.x, pt.y);
}

trait Calculate {
    fn add(&self, l: u64, r: u64) -> u64;
    fn mul(&self, l: u64, r: u64) -> u64;
}

struct Modulo(pub u64);

impl Calculate for Modulo {
    fn add(&self, l: u64, r: u64) -> u64 {
        (l + r) % self.0
    }
    fn mul(&self, l: u64, r: u64) -> u64 {
        (l * r) % self.0
    }
}

fn add_four<T: Borrow<i32>>(v: T) -> i32 {
    v.borrow() + 4
}

pub fn test() {
    let pt = Point { x: 1, y: 2 };
    let box_pt = Box::new(Point { x: 3, y: 4 });

    show(&pt);
    show_as_ref(&box_pt);

    let array = [0u64; 5];
    let slice = &array[1..3];

    let mut vec = Vec::<u64>::with_capacity(8);
    for i in 0..5 {
        vec.push(i);
    }
    let slice = &vec[1..3];

    let mod3 = Modulo(3);

    let tobj: &dyn Calculate = &mod3;
    let result = tobj.add(2, 2);
    assert_eq!(result, 1);

    assert_eq!(add_four(&2), 6);
    assert_eq!(add_four(2), 6);
}

fn test1() {
    let rc1: Rc<u64> = Rc::new(42);
    let rc2 = rc1.clone();
    let wk = Rc::downgrade(&rc1);
}

pub fn test2() {
    let rc: RefCell<u64> = RefCell::new(42);
    // let b1 = rc.borrow_mut();

    *rc.borrow_mut() = 21;

    println!("{:?}", rc);
}

pub fn test3() {
    let pt = Rc::new(RefCell::new(Point { x: 1, y: 2 }));
    let a = Rc::clone(&pt);

    *pt.borrow_mut() = Point { x: 10, y: 20 };

    println!("{:?}", a);
}
