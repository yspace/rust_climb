
## IDE 
选择多行后 
shift+alt+I

可以对多行进行头尾插入相同的内容 试试就知道啦

## 光标跳转

VSCode 返回上一个光标(上一个浏览位置)

键盘映射使用了 jetbrains家的
- ctrl + - 目前是可用的
- ctrl + shift + - 跳转下一次编辑位置

在快捷键编辑界面

搜索

Go Back是回退到上一次编辑位置
Go Faward是回到一次编辑位置

另外搜索栏有个键盘图标，点击可按照按键搜索快捷键哦
// @see https://www.zhihu.com/question/297904236/answer/509741181

## 键盘映射
可以使用 JetBrains IDE Keymap

在其 `贡献功能`选项卡下 可以看到完整的键盘功能映射

也可以看看这个[VsCode 技巧和常用插件收集](https://www.jianshu.com/p/402a9dddc2ab)


## 自定义代码片段

在 Visual Studio Code 中添加自定义的代码片段
https://blog.walterlv.com/post/add-custom-code-snippet-for-vscode.html

需要注意的是，Visual Studio Code 中 Markdown 默认是没有打开智能感知提示的。你需要在你的工作区或者全局打开它。

默认是这样的：

{
  // Configure editor settings to be overridden for [markdown] language.
  "[markdown]": {
    "editor.wordWrap": "on",
    "editor.quickSuggestions": false
  }
}
你需要把 editor.quickSuggestions 设置为 true。

例子：

"博客配置": {
    "prefix": "setting",
    "body": [
        "---",
        "title: ${1:标题}",
        "date: $CURRENT_YEAR-$CURRENT_MONTH-$CURRENT_DATE",
        "categories:",
        " - ${2|使用上下键选择分类,📒笔记,🔧工具|}",
        "tags:",
        " - ${3|使用上下键选择标签,vscode,JS,css,html,Vue,uniapp,微信小程序,React,TypeScript|}",
        "---",
        "",
        ":::tip",
        "${4:输入摘要}",
        ":::",
        "",
        "<!-- more -->",
        "",
        "$5"
    ],
    "description": "博客配置"
}

作者：裹被吹空调
链接：https://juejin.cn/post/7003530143931039774
来源：稀土掘金
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。