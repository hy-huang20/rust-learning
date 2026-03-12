# git orphan branch

## 1. 需求概述

在我的 [rust-learning](https://github.com/hy-huang20/rust-learning) github 仓库中，不同的项目被放在不同的分支上，并希望这些分支之间没有任何联系。而之前不知道使用 git 的 `--orphan` 参数创建分支，已经在以常规方式创建的分支上进行过开发和 commit 了，从而使得这些分支带有 checkout 前原分支的 commit 记录。

## 2. 调整过程

上述需求其实就是希望创建一系列的 orphan 分支。

以仓库中的 `async-await` 分支为例（从 `main` 分支 checkout 出来的新分支）进行修改。首先保证目前确实在 `async-await` 分支上。

执行 `git log` 或者 `git log --oneline` 查看历史记录，找到曾经在这个分支上提交的第一个 commit（注意不是从原分支继承过来的那些历史），假设它的哈希值是 `<START_HASH>`。或者，查看 main 和 `async-await` 分支的交叉其实还有更好的办法：`git merge-base main async-await`。

然后，在切口处创建一个临时的孤儿分支 `temp-async-await`：

```bash
git checkout --orphan temp-async-await <START_HASH>
```

此时可以发现已经切换到 `temp-async-await` 分支上来了，并且这个 `git log` 没有历史记录。通过 `git status` 可以看到 `<START_HASH>` 当时的代码原封不动地留在暂存区，准备提交。

克隆根节点：

```bash
git commit -C <START_HASH>
```

注意是大写的 `-C`。上面的指令会把代码提交为一个全新的 root commit 并且完美复制当时写的 commit 注释、作者信息和时间戳。

将后续所有的 commit 嫁接过来：（下面的指令会把 `async-await` 位于 `<START_HASH>` 之后的一串 commit **复制**（确切地说是提取差异并重新生成 commit，所以说新分支上的 commit 虽然是复制过来的但其实和旧 commit 拥有不同的哈希，然后其实旧 commit 虽然没有指针指向了但却不会立刻物理消失）并拼接到 `temp-async-await` 之后，然后 `async-await` 会自动指向形成的新链条的最末端。你会发现执行完下面这条指令后，你所在分支名称自动变回了 `async-await`。这是因为下面的指令会在剪切 commit 前会帮你自动先执行一次 `git checkout async-await`）

```bash
git rebase --onto temp-async-await <START_HASH> async-await
```

把临时分支 `temp-async-await` 清掉（顺便提一下，`temp-async-await` 指向的是前面提到的 root commit，也即形成的新链条的第一个 commit。由于新链条的末端已经被 `async-await` 所指向所跟踪，于是 `temp-async-await` 这个临时指针便失去了存在价值，而且删除它不会影响到那个 root commit，因为这个 root commit 依然为 `async-await` 所持有）：

```bash
git branch -d temp-async-await
```

最后 push 到远程仓库，注意只能强制 push 上去：

```bash
git push origin async-await --force
```

对于你想要实现为 orphan 分支但曾经却按一般方式创建并提交了 commit 的分支们，一一重复上述的步骤即可。

## 3. 补充

Github 网页 `insights -> Network` 可视化分支的更新可能会有延迟。更新后可以看到分支之间相互独立，之间没有箭头。

在解决上述需求的时候发现了**新的问题**。

### 3.1. 问题 1

有些分支的项目是已经在 main 分支上开发并提交过一些 commit 后，才发觉应该重新开一个分支而不是直接在 main 分支上开发；在开了新分支后，又在新的分支上进行了开发和提交。然后希望在一个新的 orphan 分支上保留上述提到的所有 commit。

其实如果只考虑新分支的事，而不处理 main 上的对于 main 而言的这些所谓“垃圾记录”的话，解决方法其实和上面大致一样的，只是 `<START_HASH>` 需要改成 main 上那个你针对这个项目的第一次提交。

### 3.2. 问题 2

分支 `embassy-learning` 从 main 创建，但是多次 merge 回 main。希望：

- 不保留 merge 的记录
- 发现 commit 中包含了其它文件夹。在 `embassy-learning` 分支上只希望保留和 `embassy-learning` 相关的修改

这需要上面 `rebase --onto` 的步骤加上一个参数 `-i`（interactive 交互）：

```bash
git rebase -i --onto temp-embassy-learning <START_HASH> embassy-learning
```

进入交互页面后可以看到一系列以 `pick` 开头的 commit 记录，把不需要的记录行删除掉或者把该行的 `pick` 改成 `drop`。

如果出现报错的话，我的做法是多试几次 `git rebase --continue` 直到没有报错出现。

如果打开了 nano 编辑器，按 `Ctrl + O` 后 `Enter` 保存修改，按 `Ctrl + X` 退出。


