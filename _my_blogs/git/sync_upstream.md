
查看远程 仓库

git remote -v
origin  https://github.com/yspace/rust_climb.git (fetch)
origin  https://github.com/yspace/rust_climb.git (push)

可以看到只有一个

可以通过 remote add 添加其他仓库 比如上游 或者是其他托管平台如bitbucket gitee之类

> git remote add upstream https://github.com/some-org/some-project.git

这条命令添加了一个名为upstream 的另一个远程源 这个源往往是我们最初项目fork到自己账户下的那个原始项目

注意名称是按照惯例来的 实际上叫什么无所谓 

再次 git remote -v   应该就会显示两个源了

~~~sh
git fetch upstream
git checkout master
git merge upstream/master
~~~

上面👆命令拉取上游代码  迁出master分支（这步是本地分支） 然后跟上游master合并
或者rebase
> git pull --rebase upstream master #-- 拉取上游分支到本地master分支


合并后在推送到我们自己的远程库上：

git push origin master

## 注意 

origin upstream 都是管用名称 可以随便起 在多平台时可以用平台名称 
比如 github-origin  gitee-origin ...
