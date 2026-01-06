mod cli;
mod constants;
mod debug;
mod error;
mod logic;
mod watcher;

use cli::*;
use debug::*;
use logic::*;
use watcher::*;

#[cxx::bridge(namespace = "rt")]
mod ffi {
    extern "Rust" {
        fn cli() -> Result<String>;
    }

    #[namespace = "rt::logic"]
    extern "Rust" {
        #[rust_name = "logic_test"]
        fn test() -> Result<()>;
    }

    #[namespace = "rt::watcher"]
    extern "Rust" {
        #[rust_name = "watcher_run"]
        fn run(some_path: String) -> Result<()>;
        #[rust_name = "watcher_poll_events"]
        fn poll_events() -> Result<()>;
    }

    #[namespace = "rt::debug"]
    extern "Rust" {
        fn debug(input: &str);
        fn info(input: &str);
        fn warn(input: &str);
        fn error(input: &str);
    }
}
