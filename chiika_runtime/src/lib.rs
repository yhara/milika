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

pub type ChiikaValue = *mut c_void;
#[derive(Debug)]
pub struct ContAndValue(Option<ChiikaCont>, ChiikaValue);
pub type ContFuture = Pin<Box<dyn Future<Output = ContAndValue>>>;

#[allow(improper_ctypes_definitions)]
type ChiikaCont = extern "C" fn(env: *mut ChiikaEnv, value: ChiikaValue) -> ContFuture;

#[allow(improper_ctypes)]
extern "C" {
    fn chiika_start_user(env: *mut ChiikaEnv, cont: ChiikaCont) -> ContFuture;
}

#[allow(improper_ctypes_definitions)]
pub extern "C" fn chiika_finish(_env: *mut ChiikaEnv, _v: ChiikaValue) -> ContFuture {
    Box::pin(poll_fn(move |_context| Poll::Ready(ContAndValue(None, _v))))
}

#[no_mangle]
pub extern "C" fn chiika_start_tokio(_: i64) -> i64 {
    let mut env = ChiikaEnv::new();
    let mut future: Option<_> = None;
    let poller = poll_fn(move |context| loop {
        if future.is_none() {
            future = Some(unsafe { chiika_start_user(&mut env, chiika_finish) });
        }
        let tmp = future.as_mut().unwrap().as_mut().poll(context);
        match tmp {
            Poll::Ready(ContAndValue(Some(cont), value)) => {
                let new_future = cont(&mut env, value);
                future = Some(new_future);
            }
            Poll::Ready(ContAndValue(None, _)) => return Poll::Ready(()),
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
