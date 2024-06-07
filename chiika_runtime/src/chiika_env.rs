#[repr(C)]
#[derive(Debug)]
pub struct ChiikaEnv {
    // Element is either 64-bit integer or 64-bit pointer.
    stack: Vec<i64>,
}

impl ChiikaEnv {
    pub fn new() -> ChiikaEnv {
        ChiikaEnv { stack: vec![] }
    }
}

/// Push an item to the stack.
#[no_mangle]
pub extern "C" fn chiika_env_push(env: *mut ChiikaEnv, item: i64) {
    unsafe {
        (*env).stack.push(item);
    }
}

/// Pop last n items from the stack and returns the last popped item (i.e. the n-th item)
#[no_mangle]
pub extern "C" fn chiika_env_pop(env: *mut ChiikaEnv, n: i64) -> i64 {
    unsafe {
        let mut item = 0;
        let mut popped = 0;
        for _ in 0..n {
            match (*env).stack.pop() {
                Some(x) => item = x,
                None => {
                    panic!("[BUG;chiika_env_pop] Stack underflow: tried to pop {} items, but only {} items left", n, popped);
                }
            }
            popped += 1;
        }
        item
    }
}

/// Peek the n-th item (from the stack top)
#[no_mangle]
pub extern "C" fn chiika_env_ref(env: *mut ChiikaEnv, n: i64) -> i64 {
    let stack = unsafe { &(*env).stack };
    if n > (stack.len() as i64) - 1 {
        panic!("[BUG;chiika_env_ref] Stack underflow: tried to peek {}-th item, but only {} items left", n, stack.len());
    }
    stack[stack.len() - 1 - (n as usize)]
}
