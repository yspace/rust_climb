[作者们的讨论！](https://bytemeta.vip/repo/jonhoo/fantoccini/issues/145?page=1)
[更早的：](https://www.reddit.com/r/rust/comments/evlc49/thirtyfour_a_new_selenium_library_for_rust_for/) 

## 先决步骤

安装 mozilla出品的[geckodriver](https://github.com/mozilla/geckodriver)
惊喜的是可以用cargo安装 之后直接运行
> cargo install geckodriver && geckodriver

进程都可以指定端口 不指定就用默认的了


也可以安装 WebDriver compatible 就是兼容WebDriver的进程 Selenium

chrome 的好像有更多扩展插件


更多依赖看examplers：
https://github.com/jonhoo/fantoccini/tree/main/examples

## dom 操作
网页抓取后 可以把文本的html页面变为dom结构 这样可以做jquery类似的操作 可选库也挺多
有国人开发的visdom 挺好用的jquery风格！[visdom](https://github.com/fefit/visdom)
此库adobe好像用了！


## 存db 

爬网页 总归是要落地的 需要的信息可以存储在文件或者db中 方便后续查看 提取有用的东西

这里使用最简单的sqlite 不过想方便的使用它可不那么简单 因为需要考虑异步运行时 sqlx稍显笨重
这里使用了一个最近才开始的库 发现挺轻便好用：[tokio-rusqlite](https://github.com/programatik29/tokio-rusqlite/tree/master/examples) 配合 rusqlite 库一起使用 如果还想支持更人性化的查询构造 还可以考虑sea-query