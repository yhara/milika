mod chiika_env;
use crate::chiika_env::ChiikaEnv;
mod async_functions;
mod sync_functions;
use std::ffi::c_void;
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

pub type VoidFuture = Pin<Box<dyn Future<Output = ()>>>;
#[allow(improper_ctypes_definitions)]
type ChiikaCont = extern "C" fn(env: *mut ChiikaEnv, value: *mut c_void) -> VoidFuture;

#[allow(improper_ctypes)]
extern "C" {
    fn chiika_start_user(env: *mut ChiikaEnv, cont: ChiikaCont) -> VoidFuture;
}

#[allow(improper_ctypes_definitions)]
pub extern "C" fn chiika_finish(_env: *mut ChiikaEnv, _: *mut c_void) -> VoidFuture {
    Box::pin(poll_fn(|_context| Poll::Ready(())))
}

#[no_mangle]
pub extern "C" fn chiika_start_tokio(_: i64) -> i64 {
    let mut env = ChiikaEnv::new();
    let mut future: Option<_> = None;
    let poller = poll_fn(move |context| {
        if future.is_none() {
            future = Some(unsafe { chiika_start_user(&mut env, chiika_finish) });
        }
        future.as_mut().unwrap().as_mut().poll(context)
    });
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(poller);

    // Q: Need this?
    // sleep(Duration::from_millis(50)).await;

    0
}
