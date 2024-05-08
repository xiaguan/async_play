use std::{
    future::{Future, IntoFuture},
    task::Poll,
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Time expired");
            Poll::Ready(())
        } else {
            println!("Time is not reached");
            // 如果不加入下面这个代码，会卡死
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// 关于async有两种用法，
// 1. async fn
// 2. async block
//
// Async fn
// Returns a type that implements `Future<Output = u8>`
async fn foo() -> u8 {
    5
}

// Async block
fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

async fn borrow_x(x: &u8) -> u8 {
    x + 1
}

// Life time example
// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     borrow_x(&x) // ERROR: `x` does not live long enough
// }
// 区别在于是否.await
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&5).await
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };
    future.await;
}
