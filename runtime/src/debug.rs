pub fn debug(input: &str) {
    let flag_str = "\x1b[90m[DEBUG]\x1b[0m";
    println!("{} {}", flag_str, input);
}
pub fn info(input: &str) {
    let flag_str = "\x1b[94m[INFO]\x1b[0m";
    println!("{} {}", flag_str, input);
}
pub fn warn(input: &str) {
    let flag_str = "\x1b[93m[WARN]\x1b[0m";
    println!("{} {}", flag_str, input);
}
pub fn error(input: &str) {
    let flag_str = "\x1b[91m[ERROR]\x1b[0m";
    println!("{} {}", flag_str, input);
}
