# WSL2 代理问题

## 1. WSL2 无法科学上网

### 1.1. 问题描述

使用 `cargo install cargo-expand` 时遇到超时错误：

```
warning: spurious network error (3 tries remaining): [28] Timeout was reached (Connection timed out after 30030 milliseconds)
```

使用 `curl www.google.com` 测试网络也异常。

>有时 curl 某些网址没有输出，可以给 curl 加上 -v 参数查看连接、请求和响应头，给 curl 加上 -L 参数跟随重定向自动跳转到相应页面获取最终内容。

### 1.2. 解决方法

发现是本地使用的 clash verge 客户端 `allow lan` 的问题。我在[这个 issue](https://github.com/clash-verge-rev/clash-verge-rev/issues/1383) 中找到解决方法，可以根据其中[这条 comment](https://github.com/clash-verge-rev/clash-verge-rev/issues/1383#issuecomment-2239291941) 的做法，在`允许应用通过防火墙`中添加 clash verge 安装文件夹里的 `verge-mihomo.exe` 进去，问题即解决。

`env | grep -i proxy` 检查 `http_proxy`, `https_proxy`, `ftp_proxy` 的值 `http://ip:port`。这里的 `port` 为本地 clash 客户端设置的端口值；这里的 `ip` 经测试使用 `/etc/resolv.conf` 中的 ipv4 地址（即 windows 创建的虚拟网卡 vEthernet(WSL) 的 ipv4 地址，作为 WSL 的默认网关，ip 固定）或者本地 WLAN 的 ipv4 地址（最好不要这样做，ip 为动态分配）均可使 `curl www.google.com` 成功获取到内容。

## 2. cargo install 仍然失败

### 2.1. 问题描述

虽然 curl 测试能过了，但是此时发现 `cargo install cargo-expand` 仍然会报相同的错误。

### 2.2. 解决方法

为 `Cargo` 专属配置代理。编辑配置 `vim ~/.cargo/config.toml`，在其中输入：

```
[http]
proxy = "ip:port"
```

或者也可以按照[rCore-2025S 课程实验指导书](https://learningos.cn/rCore-Tutorial-Guide-2025S/0setup-devel-env.html)中的方法换源：

>最好把 Rust 包管理器 cargo 镜像地址 crates.io 也替换成中国科学技术大学的镜像服务器，来加速三方库的下载。 打开或新建 `~/.cargo/config.toml` 文件，添加以下内容：
>
>```
>[source.crates-io]
>replace-with = 'ustc'
>
>[source.ustc]
>registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
>```