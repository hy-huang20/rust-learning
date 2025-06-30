# embassy-learning

[embassy 文档](https://embassy.dev/book/#_getting_started)

这里为了使用 embassy 库便直接将 embassy 项目仓库 clone 到本地，然后在 ``Cargo.toml`` 中通过相对路径寻找 dependency。项目根目录下的 ``embassy/`` 是从 [embassy 仓库](https://github.com/embassy-rs/embassy) clone 的 submodule。

注意这里是在 PC 上而非某 board 上运行，所以应该参考 embassy 仓库中 ``examples/std/`` 中关于 Cargo.toml 的配置。

目前 ``src/main.rs`` 的功能很简单，就是在命令行每秒输出一个单词 tick。