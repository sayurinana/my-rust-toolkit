//! 日志模块提供了初始化日志记录器的功能，使用 `tracing` 和 `color_eyre` 库来实现日志记录和错误处理。
//!
//! ## 主要功能
//! - 初始化日志记录器，支持控制台输出和文件输出。
//! - 提供了灵活的日志级别配置。
//! - 支持日志文件的滚动存储。
//!
//! ## 使用示例
//! ```rust
//! use my_rust_toolkit::logger::get_guard_from_init_tracing_subscriber_and_eyre;
//! use tracing_appender::rolling::Rotation;
//! use tracing::Level;
//! 
//! # 或者是全部导入，因为其实东西不多就是一些状态结构和常用的日志宏
//! use my_rust_toolkit::logger::*;
//!
//! fn main() {
//!     let t = get_guard_from_init_tracing_subscriber_and_eyre(
//!         "logs",
//!         "myapp",
//!         "log",
//!         Rotation::HOURLY,
//!         true,
//!         true,
//!     );
//!     match t {
//!         Ok(_) => {}
//!         Err(_) => {}
//!     }
//!     // 你的代码逻辑
//! }
//! ```
// #![allow(unused_imports)]
use anyhow::Result;
use color_eyre::eyre;
use std::fs;
use tracing_appender::{non_blocking, rolling::RollingFileAppender};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Registry,
};

pub use tracing::{debug, error, info, instrument, trace, warn, Level};
pub use tracing_appender::rolling::Rotation;

/// 初始化 `tracing` 日志记录器和 `color_eyre` 错误处理器。
///
/// # 参数
/// - `_log_filter_level`: 日志过滤器级别
/// - `_logs_dir`: 日志文件存储的目录路径。
/// - `_logfile_prefix`: 日志文件名的前缀。
/// - `_logfile_suffix`: 日志文件名的后缀。
/// - `_rotation`: 日志文件的滚动策略。
/// - `_enable_formatting_layer`: 是否启用控制台格式化输出层。
/// - `_install_eyre_color`: 是否安装 `color_eyre` 的颜色支持。
///
/// # 返回值
/// 返回一个 `Result`，包含 `non_blocking::WorkerGuard` 或 `ErrorFromInitTracingSubscriberAndEyre` 错误。
///
/// # 示例
/// ```rust
/// use logger::get_guard_from_init_tracing_subscriber_and_eyre;
/// use tracing_appender::rolling::Rotation;
///
/// fn main() {
///     env::set_var("RUST_LOG", "debug");
///     let t = get_guard_from_init_tracing_subscriber_and_eyre(
///         "logs",
///         "myapp",
///         "log",
///         Rotation::HOURLY,
///         true,
///         true,
///     );
///     match t {
///         Ok(_) => {}
///         Err(_) => {}
///     }
///     // 你的代码逻辑
/// }
/// ```
pub fn get_guard_from_init_tracing_subscriber_and_eyre(
    _logs_dir: &str,
    _logfile_prefix: &str,
    _logfile_suffix: &str,
    _rotation: Rotation,
    _enable_formatting_layer: bool,
    _install_eyre_color: bool,
) -> Result<non_blocking::WorkerGuard, ErrorFromInitTracingSubscriberAndEyre> {
    // if !enable_formatting_layer && !enable_file_layer {
    //     return Err(anyhow!(
    //         "既不开格式化控制台输出，也不开文件输出，你开日志干嘛？？？"
    //     ));
    // }

    // 尝试从环境变量中解析日志级别，如果失败则默认为"info"级别
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 获取本地时间偏移量
    let offset_time = OffsetTime::local_rfc_3339()?;

    let mut formatting_layer = None;
    if _enable_formatting_layer {
        // 创建一个格式化层，用于控制台日志输出，包含漂亮打印和本地时间戳
        formatting_layer = Some(
            fmt::layer()
                .pretty()
                // .with_ansi(false)
                .with_timer(offset_time.clone())
                .with_writer(std::io::stderr),
        );
    }

    // 尝试创建日志目录，如果失败则抛出错误
    fs::create_dir_all(_logs_dir)?;

    // 构建一个滚动文件追加器，用于日志文件的滚动存储
    let file_appender = RollingFileAppender::builder()
        .rotation(_rotation) // 每小时滚动一次日志文件
        .filename_prefix(_logfile_prefix) // 日志文件名前缀为`myapp.`
        .filename_suffix(_logfile_suffix) // 日志文件名后缀为`.log`
        .build(_logs_dir)?; // 在`logs`目录下存储日志文件

    // 将文件追加器包装为非阻塞模式，以提高性能
    let (non_blocking_appender, guard) = non_blocking(file_appender);
    // 创建一个用于文件日志输出的格式化层，禁用ANSI颜色和包含本地时间戳
    let file_layer = Some(
        fmt::layer()
            .pretty()
            .with_ansi(false)
            .with_timer(offset_time.clone())
            .with_writer(non_blocking_appender),
    );

    if _install_eyre_color {
        // 安装color-eyre，以提供更丰富的错误处理和报告
        color_eyre::install()?;
    }

    match formatting_layer {
        Some(a) => {
            Registry::default()
                .with(env_filter)
                // ErrorLayer 允许 color-eyre 获取 span 的信息
                .with(ErrorLayer::default())
                .with(file_layer)
                .with(a)
                .init();
        }
        None => {
            Registry::default()
                .with(env_filter)
                // ErrorLayer 允许 color-eyre 获取 span 的信息
                .with(ErrorLayer::default())
                .with(file_layer)
                .init();
        }
    };
    return Ok(guard);
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorFromInitTracingSubscriberAndEyre {
    #[error("获取日期时间偏移量错误")]
    TimeOffsetGetError(#[from] time::error::IndeterminateOffset),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("日志文件追加器初始化错误")]
    AppenderInitError(#[from] tracing_appender::rolling::InitError),
    #[error("安装color-eyre失败")]
    EyreInstallFailure(#[from] eyre::Report),
}
