use crate::chiika_env::ChiikaEnv;
use crate::{ChiikaCont, ContAndValue, ContFuture};
use std::future::{poll_fn, Future};
use std::task::Poll;
use std::time::Duration;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn sleep_sec(n: i64, _env: &'static mut ChiikaEnv, cont: ChiikaCont) -> ContFuture {
    async fn sleep_sec(n: i64) -> i64 {
        // Hand written part (all the rest will be macro-generated)
        tokio::time::sleep(Duration::from_secs(n as u64)).await;
        0
    }
    let mut future = Box::pin(sleep_sec(n));
    Box::pin(poll_fn(move |ctx| match future.as_mut().poll(ctx) {
        Poll::Ready(v) => {
            let as_void = v as *const i64 as *mut std::ffi::c_void;
            Poll::Ready(ContAndValue(Some(cont), as_void))
        }
        Poll::Pending => Poll::Pending,
    }))
}
