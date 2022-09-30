chrome 浏览器下
右键某个元素 -》 审查｜检查

然后在Elements tab下 command+f 就可以用xpath搜索过滤元素啦



## 绝对路径

## 相对路径

//html_tag[@attribute='some_str']
//html_tag[@attribute='some_str' and @another_attribute='some_str']

//a[text()='some_text']
text() 是个函数

//a[contains(text(),'some_text')]
选取哪些a元素的文本包含'some_text'的