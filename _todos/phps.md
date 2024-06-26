## 版本切换

安装多个php时 需要切换不同的版本


## psr
- [psr](php-fig.org/psr)

## 新特性
- generator yield


## 开发套件
 宝塔
 upuw
 小皮 是啥东东？
 phpenv

## psr
[PHP PSR 标准规范 PHP Standard Recommendations ](https://www.twle.cn/l/yufei/phppsr/php-psr-index.html)

## 审计
`PHP代码审计` 可以google了解下 ^_^

- [PHP代码审计入门指南](https://github.com/burpheart/PHPAuditGuideBook)
[PHP代码审计入门](https://www.freebuf.com/articles/web/252333.html)

https://www.hacking8.com/MiscSecNotes/php/security-coding.html
https://www.hacking8.com/MiscSecNotes/php/audio.html

## 思考题：
- nginx与php之间的交互方式 
    socket { tcp | unix socket }

    [PHP-FPM,Nginx,FastCGI 之间的关系](https://www.likecs.com/show-204444825.html)

    [nginx如何调用php](https://www.cnblogs.com/donghui521/p/10334776.html)
    整理的很好

## 优秀项目
- [laravel admin](https://github.com/z-song/laravel-admin)
- [fast admin](https://www.fastadmin.net/demo.html)


- [fast route](https://github.com/nikic/FastRoute) 路由组件

- [php ast](https://github.com/nikic/) 有个抽像语法树解析库


## php 框架


## 工具
- 压测 ab工具  webbench
- macos下 可以试用siege
- benchmark 工具 wrk

### webman
https://www.workerman.net/webman

### swoole 安装
默认的php扩展目录 被系统保护了 swoole.so 拷贝不进去 
只能在php.ini 文件的 添加一个扩展目录：

~~~php.ini

extension_dir = "/usr/local/lib/php/pecl/20220829"
extension_dir = "/Volumes/macintosh-hd/workspace/php_space/swoole_hello/swoole-src/modules"
~~~

## 优秀库

- [php-pkg](https://github.com/phppkg)
- [php-toolkit](https://github.com/php-toolkit)


## 学习资源
- [w3school php 基础知识](https://www.w3school.com.cn/php/php_ref_array.asp)