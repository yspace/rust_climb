代码复用
---

多人协作 功能划分

linux中近两千多个命令都是shell函数 

平时调用的命令大部分都是函数编写 


## 语法

~~~sh

function fn_name(){

	指令
	return n # $? 可以获取到

}


~~~

简化形式1
~~~sh

function fn_name{

	指令
	return n # $? 可以获取到

}

~~~

简化形式2
~~~sh

fn_name {

	指令
	return n # $? 可以获取到

}

~~~

将函数定义保存为脚本 
然后可以使用 . 或者 source 加载进来 不会产生子shell 


~~~sh

[ -f "/opt/scripts/func_demo.sh" ] && . "/opt/scripts/func_demo.sh" || exit 1

fn_demo 

~~~

函数如果有返回值 再执行函数后想获取其返回值 可以使用 $?

~~~sh

function say_hi(){
	echo "hi"
	return "say_hi_called"
}
say_hi

echo $?


~~~

传参

~~~sh

function say_hi(){
	echo "hi"
	
	echo $1 ;
	
	return "say_hi_called"
}
say_hi qing

echo $?


~~~

不像其他语言 参数列表不需要定义在小括号里面 直接用$1,$2 ... $n. 来访问第N个参数 注意$0 还是脚本名称


## 函数中用echo 

感觉有点怪 但确实有人这么用

~~~sh

function fn_x(){
		echo 'as return '
}

result=$( fn_x )

echo $result

~~~
