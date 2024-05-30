mod test1;
mod test2;
mod test3;

async fn do_something() {
    println!("go go go !");
}

async fn hello_cat() {
    println!("hello, kitty!");
}

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

pub fn test() {
    test3::test();
}
