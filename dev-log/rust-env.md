# rust 环境配置

## 1. 问题描述

在普通用户权限下尝试 `cargo install cargo-expand` 下载时发现以下问题：

- 普通用户命令行找不到 `rustup`，root 正常

- 执行 `cargo install` 和 `sudo cargo install` 命令行的显示有所不同，前者显示 `Updating crates.io index`，而后者显示 `Updating ustc index`

- 环境有些混乱。在 `/usr/bin/` 和 `/root/.cargo/bin/` 下面找到了 `cargo` 二进制文件而在 `/home/hy-huang/.cargo/` 下面没找到

## 2. 问题分析

`/usr/bin/` 下的 `cargo` 应该是用 `apt` 这样的工具安装的。

关于 `/root/.cargo/bin/` 下的 `cargo`。root 下的 rust 环境应该是参考了[rCore-2025S 课程实验指导书](https://learningos.cn/rCore-Tutorial-Guide-2025S/0setup-devel-env.html)中的步骤配置的，因此用的是 `ustc` 镜像源。之前做实验以及跑 rCore-N 都是开了 `su` 基于这个环境做的，**没问题不要动，没问题不要动，没问题不要动**。`/root/.cargo/bin/` 下的内容：

```
cargo         cargo-nm        cargo-size     rust-analyzer  rust-ld       rust-objdump   rustc
cargo-clippy  cargo-objcopy   cargo-strip    rust-ar        rust-lld      rust-profdata  rustdoc
cargo-cov     cargo-objdump   clippy-driver  rust-cov       rust-lldb     rust-readobj   rustfmt
cargo-fmt     cargo-profdata  just           rust-gdb       rust-nm       rust-size      rustup
cargo-miri    cargo-readobj   rls            rust-gdbgui    rust-objcopy  rust-strip
```

`/home/hy-huang/.cargo/bin/` 下没有 `cargo` 也没有 `rustc`：

```
cargo-cov      cargo-objdump   cargo-size   rust-as   rust-lld      rust-objdump   rust-size
cargo-nm       cargo-profdata  cargo-strip  rust-cov  rust-nm       rust-profdata  rust-strip
cargo-objcopy  cargo-readobj   rust-ar      rust-ld   rust-objcopy  rust-readobj
```

所以平常在普通用户权限下用的应该都是 `/usr/bin/` 下的 `cargo`，至于 `/home/hy-huang/.cargo/bin/` 下为什么会有上述这些文件，可能是由于之前在普通用户权限下执行过 `cargo install cargo-binutils` 这样的命令从而安装到 `/home/hy-huang/.cargo/bin/` 下。

>### `cargo`, `sudo cargo` 和 `su && cargo` 三者的区别
>
>以下表格基于我自己的电脑环境：
>
>|命令|当前身份|读取的配置文件|
>|---|---|---|
>|`cargo`|`hy-huang`|`/home/hy-huang/.cargo/config.toml`|
>|`sudo cargo`|临时 `root`|`/root/.cargo/config.toml`|
>|`su && cargo`|`root`|`/root/.cargo/config.toml`|

>### `su` 和 `sudo` 的一些区别
>
>||`Switch User`|`SuperUser Do`|
>|---|---|---|
>|密码|root 密码|当前普通用户密码|
>|权限|彻底切换身份为另外一个用户|临时借用 root 身份执行单条命令|
>
>需要理解一个问题：在普通用户权限下执行 `sudo vim ~/.cargo/config.toml` 时找的是普通用户的还是 root 用户的 config.toml。答案是找的普通用户的文件，但以 root 用户的身份强制修改。因为终端 shell 会先解析符号 `~` 然后把命令交给 sudo 执行。
>
>### 除非必须否则谨慎使用 `sudo vim`
>
>最好不要这么做 `sudo vim ~/.cargo/config.toml`。通过前面的踩坑也意识到这么做会造成**权限的污染**，使得后续不能以普通用户的身份去修改这个文件了，因为这个文件已经为 root 所有。

## 3. 解决方法

全程在**普通用户权限**下。

首先删除掉 `/usr/bin/` 里面的 `cargo`：

```
sudo apt remove --purge rustc cargo
sudo apt autoremove
```

按照[rCore-2025S 课程实验指导书](https://learningos.cn/rCore-Tutorial-Guide-2025S/0setup-devel-env.html)中的步骤为普通用户重新配置环境。

但是在执行 `curl https://sh.rustup.rs -sSf | sh` 时遇到了问题：

```
error: could not copy file from '/tmp/tmp.PwMcsobzvm/rustup-init' to '/home/hy-huang/.cargo/bin/rustup': Permission denied (os error 13)
```

**请不要尝试在这条命令前加上 sudo，请不要尝试在这条命令前加上 sudo，请不要尝试在这条命令前加上 sudo**，这样会覆盖掉 root 环境。

大概是因为以前在这台电脑上执行过 ``sudo cargo ...`` 这样的命令，导致系统在普通用户的主目录（``/home/hy-huang/.cargo/``）里，创建了一些所有者为 root 的文件或文件夹。现在用普通用户去运行官方脚本，脚本试图写入 ``~/.cargo/`` 目录时，便会发现这些文件夹是 root 所有的，于是报权限不足。

解决方法便是用 root 权限把这些东西删了：

```
sudo rm -rf /home/hy-huang/.cargo /home/hy-huang/.rustup
```

然后继续按照指导书中的步骤来。

普通用户权限下，`cargo-expand` 下载成功！于 20260304 18:10