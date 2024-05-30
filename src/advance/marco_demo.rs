#![allow(dead_code)]
#![allow(unused_variables)]

use demo_macro::{HelloMacro, MyDefault};
use rust_demo::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

#[derive(MyDefault, Debug)]
struct SomeData(u32, String);

#[derive(MyDefault, Debug)]
struct User {
    name: String,
    data: SomeData,
}

pub fn test() {
    Sunfei::hello_macro();

    let user = User::default();
    println!("user = {:?}", user);
}
