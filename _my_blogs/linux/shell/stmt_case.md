case 结构条件语句
---


case语句相当于一个if的多分支结构语句

查看帮助文档:
> help case

>> case: case WORD in [PATTERN [| PATTERN]...) COMMANDS ;;]... esac
    Execute commands based on pattern matching.
    
    Selectively execute COMMANDS based upon WORD matching PATTERN.  The
    `|' is used to separate multiple patterns.
    
    Exit Status:
    Returns the status of the last command executed.

~~~sh

case "字符串变量" in
    值1）
        指令1
        ;;
    值2 ｜值2 ｜值3 ）
        指令2
        ;;
    *)
        指令
esac
~~~

注意 双分号作用类似break！表示一个分支结束


分支支持正则哦 就是其强大之处 可以做范围匹配
~~~sh

read -p "input a number " num

case "$num" in
	1)
		echo "1"
		;;
	2) echo "2"
		;;
	[3-9])
		echo "3 .. 9"
		;;
	*)
		echo "error"
esac
~~~



case 语句小结：
1）case语句就相当于多分支的if语句。case语句的优势是更规范，易读
2）case语句适合变量的值少，且为固定的数字或字符串集合
3）系统服务启动脚本传参的判断多用case语句
