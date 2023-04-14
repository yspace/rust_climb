## 基本命令

很多命令可以接受多个参数，

- ls [dir|file]
	list 某个目录的内容 默认是当前目录
	
	有一些options
	-l 详情
	-F 文件类型信息
	
- cp file1 files
	拷贝一个文件到另一个目录 文件名不变
	> cp file dir
	拷贝多个文件到一个文件夹
	> cp file1 file2 file3 dir
	
- mv file1 file2
	移动文件
	很像cp 最简单形式会重命名一个文件，
	> mv file1	file2
	也可以如cp那样移动多个文件到其他目录
	
- touch 
	可以创建一个文件，如果文件已存在那么不对文件做改动 但是会更新文件的修改时间。
	
- rm
	删除文件 且不可恢复 ！
	
- echo 	
    将其参数输出到标准输出
	
	此命令非常有用 可用来展开shell globs (比如*) 和一些变量 如$HOME
	
	有点类似php了！
	
## 浏览目录

始于/ 亦称根目录 分割符是`/` 不是反斜杠哦

有些标准目录如 /usr

绝对路径以斜杠开始
.. 表示父目录
.  单点表示当前目录 很多命令对文件路径处理上 默认目录就是当前目录 如果不带/的话

不以/开始 的目录表示即为相对目录

- cd
  pwd 命令可以用来查看当前目录
  
  每个进程都可以单独设置其当前的工作目录 
  cd dir 可以改变shell的当前工作目录 如果没有给出dir参数 那么会切到`home directory` 有些程序用`~` 简称home目录
  
  cd 命令是shell 内建命令 不可以独立出来
  
 - mkdir
 	创建目录
   > mkdir some-dir
   
 - rmdir dir
 	移除目录 如果目录非空命令会失败，
	rm -r dir 会递归删除其内容 ！所以小心点使用 特别是作为superuser时
	
	运行前再次检查下
	
- shell 通配符
	globbing 
	
	> echo *
	打印当前目录下的文件列表
	
	展开expansion 
	+ at* 开始名称是at
	+ *at 展开所有at结尾的文件名
	+ *at* 展开所有文件名包含at的名称
	
	如果没有文件匹配到一个glob bash-shell不执行展开 输出常规字面量 
比如没有文件匹配到以.为后缀时 `echo *.`输出 *.

*可以匹配任何文件

另一个？question-mark  指挥shell匹配精确的一个任意字符 如果不想shell在命令中展开glob 可以使用 '*'

shell 在运行command之前会做展开 至于不做任何展开的情况 后续怎么做有command做决定

## intermediate command

