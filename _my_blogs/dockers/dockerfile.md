
结构 

~~~dockerfile

# 这是baseImage
FROM node:19-alpine

# 执行copy命令 把当前文件夹下的文件｜目录拷贝到容器指定位置
COPY package.json /app/
COPY src /app/

# 相当于本地的 chdir 切换当前工作目录到指定位置后
WORKDIR /app

# 运行shell命令 可以是多个
RUN npm install

# CMD是作为最后一个命令出现的 且只能有一个
CMD ["node" , "server.js"]

~~~

这是`定义`文件 用来build image的

> docker build --tag my-node-app:1.0 .

上面的命令是 根据dockerfile的定义构建一个image 可以指定我们自己的tag 和生成位置 


## note

docker image 由层构成 每个指令会生成一个层