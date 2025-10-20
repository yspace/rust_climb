// [js防抖和节流优化浏览器滚动条滚动到最下面时加载更多数据](https://www.cnblogs.com/czkolve/p/10859176.html)
// [浅谈 JS 防抖和节流](https://segmentfault.com/a/1190000018428170)


~~~js
/* 不做处理input触发keyup事件 *//* 输入快的话会出现输出结果多次重复 */window.onload = function () {
    let obj = document.getElementById('input')
    obj.addEventListener('keyup',()=>{
        let val = obj.value;
        inputchange(val)
    },false)
}
function inputchange(val) {
    console.log(val)
}// 11  11  123 123  123456 123456 123456


/* 使用防抖去处理input触发keyup事件 */
/* 输入快的话会也不会输出多次重复结果 */
window.onload = function () {
    let obj = document.getElementById('input')
    let callback = inputchange()
    obj.addEventListener('keyup',()=>{
        let val = obj.value;
        callback(val)
    },false)
}
// 高级函数的应用
// 闭包time为局部变量 但不会在inputchange 函数调用完后释放
// 函数返回函数
function inputchange() {
    var time
    return function(val){
        if(time)
        {
            clearTimeout(time)
        }
        time = setTimeout(() => {
            console.log(val)
        }, 200);
    }
}
~~~


// 

~~~js

/*
* fn [function] 需要防抖的函数
* delay [number] 毫秒，防抖期限值
*/
function debounce(fn,delay){
    let timer = null //借助闭包
    return function() {
        if(timer){
            clearTimeout(timer) //进入该分支语句，说明当前正在一个计时过程中，并且又触发了相同事件。所以要取消当前的计时，重新开始计时
            timer = setTimeout(fn,delay) 
        }else{
            timer = setTimeout(fn,delay) // 进入该分支说明当前并没有在计时，那么就开始一个计时
        }
    }
}


/*****************************简化后的分割线 ******************************/
function debounce(fn,delay){
    let timer = null //借助闭包
    return function() {
        if(timer){
            clearTimeout(timer)
        }
        timer = setTimeout(fn,delay) // 简化写法
    }
}
// 然后是旧代码
function showTop  () {
    var scrollTop = document.body.scrollTop || document.documentElement.scrollTop;
　　console.log('滚动条位置：' + scrollTop);
}
window.onscroll = debounce(showTop,1000) // 为了方便观察效果我们取个大点的间断值，实际使用根据需要来配置


//
function throttle(fn,delay){
    let valid = true
    return function() {
        if(!valid){
            //休息时间 暂不接客
            return false
        }
        // 工作时间，执行函数并且在间隔期内把状态位设为无效
        valid = false
        setTimeout(() => {
            fn()
            valid = true;
        }, delay)
    }
}
/* 请注意，节流函数并不止上面这种实现方案,
   例如可以完全不借助setTimeout，可以把状态位换成时间戳，然后利用时间戳差值是否大于指定间隔时间来做判定。
   也可以直接将setTimeout的返回的标记当做判断条件-判断当前定时器是否存在，如果存在表示还在冷却，并且在执行fn之后消除定时器表示激活，原理都一样
    */

// 以下照旧
function showTop  () {
    var scrollTop = document.body.scrollTop || document.documentElement.scrollTop;
　　console.log('滚动条位置：' + scrollTop);
}
window.onscroll = throttle(showTop,1000)

~~~


[JavaScript实现页面滚动条平滑滚动技术详解](https://comate.baidu.com/zh/page/nx6b0vyl7iz)

~~~js
// 滚动到指定元素并启用平滑效果
document.getElementById('target').scrollIntoView({
  behavior: 'smooth'
});

// 当需要更精确控制滚动位置时，可使用window.scrollTo配合requestAnimationFrame：
function smoothScroll(targetY) {
    const startY = window.scrollY;
    const distance = targetY - startY;
    const duration = 800; // 动画持续时间(ms)
    let startTime = null;

    function scrollStep(timestamp) {
        if (!startTime) startTime = timestamp;
        const elapsed = timestamp - startTime;
        const progress = Math.min(elapsed / duration, 1);
        const easeInOut = progress < 0.5 ?
            2 * progress * progress :
            1 - Math.pow(-2 * progress + 2, 2) / 2;

        window.scrollTo(0, startY + distance * easeInOut);

        if (progress < 1) {
            window.requestAnimationFrame(scrollStep);
        }
    }
    window.requestAnimationFrame(scrollStep);
}

// 使用示例：滚动到页面底部
smoothScroll(document.body.scrollHeight);


// 当需要控制特定容器的滚动时，可使用scrollTop属性：
function smoothScrollContainer(containerId, targetY) {
    const container = document.getElementById(containerId);
    const startY = container.scrollTop;
    const distance = targetY - startY;
    const duration = 600;
    let startTime = null;

    function scrollStep(timestamp) {
        if (!startTime) startTime = timestamp;
        const elapsed = timestamp - startTime;
        const progress = Math.min(elapsed / duration, 1);

        container.scrollTop = startY + distance * progress;

        if (progress < 1) {
            window.requestAnimationFrame(scrollStep);
        }
    }
    window.requestAnimationFrame(scrollStep);
}

// 使用示例：滚动到容器底部
const container = document.getElementById('scrollable');
smoothScrollContainer('scrollable', container.scrollHeight);


// 处理滚动事件时，应使用节流(throttle)或防抖(debounce)减少处理频率：

// 节流函数实现
function throttle(func, limit) {
    let inThrottle;
    return function() {
        const args = arguments;
        const context = this;
        if (!inThrottle) {
            func.apply(context, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    };
}

// 应用节流
window.addEventListener('scroll', throttle(function() {
    console.log('处理滚动事件');
}, 100));
~~~


无限滚动加载

~~~js

let isLoading = false;

window.addEventListener('scroll', () => {
  const { scrollTop, scrollHeight, clientHeight } = document.documentElement;
  
  // 接近底部时加载更多内容
  if (scrollTop + clientHeight >= scrollHeight - 100 && !isLoading) {
    isLoading = true;
    loadMoreContent().finally(() => {
      isLoading = false;
    });
  }
});

async function loadMoreContent() {
  // 模拟API调用
  await new Promise(resolve => setTimeout(resolve, 1000));
  console.log('加载更多内容');
}
~~~