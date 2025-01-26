use std::env;

use super::super::logger::*;

#[test]
pub fn compare_level() {
    assert_eq!("TRACE", format!("{}", Level::TRACE));
    assert_eq!("DEBUG", format!("{}", Level::DEBUG));
    assert_eq!("INFO", format!("{}", Level::INFO));
    assert_eq!("WARN", format!("{}", Level::WARN));
    assert_eq!("ERROR", format!("{}", Level::ERROR));
}

#[test]
pub fn call_some_func() {
    println!("请自行查看控制台输出和文件内容，以确认运行结果是否正确");
    env::set_var("RUST_LOG", "debug");
    // let t = tracing_subscriber_init();
    let t = get_guard_from_init_tracing_subscriber_and_eyre(
        "logs",
        "test",
        "log",
        Rotation::HOURLY,
        true,
        true,
    );
    match t {
        Ok(_) => {},
        Err(_) => {}
    }
    run_some();
}

fn run_some() {
    run_instrument(7);
    run_some_log();
}

#[instrument]
fn run_instrument(_i: i32) {
    // 在函数内部记录一条信息级别的日志
    info!("Inside the instrumented function");
}

#[instrument]
fn run_some_log() {
    // 记录一条跟踪级别的日志
    trace!("This is a trace log");
    // 记录一条调试级别的日志
    debug!("This is a debug log");
    // 记录一条信息级别的日志
    info!("This is an info log");
    // 记录一条警告级别的日志
    warn!("This is a warning log");
    // 记录一条错误级别的日志
    error!("This is an error log");
}
