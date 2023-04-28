

https://rustrepo.com/repo/mongodb-mongo-rust-driver-rust-database

https://developer.mongodb.com/quickstart/rust-crud-tutorial/

https://github.com/thedodd/wither

## setup the mongo enviroments

> docker-compose -f mongo.yaml up

shut down all container

> docker-compose -f mongo.yaml down

this cmd also remove the default network .
view it:
> docker network ls

## docker with rust
[当 docker 遇上 rust](https://zhuanlan.zhihu.com/p/356274853)

1.
> cargo vendor

2.
then create a .cargo/config.yaml to use the vendor dir

~~~
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "/usr/src/<your-project-dir>/vendor"
~~~
docker build will use the vendor dir path

3.
add the vendor to svc  
> git add vendor && git commit -am 'add vendor'

4.
prepare the dockerfile

~~~

FROM rust:1.50 as builder
# Use prebuilt builder image
# FROM rust:1.50-prebuilt as builder
WORKDIR /usr/src
ARG APP=<your-app-name>

# New cargo project and copy Rust dependencies (and store as a separate Docker layer)
# NOTE: must call `cargo vendor` first and add `vendor` folder to git
RUN USER=root cargo new $APP
WORKDIR /usr/src/$APP
RUN mkdir -p .cargo
COPY config.toml .cargo/
COPY vendor vendor
COPY Cargo.toml Cargo.lock ./

COPY ./src src
RUN cargo install --path . --color always

# Copy the app to an base Docker image, here we use distroless
FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/local/cargo/bin/$APP /$APP
USER 1000
ENTRYPOINT ["/$APP"]

~~~

CMD = entrypoint command
but you can have multiple RUN command

5
docker build

> docker build -t <my-app-name>:1.0 .

if you want rebuild the image , just delete that image first then build it again . (using the docker rmi <img-id> , may be you should stop the container and delete it firstly)



## mongo 场景
rust后花园 群佬分享：

- 可以做聊天室 聊天消息是 高写入 低读取 时序库也不合适 先存入文档db 后期再做清洗
 聊天分
 1. 私聊
 2. 聊天室
 3. 群聊
 访问特征不同 私聊 读写持平  聊天室读多写少        群聊 高写低读 （注意 话痨情况 容易占空间很多）

 私聊简单点，群聊有读写上线导致群聊有人数上限，聊天室无人数上限但做不到消息完整

 三类消息需要做不同处理


 写放大 或者说扩散是不是 要把消息 推送给所有群成员的各自队列
 谁在线给谁推。不在线的不需要推。