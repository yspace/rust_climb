## 常见目录

- boot 挂载到引导分区 一般500m左右就够用了

- dev 设备目录 所有的设备都映射到了文件或者文件夹了！
cpu bus 硬盘 网络 都是！

- etc 配置文件
安装的各种应用配置文件在这里了

- home 每个用户的个性化数据 都在home这里  只是普通用户的文件夹 没有root的

- root root用户的主目录

- opt optional 第三方软件可以装在这里 

- media 媒体 u盘 光驱 可移动媒体设备的挂载点
- mnt mount目录 也是挂载目录 跟上面差不多

- proc process 进程目录 是个虚拟目录 系统内存的映射

- run 进程的实时信息 重启后就会清掉 

- srv service 的缩写 系统服务相关的东西

- sys system 系统硬件信息相关文件

- tmp 临时目录 可以安全删除里面的东西？

- var 可变目录 存放经常修改的数据 日志之类

### 链接类目录
- bin  实际是/usr/bin 的链接     快捷方式类似
- sbin 系统级可用的工具 也是链接 到/usr/sbin

- lib 库目录 类似win中dll system32

- lib64 64位专用？类似win中system
都链接到 /usr/ 下

/usr/local 下还有类似于/usr下的结构！有点递归的感觉