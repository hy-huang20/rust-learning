# git submodule

## 1. 问题描述

现在情况是在 `embassy-learning` 分支下有一个 `submodule` 为 `embassy`。使用普通的 `git checkout` 切换到别的分支时会把 `embassy/` 完整地带过去：

```bash
>git checkout stack-less-coroutine
warning: unable to rmdir 'embassy': Directory not empty
Switched to branch 'stack-less-coroutine'
```

从而成为别的分支的 untracked files。在别的分支上试图删除 `embassy/` 后，再切换回原分支时发现 `embassy/` 下已经空了。又需要重新执行 `git submodule update --init --recursive` 恢复 `embassy/`。

## 2. 解决方法

如果只是为了不把 `embassy/` 带过去的话，可以在切换分支时加上 `--recurse-submodules` 参数：

```bash
git checkout --recurse-submodules stack-less-coroutine
```

但缺点是未来再次切换回来后仍然需要 `git submodule update --init --recursive` 恢复。一劳永逸的方法是设置 config：

```bash
git config submodule.recurse true
```

这样上述的问题就完全解决了。

## 3. 补充

解决上述问题后又引入了新的问题。

### 3.1. 问题 1

在 `embassy-learning` 分支上，发现更新 embassy 后，提示 `Cargo.toml` 中库的版本问题，按照输出提示修改版本后，运行时出现编译错误。发现原来是官方 github 仓库的 embassy 更新了，现在的需求是把本地 embassy 子模块的 remote 修改为我自己曾经 fork 的[仓库](https://github.com/hy-huang20/embassy.git)，使得官方 embassy 更新不会导致本地代码特性**过时**。

首先将 `.gitmodules` 文件中的 url 改成自己仓库的 url。然后执行 `git submodule sync`。这样 `cd embassy/` 切换到子模块下并查看 `git remote -v` 便会发现 origin 地址已经变成自己仓库的 url 了。

但现在出现了一个问题：由于本地的仓库是来自官方的仓库，是比我自己曾经 fork 的仓库要新的。现在需要将本地的仓库变成我自己 fork 仓库的那个**旧版本**。这是 git 中很经典的**硬重置**（Hard Reset）操作。直接执行：

```bash
git fetch origin 
git checkout main
git reset --hard origin/main
```

这样返回上级目录 `cargo run` 便能正常运行了。

但子仓库变旧会被父仓库察觉的，因为父仓库只认子仓库的哈希值。所以也需要在父仓库提交 embassy 的改动。

### 3.2. 问题 2

就是每次只要涉及到 `embassy-learning` 分支的切换时，由于 `embassy/` 很大文件很多，如果经常涉及到 `embassy-learning` 分支的切换的话会造成 git 频繁读写磁盘，因此会引入开销，切换分支比较耗时。

解决方法是使用 `git worktree` 指令。注意不能在 `embassy-learning` 分支上直接做，需要先切换到其它某个任意分支，再执行：

```bash
git worktree add ../rust-learning-embassy-learning-branch embassy-learning
```

这会在 `rust-learning` 的同级目录创建一个 `rust-learning-embassy-learning-branch/` 新文件夹，然后把 `embassy-learning` 分支的内容移动新文件夹中。

在 `rust-learning` 老文件夹中 `git branch` 仍然可以看到 `embassy-learning` 分支，但**无法**再直接 `git checkout` 切过去，可以在命令行通过 `cd` 命令，也可以新开一个终端，手动移动到新文件夹。

在新文件夹 `rust-learning-embassy-learning-branch/` 中 `git status` 可以看到处于 `embassy-learning` 分支。

在本地使用 `git worktree` 在远程 github 仓库看来什么也没发生，**不会影响**远程仓库的状态与内容。

但是，新文件夹 `rust-learning-embassy-learning-branch/` 会进入父仓库 `rust-os-learning` 的视野，需要通过 `.gitignore` 忽略掉。

全部做完了。好处是涉及到 `embassy-learing` 分支的切换不再需要 git 每次创建/删除 `embassy/` 这个大文件夹，节省时间；坏处是无法在 `rust-learning/` 中通过 `git checkout` 丝滑切换到 `embassy-learning` 分支。