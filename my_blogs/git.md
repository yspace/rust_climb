
## git stash
见： http://www.manongjc.com/detail/64-vanatphzdjacbsy.html

当执行`git stash pop` 命令后产生冲突

此时如何撤销操作，又要保证新的修改不会丢失，可以尝试下面的操作：

> git reset --hard
即可撤销`git stash pop` 操作，将当前分支状态恢复。

git stash 暂存区的记录也不会被删除
可通过 git stash show 查看。