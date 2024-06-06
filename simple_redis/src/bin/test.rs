use futures::future::poll_fn;
use futures::Future;
use simple_redis::Delay;
use std::{
    pin::Pin,
    task::Poll,
    time::{Duration, Instant},
};

// #[tokio::main]
// async fn main() {
//     let when = Instant::now() + Duration::from_millis(10);
//     let future = Delay { when };

//     // 运行并等待 Future 的完成
//     let out = future.await;

//     // 判断 Future 返回的字符串是否是 "done"
//     assert_eq!(out, "done");
// }
//

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut delay = Some(Delay { when, waker: None });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        tokio::spawn(async move {
            delay.await;
        });
        Poll::Ready(())
    })
    .await;
}
