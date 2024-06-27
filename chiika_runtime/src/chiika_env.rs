type ChiikaValue = i64;
type TypeId = i64;
type EnvItem = (ChiikaValue, TypeId);
type EnvFrame = Vec<EnvItem>;

#[repr(C)]
#[derive(Debug)]
pub struct ChiikaEnv {
    // Element is either 64-bit integer or 64-bit pointer.
    stack: Vec<EnvFrame>,
}

impl ChiikaEnv {
    pub fn new() -> ChiikaEnv {
        ChiikaEnv {
            stack: vec![vec![]],
        }
    }
}

/// Push a frame to the stack.
#[no_mangle]
pub extern "C" fn chiika_env_push_frame(env: *mut ChiikaEnv) {
    unsafe {
        let v = vec![]; //std::iter::repeat(None).take(size).collect();
        (*env).stack.push(v);
    }
}

/// Push an item to the current frame.
#[no_mangle]
pub extern "C" fn chiika_env_push(env: *mut ChiikaEnv, value: ChiikaValue, type_id: TypeId) {
    unsafe {
        let frame = (*env).stack.last_mut().unwrap();
        frame.push((value, type_id));
    }
}

/// Pop last frame from the stack and returns its first item.
/// Panics if the frame size is not equal to n.
#[no_mangle]
pub extern "C" fn chiika_env_pop_frame(env: *mut ChiikaEnv, n: i64) -> i64 {
    let frame = unsafe { (*env).stack.pop() };
    match frame {
        Some(v) => {
            if v.len() != n as usize {
                panic!(
                    "[BUG;chiika_env_pop_frame] Frame size mismatch: expected size={}, but got size={}",
                    n,
                    v.len()
                );
            }
            v.first().unwrap().0
        }
        None => panic!("[BUG;chiika_env_pop_frame] Stack underflow: no frame to pop"),
    }
}

/// Peek the n-th last item in the current frame.
#[no_mangle]
pub extern "C" fn chiika_env_ref(env: *mut ChiikaEnv, n: i64, expected_type_id: TypeId) -> i64 {
    let stack = unsafe { &(*env).stack };
    let frame = stack.last().unwrap();
    if n > (frame.len() as i64) - 1 {
        panic!("[BUG;chiika_env_ref] Index out of bounds: n={}, frame_size={}", n, frame.len());
    }
    let (value, type_id) = frame[frame.len() - 1 - (n as usize)];
    if type_id != expected_type_id {
        panic!(
            "[BUG;chiika_env_ref] Type mismatch: expected type_id={}, but got type_id={}",
            expected_type_id, type_id
        );
    }
    value
}
