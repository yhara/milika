/// Returns the functions needed to run the Milika program.
pub fn prelude_funcs(main_is_async: bool) -> String {
    let main_sig = if main_is_async {
        "extern(internal) chiika_main(ENV env, FN((ENV,Null)->FUTURE) cont) -> FUTURE"
    } else {
        "extern(internal) chiika_main() -> Null"
    };
    let call_uesr_main = if main_is_async {
        "return chiika_main(env, cont)"
    } else {
        "return cont(env, chiika_main())"
    };
    String::new()
        + main_sig
        + "
        extern chiika_env_push(ENV env, ANY obj) -> Null
        extern chiika_env_pop(ENV env, Int n) -> ANY
        extern chiika_env_ref(ENV env, Int n) -> Int
        extern chiika_start_tokio(Int n) -> Int
        fun chiika_start_user(ENV env, FN((ENV,Null)->FUTURE) cont) -> FUTURE {
    " + call_uesr_main
        + "
        }
        fun main() -> Int {
          chiika_start_tokio(0)
          return 0
        }
    "
}
