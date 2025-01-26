# 更新日志

此项目的所有重要变更都将记录在此文件中。

本格式基于 [如何维护更新日志](https://keepachangelog.com/zh-CN/1.1.0/)，并且此项目遵循 [语义化版本 2.0.0](https://semver.org/lang/zh-CN/)。

其中更新日志的时间戳改用`YYYY-MM-DDThh-mm-ss`格式，
例如`2025-01-16T23-11-09`表示2025年1月16日23时11分9秒，中国时间

标记了版本的会在git提交信息的\<Scope\>块中填入

## [0.5.0] - 2025-01-26T22-14-58

### Changed

- 对`logger::get_guard_from_init_tracing_subscriber_and_eyre`添加了日志过滤器等级参数，用于在没有从环境变量中读取到值时使用

## [0.4.0] - 2025-01-26T19-38-41

### Changed

- 对`logger::get_guard_from_init_tracing_subscriber_and_eyre`取消了日志过滤器等级参数

## [0.3.0] - 2025-01-20T14-34-27

### Changed

- 对`logger::get_guard_from_init_tracing_subscriber_and_eyre`添加了日志过滤器等级参数

## [0.2.0+doc] - 2025-01-20T14-01-03

### Added

- 添加了一些文档内容

## [0.2.0] - 2025-01-20T01-56-11

### Changed

- 改用实现自定义错误代替.map_err，以便于在调用初始化后，根据失败类型调整对应参数重新调用初始化，
  - 但是还没有实际引发错误测试过的，暂时没时间测这个了，应该够用，之后有问题再修

## [0.1.0-dev.2] - 2025-01-20T01-41-04

### Changed

- 删去了`logger::get_guard_from_init_tracing_subscriber_and_eyre`的文件开关，只允许参数控制是否输出到控制台，

## [0.1.0-dev.1+checkbug] - 2025-01-20T01-30-30

### Added

- 添加了一个用来初始化tracing订阅器和安装color-eyre的东西，
- 这里面有个bug，真的现在怎么都解决不了，花了很多时间了，没时间了，存一下吧，服了也是
  - 主要是从`logger.rs`第79行开始，反正就是要么只有ab,a或者只有ab,b,就是不能同时有ab,a,b三种分支，逆天。。。