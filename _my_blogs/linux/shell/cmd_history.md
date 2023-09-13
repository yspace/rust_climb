
history
---

可以查看历史命令

看下历史命令容量 
> echo $HISTSIZE

查看历史命令存放的文件
> echo $HISTFILE


## 清空

history -c 

## 恢复
从 $HISTFILE 恢复历史命令

history -r ~/.zsh_history


## 调用历史命令

!100

感叹号+历史命令id 

!! 

双感叹 执行上次命令