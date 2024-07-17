mod chiika_env;
use crate::chiika_env::ChiikaEnv;
mod async_functions;
mod sync_functions;
use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::task::Poll;

//async fn read(_: i64) -> i64 {
//    match fs::read_to_string("count.txt").await {
//        Ok(s) => s.parse().unwrap(),
//        Err(_) => 0,
//    }
//}
//
//async fn write(n: i64) -> i64 {
//    let _ = fs::write("count.txt", n.to_string()).await;
//    0
//}

pub type ChiikaValue = u64;

#[allow(improper_ctypes_definitions)]
pub type ContFuture = Box<dyn Future<Output = ChiikaValue> + Unpin>;

#[allow(improper_ctypes_definitions)]
type ChiikaCont = extern "C" fn(env: *mut ChiikaEnv, value: ChiikaValue) -> ContFuture;

#[allow(improper_ctypes_definitions)]
type ChiikaThunk = extern "C" fn(env: *mut ChiikaEnv, cont: ChiikaCont) -> ContFuture;

#[allow(improper_ctypes)]
extern "C" {
    fn chiika_start_user(env: *mut ChiikaEnv, cont: ChiikaCont) -> ContFuture;
}

#[allow(improper_ctypes_definitions)]
extern "C" fn chiika_finish(env: *mut ChiikaEnv, _v: ChiikaValue) -> ContFuture {
    unsafe {
        (*env).cont = None;
    }
    Box::new(poll_fn(move |_context| Poll::Ready(_v)))
}

#[no_mangle]
pub extern "C" fn chiika_start_tokio(_: i64) -> i64 {
    let mut env = ChiikaEnv::new();
    let mut future: Option<_> = None;
    let poller = poll_fn(move |context| loop {
        if future.is_none() {
            future = Some(unsafe { chiika_start_user(&mut env, chiika_finish) });
        }
        let pinned = Pin::new(future.as_mut().unwrap());
        let tmp = pinned.poll(context);
        match tmp {
            Poll::Ready(value) => {
                if let Some(cont) = env.cont {
                    let new_future = cont(&mut env, value);
                    future = Some(new_future);
                } else {
                    return Poll::Ready(());
                }
            }
            Poll::Pending => return Poll::Pending,
        }
    });
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(poller);

    // Q: Need this?
    // sleep(Duration::from_millis(50)).await;

    0
}
