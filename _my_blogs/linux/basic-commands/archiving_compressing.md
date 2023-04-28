## gzip

gun zip 是unix 标准压缩程序

可用 gunzip some-file.gz 解压gz文件 移除后缀

> gzip file

用来压缩文件 

## tar 
gzip 并不创建文件的archives ；即 它并不打包多个文件或者文件夹到一个单独的文件。tar为此而来

> tar cvf archive.tar file1 file2 ...

使用.tar 做后缀只是惯例 并不强迫

c flag 激活**create mode**
v flag 激活 verbose diagnostic 输出  文件列表 再加一个v的话会输出文件更详细的信息 如文件大小 权限

> tar cvvf my-archive.tar  1.txt 2.txt 

f flag 指示 文件选项 近邻f的就是tar将要创建的归档目标 我们总是使用此选项跟一个文件名 除非遇到磁带设备 想使用标准输入输出可以设置文件名为`-`

### 解压 .tar 文件
> tar xvf archive.tar
x flag 把tar带到 **extract mode** 
可以只解压归档的单独部件 通过输入其名称在命令的尾部 但是必须知道确切的名字

### 使用table-of-contents 模式

解压前，检查下内容经常是个好主意 通过 t flag 而不是x flag

> tar tvvf archive.tar 
此模式会检查归档的完整性并打印其中的所有文件名 如果不在解压前测试其内文件数量 会导致解压后在当前目录搞出一堆文件 会很不好善后

可以先创建一个临时文件 然后解压 就安全多了 如果文件结构不会太糟糕 然后再使用 mv * .. 移到父目录去

解包时 可以考虑使用p 选项 保留权限设置 此选项在你使用superuser模式下是默认采用的
在检查完整个归档文件后才设置权限的 所以中途不应该中断解包过程
如果有任何问题 搞个flash card 练练兵先。

### 压缩的归档 .tar.gz

> gunzip file.tar.gz
> tar xvf file.tar

先用gunzip 解压 然后用tar验证并解包

打包并不压缩么！！！😄

想创建压缩的archive 反向操作即可：先运行tar 之后 gzip

有一次性搞定两个步骤的东西?

### zcat 

上面的方式既浪费磁盘空间又浪费i/o 时间 更好的是合并此二者为一个pipeline

> zcat file.tar.gz | tar xvf -

zcat命令和  gunzip -dc 一样  -d选项解压缩 -c选项发送结果到标准输出 对我们目前的情况就是tar命令

因为经常使用zcat 我们可以使用z 作为一个选项来在一个归档上调用gzip ；
> tar ztvf  file.tar.gz 
但是记住 实际上我们执行了两步 

NOTE .tgz 跟.tar.gz 文件一样 后缀意味可以适用到FAT文件系统

### 其他压缩工具

xz and bzip2