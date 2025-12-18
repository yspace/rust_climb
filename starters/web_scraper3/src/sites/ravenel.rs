use crate::TargetSite;
use crate::MySite;
use crate::Initializer;



#[derive(Default, Debug,Copy, Clone)]
pub struct SiteRavenel;


#[async_trait::async_trait]
impl  TargetSite   for SiteRavenel{
    fn get_name(&self) -> String {
        String::from("ravenel")
    }
    fn get_title(&self) -> String {
        String::from("羅芙奧藝術集團")
    }

    fn start_url(&self, user: &str) -> String {
        let target_url = format!("https://ravenel.com/search/{}", user) ;
        return target_url;
    }

    fn get_pagination_js(&self) -> &str {

        const JS_PAGINATION: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            console.log("😄 开始执行js啦！");

            function addSuffix4Duplicates(arr) {
                let newArr = [];
                let countMap = {}; // 用于记录每个元素出现的次数

                for (let i = 0; i < arr.length; i++) {
                    let element = arr[i];
                    let suffix = '';
                    if (countMap[element]) {
                        suffix = `_${countMap[element]}`; // 添加后缀
                        countMap[element]++; // 更新计数
                    } else {
                        countMap[element] = 1; // 初始化计数
                    }
                    newArr.push(element + suffix); // 将带有后缀的元素添加到新数组中
                }

                return newArr;
            }

            const zip = (arr1, arr2) => arr1.map((item1, index) => {
               return  item1+'|'+  arr2[index] ; //[num, arr2[index]]
             }
            );


            // 😄TODO：这里因页面不同逻辑不通
            // var my_f = function(){}
            function getCurrentPageItems()
            {

                // ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️ 这里是获取当前页面的分页链接的css选择逻辑 为主要逻辑
                var itemCssSelector = ".cate-products-content-list .cate-imges-wrap-l>a" ;
                var elements = document.querySelectorAll( itemCssSelector );


                var js_ruslts = [] ;
                for (let element of elements) {
                  // console.log(element);

                  let s_item_href = element.href;

                  js_ruslts.push(s_item_href);
                  // 🤔 其实可以在这里一并处理 获取相关的信息
                }

                // 🐯这里获取链接的名字 可以作为文件存储的名称 但也可以在单个页面中获取 哪里信息更全面
                //  var ele_infos = document.querySelectorAll(itemCssSelector+' h3>a');;
                // var js_ruslts2 = [] ;
                // for (let element of ele_infos) {
                //   // console.log(element);
                //   //
                //   var title = element.textContent ; // element.innerText ;
                //   console.log('目标item名称--- >>>'+title);
                //   js_ruslts2.push(title);
                // }

                // js_ruslts2 =  addSuffix4Duplicates(js_ruslts2);
                // js_ruslts = zip(js_ruslts,js_ruslts2);


                return js_ruslts;
            }

             // 😄TODO：这里因页面不同逻辑不通
            function getNextPageLink(){
                result = null ;

                var elements = document.querySelectorAll('div.listJump a');
                var text = "下一页";
                var text2 = "下一個";
                var found = false;

                elements.forEach(function(element) {
                    if (element.textContent.includes(text) || element.textContent.includes(text2)) { // 或者使用 element.innerText.includes(text)
                        found = true;

                        console.log("找到了元素", element);
                         result = element

                        return; // 如果只需要找到第一个匹配的元素，可以使用 return 终止循环
                    }
                });

                return result ;
            };

            // 将字符串转换为数字
            function extractAndConvertToNumber(str) {
                // 使用正则表达式匹配字符串中的所有数字
                const numbers = str.match(/\d+/g);
                // 如果匹配到数字，则将它们连接起来并转换为数字
                if (numbers) {
                    return parseInt(numbers.join(''), 10);
                }
                // 如果没有匹配到数字，返回0或其他你希望返回的值
                return 0;
            }
            function getSearchResultTotals(){
              var result = 0 ;

               var result_ele = document.querySelector(".active-tab span:last-child") ;
               // var total_number = result_ele.innerText;
               if(result_ele){
                  var total_number = result_ele.innerText ;
                  result = extractAndConvertToNumber(total_number);
               }

              return result;
            }

            function getIsNeedPage(){

                var nextPageLink = getNextPageLink();
                if(nextPageLink == null){
                     return false ;
                }else{
                 return true;
                }
            }
            function goToNextPage(){
                 if(getIsNeedPage()) {



                        const nextPage =getNextPageLink();
                        console.log(nextPage);

                        // 检查是否存在下一个兄弟元素节点
                        if (nextPage !== null) {
                             console.log('模拟下点击分页');

                             // 点击分页
                             nextPage.click();

                        }
                 }
            }
            // 获取当前页面页码
            function getCurrentPageNum(){
               // 🐱可选逻辑 获取当前页面的页码 也可以通过分析location 浏览器url获取
               var currentPageLink = document.querySelector("div.listJump a.cur") ;
               var pageNumber = currentPageLink.innerText;
               if(currentPageLink != null) {
                     var pageNumber = currentPageLink.innerText;
                    return pageNumber ;
               }else{
                    return 0 ;
               }
            }
            // 获取关键字
            function getKeyword(){
                // 获取查询字符串参数对象
                var params = new URLSearchParams(window.location.search);
                var paramValue = params.get('keyword'); // 注意：这将返回null如果没有找到对应的键。
                console.log('🐞： keyword is '+ paramValue);
                return paramValue;
            }
            // 构造详情页url
            function buildAuctionDetailUrl(id){
                // return `https://www.artron.net/auction/detail/${id}?from=search`;

                const detailUrlTpl = 'https://www.artron.net/auction/detail/__ID__?from=search';
                return detailUrlTpl.replace('__ID__',id);
            }

            // 滚动到底部
            // document.documentElement.scrollIntoView({ behavior: 'smooth' });
            // window.scrollTo(0, document.body.scrollHeight-120);

            // console.log("is_need_page:"+ getIsNeedPage());

            // ⚠️ 异步执行
            //  setTimeout(function(){
             setTimeout(()=>{
                    // console.log("🐞getNextPageLink:"+ getNextPageLink());
                    console.log("🐞getIsNeedPage:"+ getIsNeedPage());
                     console.log("---->>>----");

                     var currentPageItems = getCurrentPageItems();
                     var currentPageItemsCount = currentPageItems.length ;

                     var isNeedPage =  false ;  // getIsNeedPage();
                     // 全局变量
                     if( window.__globalWindowVar ){
                            window.__globalWindowVar = window.__globalWindowVar + currentPageItemsCount ;
                     }else{
                          window.__globalWindowVar =  currentPageItemsCount ;
                     }
                     isNeedPage = window.__globalWindowVar < getSearchResultTotals();



                     var js_ruslts =
                     {
                       items: currentPageItems,
                       // total: data.data.total,
                       need_page: isNeedPage,
                       // has_more: isNeedPage, // hasNextPage
                       load_more: isNeedPage
                     };

                     if(isNeedPage) {
                         // 删除当前页 元素
                         var itemCssSelector = ".cate-products-content-list" ;
                         var elements = document.querySelectorAll( itemCssSelector );

                         elements.forEach(function(element) {
                             element.parentNode.removeChild(element);
                         })
                         document.documentElement.scrollIntoView({ behavior: 'smooth' });
                         window.scrollTo(0, document.body.scrollHeight-120);
                     }
                     // callback( "any thing return to rust!" )
                     callback( js_ruslts );

             },9000) ;
        "#;
        return JS_PAGINATION ;
    }
    fn get_infinite_scroll_js(&self) -> &str {
        const JS_SCROLL_FOR_LOAD_MORE: &'static str = r#"
         // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            console.log("😄 开始执行js啦！");

            function getResultsTotal(){
                // 获取所有符合xpath表达式的元素
                var result = document.evaluate('//p[@data-testid="results-total"]', document, null,XPathResult.ANY_TYPE, null);
                // 获取第一个元素
                var node = result.iterateNext();
                // console.log(node.innerText);
                var resultText = node.innerText;
                return extractAndConvertToNumber(resultText);

            }

            // 😄TODO：这里因页面不同逻辑不通
            // var my_f = function(){}
            function getCurrentPageItems()
            {

               // 获取所有符合xpath表达式的元素
               //     var result = document.evaluate('//p[@data-testid="results-search-item"]', document, null,XPathResult.ANY_TYPE, null);
               // 获取所有div元素
                var xpathResult = document.evaluate('//div[starts-with(@class, "SearchResultsGrid_lotCardWrapper__")]', document, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);

                var allElements = [];
                for (var i = 0, length = xpathResult.snapshotLength; i < length; ++i) {
                    allElements.push(xpathResult.snapshotItem(i));
                }
                //console.log(allElements);
                return allElements;
                // 🤔其实可以返回所有待处理项的url的


                // 🐶👇是大致逻辑
                // var elements = document.querySelectorAll("ul.artworks>li>a");
                // var js_ruslts = [] ;
                // for (let element of elements) {
                //   // console.log(element);
                //
                //   let s_item_href = element.href;
                //
                //   js_ruslts.push(s_item_href);
                //   // 🤔 其实可以在这里一并处理 获取相关的信息
                // }
                //
                // return js_ruslts;
            }

            // 将字符串转换为数字
            function extractAndConvertToNumber(str) {
                // 使用正则表达式匹配字符串中的所有数字
                const numbers = str.match(/\d+/g);
                // 如果匹配到数字，则将它们连接起来并转换为数字
                if (numbers) {
                    return parseInt(numbers.join(''), 10);
                }
                // 如果没有匹配到数字，返回0或其他你希望返回的值
                return 0;
            }

            // 滚动到底部
            function smoothScroll(targetY) {
                  const startY = window.scrollY;
                  const distance = targetY - startY;
                  const duration = 1000; // 动画持续时间(ms)
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
            function getElementTop(element) {
                const rect = element.getBoundingClientRect();
                return rect.top + window.scrollY;
            }

            function scrollToBottom(){
                // 示例用法
                const element = document.getElementById('BrightSpotFooter');
                const topPosition = getElementTop(element);
                console.log(topPosition);
                // 使用示例：滚动到页面底部
                smoothScroll(topPosition - 150);

            }

            console.log("共有："+getResultsTotal());

            function recursiveTimeout(count) {

                console.log(count);
                console.log(count < getResultsTotal());
                if (count < getResultsTotal()) {
                    setTimeout(function() {

                        scrollToBottom();
                        document.documentElement.scrollIntoView({ behavior: 'smooth' });

                        recursiveTimeout(0); // 😺这里条件不满足会一直滚动的
                    }, 10000); // 延迟10000毫秒后再次调用
                }
            }

            // recursiveTimeout(0);

            // 滚动到底部去
            scrollToBottom();
            // ⚠️ 异步执行
             setTimeout(function(){


                     console.log("---->>>----");
                     console.log("---->>>----");

                    var items = getCurrentPageItems();

                     var js_ruslts =
                     {
                       items: []  , // getCurrentPageItems(),
                       need_page: false ,
                       // load_more:  items.length < getResultsTotal(),
                       load_more:  document.querySelector('#loadMore') != null ,
                       loaded_progress:""+items.length+ "/" +getResultsTotal()
                     };

                     // callback( "hi heree" )
                     callback( js_ruslts );

             },7000) ;

        "#;
        return JS_SCROLL_FOR_LOAD_MORE ;
    }

    fn get_image_urls_js(&self) -> &str {
        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;


              // 获取大图地址 并异步返回给rust

              setTimeout(function(){
                    var itemCssSelector = ".cate-img-wrap-s img" ;
                    var eles = document.querySelectorAll( itemCssSelector ) ;
                    console.log(eles) ;

                    var src_list = [] ;
                    eles.forEach(function(ele) {
                        // console.log(ele.src);
                        src_list.push(ele.src);
                    });

                    // ⚠️ 如果是空的 可能不属于相册情形 就只有一张图片啦！
                    // if(eles.length == 0){
                    //     // alert('一张图片呀');
                    //     var ele = document.querySelector("div.productBigBox img") ;
                    //      src_list.push(ele.src);
                    //
                    // }

                    // 没有特殊计算就不用做这个！
                    // 所有的图片url做原地址获取
                    src_list = src_list.map(function(src){
                        // return src.split('src=').pop().split('&').shift();;
                        return src.replace('/M','/L');
                    });


                    callback(src_list);

              }
              , 5000); // 给3秒加载图片 应该差不多了😄




        "#;
        return JS ;
    }


    fn get_image_info_js(&self) -> &str {
        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            function getItemInfo(){

               var  ele = document.querySelector('p.cate-pdetail-name');

                var resultText = ele.innerText;


                // 获取唯一id 做后缀区分不同图片的名称 防止冲突 ；
                var currentUrl = window.location.href; // 当前url获取有很多方法
                // let regex = /lotsIn\/\.+/g; // 使用 'g' 标志进行全局搜索
                // let matches = currentUrl.match(regex);
                // console.log(matches);
                // var suffix = currentUrl.split('/').pop();
                // let suffix = matches[0].replace('/','_');
                let suffix = currentUrl.split('/').pop().replace('/','_');


                return resultText + '_'+ suffix;

            }

          setTimeout(function(){

                callback(getItemInfo());
          }
          , 6000); // 给6秒加载图片 应该差不多了😄

        "#;
        return JS ;
    }


}

//
// impl SiteRavenel {
//     pub const fn new( ) -> Self {
//         Self { }
//     }
// }
//
// inventory::submit! {
//     MySite::new("ravenel", Box::new(SiteRavenel::new()))
// }

inventory::submit! {
    Initializer::for_type::<SiteRavenel>()
}



#[test]
fn it_works() {


}