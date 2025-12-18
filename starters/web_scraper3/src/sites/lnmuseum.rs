use crate::TargetSite;
use crate::MySite;
use crate::Initializer;



// #[derive(Default, Debug,Copy, Clone)]
#[derive(Default, Debug, Clone)]
pub struct SiteLNMuseum{
    search_user: String ,
    // search_user: std::cell::RefCell<String>,
    // search_user: &dyn str + 'static,
    //  search_user: std::sync::Arc<String>,
}


#[async_trait::async_trait]
impl  TargetSite   for SiteLNMuseum{
    fn get_name(&self) -> String {
        String::from("lnmuseum")
    }
    fn get_title(&self) -> String {
        String::from("辽宁省博物馆")
    }

    fn start_url(&self, user: &str) -> String {
        // 辽博 非传统页面 搜索某个用户是比较麻烦的
        // let target_url = format!("https://www.lnmuseum.com.cn/#/collect/digital-culture/{}", user) ;
        let target_url = format!("https://www.lnmuseum.com.cn/#/collect/digital-culture") ;

        // self.search_user = user.to_string() ; todo: 需要某种内部可变性机制
        //
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

            function legacySyncFetch(url) {
               const xhr = new XMLHttpRequest();
               xhr.open('GET', url, false); // 同步模式
               xhr.send(null);
               return JSON.parse(xhr.responseText);
            }

            // 😄TODO：这里因页面不同逻辑不通
            // var my_f = function(){}
            function getCurrentPageItems()
            {
                // ⚠️注意左侧的某些按钮需要选中☑️ 并且只需要做一次
                 if( window.__globalWindowVar == null ){
                    // var category = 24 ; // 2 是绘画 24两页比较少！
                    var category =  2 ;
                    var ele = document.querySelector(`.el-checkbox-group.check-groups .el-checkbox:nth-of-type(${category})`);
                    ele.click() ;
                    window.__globalWindowVar = true ;
                 }

                // ⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️⚠️ 这里是获取当前页面的分页链接的css选择逻辑 为主要逻辑
                var currentPageSelector = "ul.el-pager>li.number.active" ;
                var currentPageEle = document.querySelector( currentPageSelector );

                var currentPageNum = currentPageEle.innerText;

                var url = 'https://www.lnmuseum.com.cn/singleMuseum/szwwkt/list?_t=1765020228&year=&category=2&name=&currentPage=__PAGE__&size=15';
                url = url.replace('__PAGE__',currentPageNum);

                var ajaxResults = legacySyncFetch(url);

                var js_ruslts = [] ;
                for (let element of ajaxResults['result']) {
                  // console.log(element);
                    js_ruslts.push( buildAuctionDetailUrl(element.collectId));
                  // 🤔 其实可以在这里一并处理 获取相关的信息
                }

                return js_ruslts;
            }

             // 😄TODO：这里因页面不同逻辑不通
            function getNextPageLink(){
                result = null ;

                var nextPageSelector = 'button.btn-next' ;
                var element = document.querySelector(nextPageSelector);

                if(element && element.getAttribute('disabled') == null){
                   return element ;
                }

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

               var result_ele = document.querySelector(".header span.count") ;
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

                // const detailUrlTpl = 'https://www.artron.net/auction/detail/__ID__?from=search';
                const detailUrlTpl = 'https://www.lnmuseum.com.cn/#/collect/detail?id=__ID__&pageType=3';
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

                     var isNeedPage = getIsNeedPage();
                     var js_ruslts =
                     {
                       items: getCurrentPageItems(),
                       // total: data.data.total,
                       need_page: isNeedPage,
                       load_more: isNeedPage
                     };

                     if(isNeedPage) {
                         document.documentElement.scrollIntoView({ behavior: 'smooth' });
                         window.scrollTo(0, document.body.scrollHeight-120);
                         // TODO 点击下一页
                         // js 模拟点击分页链接
                         goToNextPage();
                     }

                     callback( js_ruslts );

             },6000) ;
        "#;
        return JS_PAGINATION ;
    }



    fn get_image_urls_js(&self) -> &str {
        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;


              // 获取大图地址 并异步返回给rust

              setTimeout(function(){
                    var itemCssSelector = ".swiper-wrapper .swiper-slide img" ;
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
                         return src.split('&type').shift();;
                        // return src.replace('/M','/L');
                    });
                    // 去重
                    src_list = Array.from(new Set(src_list));

                    callback(src_list);

              }
              , 3000); // 给3秒加载图片 应该差不多了😄




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

               var  ele = document.querySelector('p.collect-title');

                var resultText = ele.innerText;


                // 获取唯一id 做后缀区分不同图片的名称 防止冲突 ；
                var currentUrl = window.location.href; // 当前url获取有很多方法
                // let regex = /lotsIn\/\d+/g; // 使用 'g' 标志进行全局搜索
                // let matches = currentUrl.match(regex);
                // console.log(matches);
                // var suffix = currentUrl.split('/').pop();
                // let suffix = matches[0].replace('/','_');
                let suffix = currentUrl.split('id=').pop().split('&').shift().replace('/','_');


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
// impl SiteLNMuseum {
//     pub const fn new( ) -> Self {
//         Self { }
//     }
// }
//
// inventory::submit! {
//     MySite::new("ravenel", Box::new(SiteLNMuseum::new()))
// }

inventory::submit! {
    Initializer::for_type::<SiteLNMuseum>()
}



#[test]
fn it_works() {


}



