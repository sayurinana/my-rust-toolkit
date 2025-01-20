# 准备工作

要使用这个工具包，需要先拉取一些依赖仓库，，，ennnnn，是的，要手动拉取到本地，

```sh
cd /path/to/workdir

git clone https://github.com/sayurinana/tracing.git
cd tracing
git checkout changed-for-0.1.0-dev.1+checkbug

cd ..

git clone https://github.com/sayurinana/eyre.git
cd eyre
git checkout changed-for-0.1.0-dev.1+checkbug
```

目录结构要大概像是下面这样，

以一个rust的package为例，它的路径是`path/to/working-project`，

```
path
 └─to
    ├─working-project
    ├─my-rust-toolkit
    ├─tracing
    │  ├─assets
    │  ├─bin
    │  ├─examples
    │  ├─tracing
    │  ├─tracing-appender
    │  ├─tracing-attributes
    │  ├─tracing-core
    │  ├─tracing-error
    │  ├─tracing-flame
    │  ├─tracing-futures
    │  ├─tracing-journald
    │  ├─tracing-log
    │  ├─tracing-macros
    │  ├─tracing-mock
    │  ├─tracing-serde
    │  ├─tracing-subscriber
    │  ├─tracing-test
    │  └─tracing-tower
    └─eyre
       ├─color-eyre
       ├─color-spantrace
       └─eyre
```

在这个package中有一些依赖要使用本地路径，`Cargo.toml`的部分配置如下

```toml
[dependencies]
my-rust-toolkit = { path = "../my-rust-toolkit" }
color-eyre = { path = "../eyre/color-eyre" }
tracing =  { path = "../tracing/tracing" }
tracing-core =  { path = "../tracing/tracing-core" }
tracing-appender = { path = "../tracing/tracing-appender" }
tracing-error =  { path = "../tracing/tracing-error" }
tracing-subscriber = { path = "../tracing/tracing-subscriber", features = ["env-filter", "local-time"] }
```

这里贴一个测试了运行成功的`Cargo.toml`，`src/main.rs`和`logs/test.2025-01-20-14.log`

`Cargo.toml`

```toml
[package]
name = "t1"
version = "0.1.0"
edition = "2021"

[dependencies]
my-rust-toolkit = { path = "../my-rust-toolkit" }
color-eyre = { path = "../eyre/color-eyre" }
tracing =  { path = "../tracing/tracing" }
tracing-core =  { path = "../tracing/tracing-core" }
tracing-appender = { path = "../tracing/tracing-appender" }
tracing-error =  { path = "../tracing/tracing-error" }
tracing-subscriber = { path = "../tracing/tracing-subscriber", features = ["env-filter", "local-time"] }
```

`src/main.rs`

```rust
use my_rust_toolkit::logger::*;

fn main() {
    println!("请自行查看控制台输出和文件内容，以确认运行结果是否正确");
    // let t = tracing_subscriber_init();
    let t = get_guard_from_init_tracing_subscriber_and_eyre(
        Level::TRACE,
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
```

`logs/test.2025-01-20-14.log`

```log
  2025-01-20T14:28:28.9634351+08:00  INFO t1: Inside the instrumented function
    at src\main.rs:31
    in t1::run_instrument with _i: 7

  2025-01-20T14:28:28.9642992+08:00 TRACE t1: This is a trace log
    at src\main.rs:37
    in t1::run_some_log

  2025-01-20T14:28:28.9647152+08:00 DEBUG t1: This is a debug log
    at src\main.rs:39
    in t1::run_some_log

  2025-01-20T14:28:28.9649526+08:00  INFO t1: This is an info log
    at src\main.rs:41
    in t1::run_some_log

  2025-01-20T14:28:28.965356+08:00  WARN t1: This is a warning log
    at src\main.rs:43
    in t1::run_some_log

  2025-01-20T14:28:28.965775+08:00 ERROR t1: This is an error log
    at src\main.rs:45
    in t1::run_some_log

```