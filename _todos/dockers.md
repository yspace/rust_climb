有个podman 功能类似docker？

命令支持也类似 几乎可以看作同义词 . 别名下 可以平替

>$ alias docker=podman

## vs code
在ide中 command+shift+p 
然后 输入 `docker add`   在下拉提示中选择一个子命令
可以在当前工作空间中添加docker文件

### docker volume

防止docker容器实例关闭后数据丢失 可以创建一些volume 然后映射到容器内部的文件夹去
这样容器即便停止 数据亦不会丢失
（一般我们需要做主机文件到容器文件的映射）

- docker volume create my_volume_name

- docker volume ls

- docker run -v my_volume_name:app 
这个命令就是把我们创建的卷名映射到app目录去


## docker compose
v2 compose工具用python写的 。老版是go写的！

