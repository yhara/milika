use crate::ChiikaCont;
type ChiikaValue = i64;
type TypeId = i64;
type EnvItem = (ChiikaValue, TypeId);
type EnvFrame = Vec<Option<EnvItem>>;

#[repr(C)]
#[derive(Debug)]
pub struct ChiikaEnv {
    // Element is either 64-bit integer or 64-bit pointer.
    stack: Vec<EnvFrame>,
    pub cont: Option<ChiikaCont>,
}

impl ChiikaEnv {
    pub fn new() -> ChiikaEnv {
        ChiikaEnv {
            stack: vec![vec![]],
            cont: None,
        }
    }

    fn current_frame(&mut self) -> &EnvFrame {
        match self.stack.last() {
            Some(v) => v,
            None => panic!("[BUG;ChiikaEnv::current_frame] Stack underflow: no frame there"),
        }
    }

    fn current_frame_mut(&mut self) -> &mut EnvFrame {
        match self.stack.last_mut() {
            Some(v) => v,
            None => panic!("[BUG;ChiikaEnv::current_frame_mut] Stack underflow: no frame there"),
        }
    }
}

/// Push a frame to the stack.
#[no_mangle]
pub extern "C" fn chiika_env_push_frame(env: *mut ChiikaEnv, size: i64) {
    unsafe {
        let v = std::iter::repeat(None).take(size as usize).collect();
        (*env).stack.push(v);
    }
}

/// Push an item to the current frame.
#[no_mangle]
pub extern "C" fn chiika_env_set(env: *mut ChiikaEnv, n: i64, value: ChiikaValue, type_id: TypeId) {
    let frame = unsafe { (*env).current_frame_mut() };
    if n > (frame.len() as i64) - 1 {
        panic!(
            "[BUG;chiika_env_set] Index out of bounds: n={}, frame_size={}",
            n,
            frame.len()
        );
    }
    frame[n as usize] = Some((value, type_id));
}

/// Pop last frame from the stack and returns its first item.
/// Panics if the frame size is not as expected.
#[no_mangle]
pub extern "C" fn chiika_env_pop_frame(env: *mut ChiikaEnv, expected_len: i64) -> i64 {
    let frame = unsafe { (*env).stack.pop() };
    match frame {
        Some(v) => {
            if v.len() != expected_len as usize {
                panic!(
                    "[BUG;chiika_env_pop_frame] Frame size mismatch: expected size={}, but got size={}",
                    expected_len,
                    v.len()
                );
            }
            v.first().unwrap().unwrap().0
        }
        None => panic!("[BUG;chiika_env_pop_frame] Stack underflow: no frame to pop"),
    }
}

/// Peek the n-th last item in the current frame.
#[no_mangle]
pub extern "C" fn chiika_env_ref(env: *mut ChiikaEnv, n: i64, expected_type_id: TypeId) -> i64 {
    let frame = unsafe { (*env).current_frame() };
    if n > (frame.len() as i64) - 1 {
        panic!(
            "[BUG;chiika_env_ref] Index out of bounds: n={}, frame_size={}",
            n,
            frame.len()
        );
    }
    let Some((value, type_id)) = frame[n as usize] else {
        panic!("[BUG;chiika_env_ref] value not set at index {n}");
    };
    if type_id != expected_type_id {
        panic!(
            "[BUG;chiika_env_ref] Type mismatch: expected type_id={} for index {n} but got type_id={}",
            expected_type_id, type_id
        );
    }
    value
}
