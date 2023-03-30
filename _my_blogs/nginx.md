https://blog.csdn.net/hechenhongbo/article/details/105688194

随便搜一个安装教程

先安装 
然后改配置

## 安装nginx

> brew intall nginx

## php 相关的配置

启动php-fpm
> sodu php-fpm 

第一次启动如果报错 原因多半是没有对应的conf配置文件 找到配置文件最下面的include 行
然后到对应目录去 根据default 模版 创建一个配置文件出来就好了

仔细查看一下你的php-fpm.conf文件

;;;;;;;;;;;;;;;;;;;;
; Pool Definitions ;
;;;;;;;;;;;;;;;;;;;;

; Relative path can also be used. They will be prefixed by:
;  - the global prefix if it's been set (-p argument)
;  - /app/php/7.2.7 otherwise
include=/app/php/7.2.7/etc/php-fpm.d/*.conf

注意上面最后一行，去你的对应的文件夹下面将默认的www.conf.default重命名为www.conf。