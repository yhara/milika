use crate::chiika_env::ChiikaEnv;
use crate::VoidFuture;
use std::future::{poll_fn, Future};
use std::task::Poll;
use std::time::Duration;

type ChiikaCont = extern "C" fn(env: &mut ChiikaEnv, value: i64);

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn sleep_sec(env: &'static mut ChiikaEnv, cont: ChiikaCont, n: i64) -> VoidFuture {
    async fn sleep_sec(n: i64) {
        // Hand written part (all the rest will be macro-generated)
        tokio::time::sleep(Duration::from_secs(n as u64)).await;
    }
    let mut future = Box::pin(sleep_sec(n));
    Box::pin(poll_fn(move |ctx| match future.as_mut().poll(ctx) {
        Poll::Ready(_) => {
            cont(env, n);
            Poll::Ready(())
        }
        Poll::Pending => Poll::Pending,
    }))
}
