
shell 支持数学运算 逻辑运算 


## 运算命令

- (())
- let
- expr
- bc
- s[]
- awk
- declare 

 > echo $((1+1))
 
 做赋值时$ 可以省略 ((i=i+1))
 
 ~~~bash
 
 i=1
 echo $i
 ((i=i+1))
 echo $i
 
 echo $((2>1))
 
 echo $((2**3))
 
 echo $[10+10] 
 
 let i=i+8 # 等价 ((i=i+8))
 ~~~
 $(()) 可以用$[] 代替 
 
 
 
 1. “(())”在命令行执行时不需要$符号，但是输出需要$符号
 2. “(())”里所有字符之间有无或多个空格没有任何影响
 
 
 ~~~bash
 
 # cat test.sh 
 #!/bin/env bash

 a=6 # 想从命令行传参 可以用$1 $2
 b=2
 echo "a-b =$(($a - $b))"
 echo "a+b =$(($a + $b))"
 echo "a*b =$(($a * $b))"
 echo "a/b =$(($a / $b))"
 echo "a**b =$(($a ** $b))"
 echo "a%b =$(($a % $b))"
 
 ~~~
 
 ### expr
 
 一定程度可以代替(())
 
 > expr $i + 2 
 
 基本形式 expr <number> <op> <number>
加空格哦 不然会按照字符串对待 

* 需要转义 \* 

~~~bash

i=1
i=`expr $i + 1`
echo $i

~~~

求字符串长度 在mac下好像不能用 说是语法错误

s='some string'
expr length $s


### 小数运算

awk 可以做小数运算 

> echo '1.0 2.3' | awk '{print  ($1+$2)}'


### 条件测试

- test <表达式>

> test -f var.sh && echo true || echo false 
测试 脚本文件是否存在

可以看下手册: 
>man test

- [<测试表达式 >]

[[]] 双方括号 跟单方括号 有些区别

&& || > < 等可以用在[[]] 中 但不能用在[] 

[] 可以使用的如： -a -o -gt  -lt 代替👆上面的操作

~~~bash
[[ 2 > 1 && 5 > 4 ]] && echo "真的" || echo '假的'


[ 2 > 1 -a 5 > 4 ] && echo 1 || echo 0 # 注意> 可能导致结果不对 使用-gt 代替
 
 
(( 1 > 0)) && echo yes || echo false
~~~
注意一些空格问题

- [[ <测试表达式> ]]

	可以支持通配符哦！ 

- (( <测试表达式> ))
 
 