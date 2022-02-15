[svelte tailwind](https://scottspence.com/posts/how-to-set-up-svelte-with-tailwind)

如果因为代理等原因报错
查看配置
~~~
npm config ls

~~~

~~~
解决办法：

1、执行：

npm config get proxy
npm config get https-proxy
如果返回值不为null，继续执行：
（这一步很重要，一定要保证两个命令的返回值都为null,话说回来，应该出现这个错误这两个返回值有不为null的）
npm config set proxy null
npm config set https-proxy null
2、执行：
npm config set registry http://registry.cnpmjs.org/
~~~

也可以使用cnpm
> npm install -g cnpm --registry=https://registry.npm.taobao.org 

或者使用代理试试

## 其他可配合 svelte的 ui框架
- [svelte 和bootstrap 结合 ](https://sveltestrap.js.org/ )
- [ibm carbon ui](https://carbon-components-svelte.onrender.com/)


## 还有什么ui
- Bulma