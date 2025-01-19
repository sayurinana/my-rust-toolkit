use anyhow::{anyhow, Result};
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


pub use tracing_appender::rolling::Rotation;

pub fn get_guard_from_init_tracing_subscriber_and_eyre(
    logs_dir: &str,
    logfile_prefix: &str,
    logfile_suffix: &str,
    rotation: Rotation,
    enable_file_layer: bool,
    enable_formatting_layer: bool,
    install_eyre_color: bool,
) -> Result<Option<non_blocking::WorkerGuard>> {
    if !enable_formatting_layer && !enable_file_layer {
        return Err(anyhow!(
            "既不开格式化控制台输出，也不开文件输出，你开日志干嘛？？？"
        ));
    }

    // 尝试从环境变量中解析日志级别，如果失败则默认为"info"级别
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 获取本地时间偏移量
    let offset_time = OffsetTime::local_rfc_3339().map_err(|err| anyhow!("{:?}", err))?;

    let mut formatting_layer = None;
    if enable_formatting_layer {
        // 创建一个格式化层，用于控制台日志输出，包含漂亮打印和本地时间戳
        formatting_layer = Some(
            fmt::layer()
                .pretty()
                .with_ansi(false)
                .with_timer(offset_time.clone())
                .with_writer(std::io::stderr),
        );
    }

    let result;
    let mut file_layer = None;
    if enable_file_layer {
        // 尝试创建日志目录，如果失败则抛出错误
        fs::create_dir_all(logs_dir).map_err(|_| anyhow!("创建日志目录失败！"))?;

        // 构建一个滚动文件追加器，用于日志文件的滚动存储
        let file_appender = RollingFileAppender::builder()
            .rotation(rotation) // 每小时滚动一次日志文件
            .filename_prefix(logfile_prefix) // 日志文件名前缀为`myapp.`
            .filename_suffix(logfile_suffix) // 日志文件名后缀为`.log`
            .build(logs_dir)
            .map_err(|err| anyhow!("{:?}", err))?; // 在`logs`目录下存储日志文件

        // 将文件追加器包装为非阻塞模式，以提高性能
        let (non_blocking_appender, guard) = non_blocking(file_appender);
        // 创建一个用于文件日志输出的格式化层，禁用ANSI颜色和包含本地时间戳
        file_layer = Some(
            fmt::layer()
                .pretty()
                .with_ansi(false)
                .with_timer(offset_time.clone())
                .with_writer(non_blocking_appender),
        );
        result = Some(guard);
    } else {
        result = None;
    }

    if install_eyre_color {
        // 安装color-eyre，以提供更丰富的错误处理和报告
        color_eyre::install().map_err(|err| anyhow!("{:?}", err))?;
    }

    if let (Some(a), Some(b)) = (formatting_layer, file_layer) {
        Registry::default()
            .with(env_filter)
            // ErrorLayer 允许 color-eyre 获取 span 的信息
            .with(ErrorLayer::default())
            .with(a)
            .with(b)
            .init();
    };
    if let (Some(a), None) = (formatting_layer, file_layer) {
        Registry::default()
            .with(env_filter)
            // ErrorLayer 允许 color-eyre 获取 span 的信息
            .with(ErrorLayer::default())
            .with(a)
            // .with(b)
            .init();
    };
    if let (None, Some(b)) = (formatting_layer, file_layer) {
        Registry::default()
            .with(env_filter)
            // ErrorLayer 允许 color-eyre 获取 span 的信息
            .with(ErrorLayer::default())
            // .with(a)
            .with(b)
            .init();
    }
    // let t = match (formatting_layer, file_layer) {
    //     (Some(a), Some(b)) => {
    //         Registry::default()
    //             .with(env_filter)
    //             // ErrorLayer 允许 color-eyre 获取 span 的信息
    //             .with(ErrorLayer::default())
    //             .with(b)
    //             .with(a)
    //             .init();
    //     }
    //     (Some(a), None) => {
    //         Registry::default()
    //             .with(env_filter)
    //             // ErrorLayer 允许 color-eyre 获取 span 的信息
    //             .with(ErrorLayer::default())
    //             .with(a)
    //             .init();
    //     }
    //     (None, Some(b)) => {
    //         Registry::default()
    //             .with(env_filter)
    //             // ErrorLayer 允许 color-eyre 获取 span 的信息
    //             .with(ErrorLayer::default())
    //             .with(b)
    //             .init();
    //     }
    //     (None, None) => {
    //         // Registry::default()
    //         //     .with(env_filter)
    //         //     // ErrorLayer 允许 color-eyre 获取 span 的信息
    //         //     .with(ErrorLayer::default())
    //         //     .init();
    //         ()
    //     }
    // };
    return Ok(result);
}

