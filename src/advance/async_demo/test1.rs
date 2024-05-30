use futures::{executor::block_on, join};

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: "菊花台".to_string(),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "菊花残，满地伤~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

fn run1() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn run2() {
    let f1 = learn_and_sing();
    let f2 = dance();

    join!(f1, f2);
}

pub fn test() {
    // run1();
    block_on(run2());
}
