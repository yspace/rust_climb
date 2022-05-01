
## 有用的工具
- cargo audit

- cargo deny
依赖树的脆弱性扫描
不维护的crate 软件许可 同库多版本依赖

## 章节

### ch2
捕获需求：用户故事
用户故事模版：
～～～
As a <actor>
I want to <do something>
So that <motives>
～～～

换位思考：
作为一个xx（actor 角色） 我想...（perform actions 干啥） 以便于...(达到什么目的)

## 3.8.4

先要运行脚本 初始化下db
如果docker 已经运行了 则可以使用跳过docker的方法：
> SKIP_DOCKER=true ./scripts/init_db.sh