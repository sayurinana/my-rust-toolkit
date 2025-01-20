//! 这是项目的主库文件，包含了项目的模块声明和测试模块的配置。
//!
//! ## 模块声明
//! - `example`: 示例模块，包含示例代码。
//! - `logger`: 日志模块，提供了初始化日志记录器的功能。
//!
//! ## 测试模块
//! - `tests`: 包含了对 `logger` 模块的单元测试。
pub mod example;
pub mod logger;

#[cfg(test)]
mod tests;