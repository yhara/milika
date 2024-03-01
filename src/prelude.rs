/// Returns the functions needed to run the Milika program.
pub fn prelude_funcs(main_is_async: bool) -> String {
    let main_sig = if main_is_async {
        "extern(internal,async) chiika_main() -> int;"
    } else {
        "extern(internal) chiika_main() -> int;"
    };
    let call_uesr_main = if main_is_async {
        "return chiika_main(env, cont);"
    } else {
        "return cont(env, chiika_main());"
    };
    String::new()
        + main_sig
        + "
        extern chiika_env_push(ENV env, ANY obj) -> int;
        extern chiika_env_pop(ENV env, int n) -> ANY;
        extern chiika_env_ref(ENV env, int n) -> int;
        extern chiika_start_tokio(int n) -> int;
        fun chiika_start_user(ENV env, FN((ENV,int)->FUTURE) cont) -> FUTURE {
    " + call_uesr_main
        + "
        }
        fun main() -> int {
          chiika_start_tokio(0);
          return 0;
        }
    "
}
