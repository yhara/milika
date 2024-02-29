/// Returns the functions needed to run the Milika program.
pub fn prelude_funcs(main_is_async: bool) -> String {
    let s = "
        extern chiika_env_push(ENV env, ANY obj) -> int;
        extern chiika_env_pop(ENV env, int n) -> ANY;
        extern chiika_env_ref(ENV env, int n) -> int;
        extern chiika_start_tokio(int n) -> int;
        fun main() -> int {
          chiika_start_tokio(0);
          return 0;
        }
    ";
    let more = if main_is_async {
        "
            fun chiika_start_user__sync(ENV env, CONT cont) -> FUTURE {
              return chiika_main(env, cont);
            }
        "
    } else {
        "
            fun chiika_start_user__async(ENV env, CONT cont) -> FUTURE {
              return cont(env, chiika_main());
            }
        "
    };
    s.to_owned() + more
}
