chrome 浏览器下
右键某个元素 -》 审查｜检查

然后在Elements tab下 command+f 就可以用xpath搜索过滤元素啦

## 参考：
- [XPath语法完全指南（实战详解版）](https://blog.csdn.net/m0_73978383/article/details/146946257)
- [XPath语法完全指南（实战详解版）](https://blog.csdn.net/weixin_66401877/article/details/146904839)

## 绝对路径

## 相对路径

~~~

//html_tag[@attribute='some_str']
//html_tag[@attribute='some_str' and @another_attribute='some_str']

//a[text()='some_text']
text() 是个函数

//a[contains(text(),'some_text')]
选取哪些a元素的文本包含'some_text'的

xpath('//div[contains(@class,"a") and contains(@class,"b")]') 
#它会取class同时有a和b的元素

~~~
[Xpath定位元素](https://www.cnblogs.com/gxj521test/p/16972015.html)


二，xpath介绍和基本语法
基本语法
表达式 描述
nodename 选取此节点的所有子节点。
/ 从根节点选取。绝对路径
// 从匹配选择的当前节点选择文档中的节点，而不考虑它们的位置。相对路径
. 选取当前节点。
.. 选取当前节点的父节点。
@ 选取属性。
三，绝对定位和相对定位
绝对路径：每次从根节点 html 开始
相对路径：每次只需从第一个已经找到的节点开始。
绝对定位的劣势：
前端经常变化，加 div 之类的。下次就不能用了。
表达更加繁琐
四，谓语条件
谓语被嵌在方括号中，用来查找某个特定的节点或者包含某个指定的值的节点，也就是额外的条件。文
本也可以定位。
1，索引取值：[索引值]
注意点：
索引的顺序是从1开始的，不是 0
索引的优先顺序比 // 高，建议使用索引的时候前面的部分使用括号包起来

~~~
(//a[@class, 'btn-special'][0])
# 查找第一个元素input元素，
e.find_element_by_xpath("(//input)[1]")
# (//input)[1] 和 //input[1]是有区别的

2，有某个属性：[@属性]

# 查找有value属性 的input标签
e.find_element_by_xpath("//input[@value]]")

3，属性为某个特定值 ： [@属性=值]

# 查找value等于 百度一下 的input标签
e.find_element_by_xpath("//input[@value='百度一下']")
4，子元素中属性为某个特定值：[//子元素/@属性=值]

# 查找form， from下有一个option元素的value属性为 musen
e.find_element_by_xpath("//span[input/@value='百度一下']")
5, 文本 //a[text()='']

# 查找文本内容为 百度首页的a标签
//a[text()='百度首页']
五，通配符和逻辑运算 and or

# 查找name属性等于user或者id属性等于user的input标签
//input[@name='user' or @id='username']
# 查找name属性等于user并且id属性等于user的input标签
//input[@name='rsv_enter' and @name='rsv_dl']
注意点：
find_element_by_class_name进行定位 class之间不能有空格
xpath通过class属性定位的时候，class属性值可以有空格
六，函数
函数名 说明
starts-with(str1,str2) str1是否以 str2 开头
contains(str1,str2) str1是否包含 str2

# 查找 value属性为 百 开头的input标签
//input[starts-with(@value,'百')]
# 查找 value属性 包含百度的 input标签
//input[contains(@value,'百度')]
七， 轴定位

# /轴名称（元素关系）::节点名称
# 案例：查找form标签子节点中id等于s_btn_wr的span标签
//form/child::span[@id='s_btn_wr']
# 案例二：查找form标签父节点的div标签
//form/parent::div
# 案例三：查找title等于苏宁易购的a标签 的父节点div 的所有在它之前的兄弟节点
//a[@title='苏宁易购']/parent::div/preceding-sibling::div
# 案例四：查找title等于苏宁易购的a标签 的父节点div 的所有在它之后的兄弟节点
//a[@title='苏宁易购']/parent::div/following-sibling::div
轴定位包含了路径寻找，如果需要多层往上，轴定位更加方便。
兄弟姐妹都可以
轴名称 结果
child 选取当前节点的所有子元素。
descendant 选取当前节点的所有后代元素（子、孙等）。
parent 选取当前节点的父节点。
ancestor 选取当前节点的所有先辈（父、祖父等）。
following 选取文档中当前节点的结束标签之后的所有节点。
following-sibling 选取当前节点之后的所有兄弟节点。
preceding 选取文档中当前节点的开始标签之前的所有节点。
preceding-sibling 选取当前节点之前的所有节点。

2、CSS 选择器
层级关系
.intro 选择 class="intro" 的所有元素。

firstname 选择 id="firstname" 的所有元素。
选择所有元素。
p
选择所有
div,p
选择所有
元素和所有
div p
选择
元素内部的所有
div>p
选择父元素为
元素的所有
div+p
选择紧接在
元素之后的所有
[attribute] [target] 选择带有 target 属性所有元素。
[attribute = value] a[title = 苏宁易购] title属性等于苏宁易购的a标签元素。
[attribute *= value] a[title *= 宁易] title属性包含宁易的a标签元素。
[attribute ^= value] a[title ^= 苏宁] title属性以苏宁 开头的a标签元素。
[attribute = value] a[title =  易购] title属性以易购 结尾的a标签元素。

谓语
[attribute] [target] 选择带有 target 属性所有元素。
[attribute = value] a[title = 苏宁易购] title属性等于苏宁易购的a标签
[attribute *= value] a[title *= 宁易] title属性包含宁易的a标签
[attribute ^= value] a[title ^= 苏宁] title属性以苏宁 开头的a标签
[attribute 
=
v
a
l
u
e
]
a
[
t
i
t
l
e
= 易购] title属性以易购 结尾的a标签
索引

nth-of-type：正向索引
# 获取第五个类属性为nav-item的div标签
div.nav-item:nth-of-type(5)
nth-last-of-type：反向索引
# 获取倒数第2个类属性为nav-item的div标签
div.nav-item:nth-last-of-type(2)


3、css和xpath比
css的效率快
CSS 是和 HTML 绑定在一起的，是直接操作HTML的，如果换成 XML 其他的标记语言，就
不行了。
Xpath 是和 DOM 绑定在一起的。所以还需要时间通过HTML去生成 DOM 对象。
只要有 DOM 就可以用 Xpath来定位元素, 也就是说 xpath 得用途更广，手机也可以用，其
他客户端也可以用。
xpath 在 IE 中效率会高，其他谷歌和火狐浏览器，都会比css慢。
xpath 的功能更多
xpath 在找到一个元素是可以上下移动的。 css 只能进行向下或横向去找。
xpath可以根据文本内容查找元素。
对于简单的元素定位，CSS选择器表达更加简洁。但是定位复杂的元素与 Xpath 相比不太友好。

~~~