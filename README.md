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
color-eyre = { path = "../eyre/color-eyre" }
tracing =  { path = "../tracing/tracing" }
tracing-core =  { path = "../tracing/tracing-core" }
tracing-appender = { path = "../tracing/tracing-appender" }
tracing-error =  { path = "../tracing/tracing-error" }
tracing-subscriber = { path = "../tracing/tracing-subscriber", features = ["env-filter", "local-time"] }
```