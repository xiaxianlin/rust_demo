use std::{
    cell::{Cell, RefCell},
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn foo(s: &str) {
    println!("{s}");
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{s}");
}

pub fn test1() {
    let owned = "hello".to_owned();
    let counted = Rc::new(owned);
    foo(&counted);

    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s);
}

struct HashDrop1;
struct HashDrop2;

impl Drop for HashDrop1 {
    fn drop(&mut self) {
        println!("Dropping HashDrop1");
    }
}

impl Drop for HashDrop2 {
    fn drop(&mut self) {
        println!("Dropping HashDrop2");
    }
}

struct HasTwoDrops {
    one: HashDrop1,
    two: HashDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops");
    }
}

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}

pub fn test2() {
    let _x = HasTwoDrops {
        one: HashDrop1,
        two: HashDrop2,
    };

    let _y = Foo;
    // drop(_y);
    println!("running");
    println!("y: {:?}", _y);
}

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}

pub fn test() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

trait Messager {
    fn send(&self, msg: String);
}

struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messager for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}
