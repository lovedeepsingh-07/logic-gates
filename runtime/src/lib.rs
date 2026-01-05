mod cli;
mod debug;
mod error;
mod logic;

use cli::*;
use debug::*;

#[cxx::bridge(namespace = "rt")]
mod ffi {
    extern "Rust" {
        fn cli() -> Result<()>;
    }

    #[namespace = "rt::debug"]
    extern "Rust" {
        fn debug(input: &str);
        fn info(input: &str);
        fn warn(input: &str);
        fn error(input: &str);
    }
}
