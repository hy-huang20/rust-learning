# wsl 环境下在 vscode 中进行单步调试

## 1. 概述

我的 vscode 运行在 windows 电脑上，项目是 rust 的且需要在 linux 环境下运行。

## 2. 配置步骤

### 2.1. 下载 vscode 插件

在 vscode 窗口左侧的 `extensions` 商店找一个叫 `WSL` 的插件并下载，发布者为 `Microsoft`。

### 2.2. 连接到 wsl 环境

点击 vscode 界面左下角蓝色的 `><` 标志并点击 `Connect to WSL`，然后选择要打开的项目文件夹。

### 2.3. 在 wsl 中安装 extensions

vscode 会将 extensions 分成本地扩展（windows）和远程扩展（WSL）。所以此时点击 vscode 的 `extensions` 商店，会发现原来本地安装的一些插件并没有被安装，这些插件通常会显示 `Install in WSL`。

为了使用 vscode 更好地调试 rust 项目，需要额外安装一个插件 `CodeLLDB`，发布者是 `Vadim Chugunov`。插件下载成功后可以在 `fn main()` 上面看到类似 `> Run | Debug` 这样的按钮。

>#### 问题记录
>
>远程安装扩展似乎是从远程而不是本地发出下载网络请求，因此可能会出现 windows 上安装插件一切正常而 wsl 安装插件出现问题的情况。
>
>wsl2 配置代理需要本地 clash 开启 `allow lan`。但我的 clash verge 客户端这一块功能似乎有问题，我的 clash for windows 客户端功能正常。

### 2.4. 开始调试

之前说的 `> Run | Debug` 是两个按钮。点击 `Run` 运行；在 vscode 窗口中给程序打断点，点击 `Debug` 开始调试。