use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (_tx1, mut rx1) = mpsc::channel(128);
    let (_tx2, mut rx2) = mpsc::channel(128);
    let (_tx3, mut rx3) = mpsc::channel(128);

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}