- grep
	从文件或者输出流中打印匹配到一个表达式的行
	> grep root /etc/passwd
	
	如果想在多个文件中匹配root字符串 可以使用glob展开
	$> grep root /etc/* 
	
	两个重要的命令选项
	-i 做大小写敏感匹配
	-v 做反向搜索 -- 就是说打印任何不匹配的
	有个更强大的变体 egrep 只是grep -E的同义词
	
	grep 可以理解正则式 正则式比通配符风格的匹配更强大且有不同的语法 关于正则记住有三点注意项
	
	1 .* 匹配任何数量的字符，包括不出现（如同* 在glob 的通配表现）字符数量在数学上的意义 >=0
	2 .+ 匹配任何一个或者更多字符 字符数量在数学上的意义>0
	3 . 只匹配任意一个字符
	关于正则可以参考正则式相关的书籍 正则的理论是关乎自动机的
	
- less
	大文件时  可以用此命令浏览内容 有句话 less is more
	该命令是老的more命令的加强 有些版本比如嵌入式系统可能没有这个命令 那时只能用more了
	用法如
	> less /usr/share/dict/words
	可以使用空格键一次显示一屏内容; 
	b 回跳一整屏内容 ; 
	退出 q
	
	也可以在less中搜索单词
	/<serch-word-here> 可以做前向搜索 即当前内容的后面的内容中搜索单词
	?<search-word> 可以做反向搜索
	可以按 n 继续搜索
	
	发送一个命令的输出给grep
	> grep ie /usr/share/dict/words | less
	这就是一个命令行管道的用法
	
- pwd 
	打印当前工作目录的名字
	
	>pwd -P 
	symbolic links 有时会复杂化当前工作目录的全路径
	
	
- diff
	查看两个文件的不同
	>  diff file1 file2
	
	有一些选项可以控制输出的格式
	
- file
	看到一个文件 但是不确定其格式
	> file some_filename 
	看看系统是否可以猜到
	
- file and locate
	怀疑有个文件在某个目录树的某个位置 但不知道在哪儿 
	> find dir -name file -print
	
	也可以做模式匹配 但为了躲过shell自己的glob展开 可以用单引号形式
	>find some-dir -name 'file*' -print
	
	有些系统也提供了locate命令来找文件 该命令在系统周期性构建的索引里寻找 所以比find命令会快 但如果你正在找的文件比索引新 locate就找不到它了
	
- head and tail
	快速的浏览一个文件或者数据流的部分。
	> head /etc/passwd 前十行
	> tail /etc/passwd 后十行
	-n 选项 n是想看到的行的数量
	> head -5 /etc/passwd
	
- sort 
	排序文件的行形成字典序
	-n 选项用来应对以数字开始的行
	-r 选项反转排序的顺序
	
- passwd
	改变我们的密码，会被要求旧密码 然后输入两次新密码
	
-  点 文件
	在用ls 命令时添加 -a 选项，就可以看到. 开头的文件了
	常见的如： .bashrc .login 还有点目录
	
	通常这些文件不会显示 除非用特殊选项 如-a 
	
	此外 shell globs 也不匹配点文件 除非我们明确的使用模式如： .*.
	.* 可以匹配. 和.. 
	可以使用模式如 .[^.]* 或 .??* 来获取所有的除了当前文件夹和父文件夹外的文件
	
- 环境变量 和shell 变量
	shell 可以存储临时变量，称为shell variables 包含文本字符串的值 shell变量用来跟踪脚本中的值很有用 有些变量用来控制shell的行为
	> STUFF=blah =用来赋值 注意等号左右无空格
	&VAR_NAME 用来读取变量  echo &STUFF

	环境变量类似于shell变量，但不是针对shell的 所有unix系统的进程都有环境变量存储variable storage 
	二者的主要区别 os 传递所有的shell环境变量给到shell运行的程序
	而shell变量不能被shell所运行的命令访问到
	
	环境变量的赋值可以使用shell 的export 命令 如
	> STUFF=qing
	> export STUFF
	因为子进程继承其父进程的环境变量 很多程序从配置文件或者命令选项中读取它们。
	
	less 命令行可以使用LESS 环境变量 所以可以在使用它们之前给其赋值 
	很多命令手册中 有ENVIRONMENT 段 会描述这些变量
	
- 命令路径
	PATH 是一个特殊的环境变量 包含命令路径 一些系统目录 shell在定位命令时会搜索它们的。
	> echo $PATH 
	查看路径
	
	如果多个路径中都有命令文件 那么shell运行那个首先匹配到的程序
	> PATH=some-dir:&PATH 
	可以添加shell定位命令的目录 或者添加到尾部
	> PATH=&PATH:dir
	当因为失误擦除了PATH 变量的内容时 可以关闭当前shell 重启一个shell
	
- 特定字符
		
	1 * 用于正则 glob
	2 . 当前目录 文件/主机名称分割符
	3 ！ negation 命令历史
	4 | 命令管道
	5 / 目录分隔符，搜索命令
	6 \ literals 宏
	7 $ 变量，行尾
	8 ' 字符串字面亮
	9 ` 命令替换
	10 " 半-字面量 字符串            意思就是里面的内容有些会被解析为变量 类似php中的双引号
	11 ^ Negation ，文件的开始
	12 ~ Negation , 目录的shortcut
	13 # 注释,预处理器，替换
	14 [] 范围
	15 {} 语句块 ,范围
	16 - 空格的替换，
	
- 命令行编辑
	 CTRL-B 移动光标向左
	 CTRL-F 移动光标向右
	 CTRL-P 查看前面的命令 或者上下箭头
	 CTRL-N 查看下一条命令 
	 CTRL-A 移动光标到行首
	 CTRL-E 移动光标到行尾
	 CTRL-W 擦除前面的word
	 CTRL-U 擦除光标到行首的内容
	 CTRL-K 擦除光标到行尾的内容
	 CTRL-Y 粘贴擦除的文本 比如从CTRL-U 的内容
	 
- 获取在线帮助

	> man ls
	> man -k <keyword>
	比如想找排序相关的命令
	> man -k sort
	
	man 可以指定手册段号 不同的段分类不同
	> man 5 passwd
	
	类似man的info命令  
	 $> info command
	 
	如果不喜欢info reader 可以将输出发给less 用管道符号即可|
	
	有些包导出其文档到 /usr/share/doc
	
	 
	


	
	
																						
							