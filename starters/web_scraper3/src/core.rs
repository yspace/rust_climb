use std::collections::HashMap;
use async_trait::async_trait;

// @see https://leapcell.io/blog/dynamic-dispatch-and-dependency-injection-with-trait-objects-in-rust-web-services
#[async_trait]
// pub trait TargetSite {
// pub trait TargetSite: Send + Sync + Clone{
pub trait TargetSite: Send + Sync {

    // type Error: std::error::Error + Send + Sync + 'static; // Define an associated type for errors
    // async fn create_user(&self, username: &str, email: &str) -> Result<String, Self::Error>;


    // async fn run(&self);

    fn get_name(&self) -> String;
    fn get_title(&self) -> String;
    fn start_url(&self,user : &str) -> String;

    // ⚠️ 对于js中出现的函数 最好贴在浏览器的debugger面板中进行验证
    fn get_pagination_js(&self ) -> &str{
        &""
    }

    // @see https://ashishmisal.medium.com/pagination-vs-infinite-scroll-vs-load-more-data-loading-ux-patterns-in-react-53534e23244d
    fn get_infinite_scroll_js(&self ) -> &str{
        &""
    }

    fn get_image_urls_js(&self ) -> &str{
        &""
    }
 fn get_image_info_js(&self ) -> &str{
        &""
    }

}

// ================================= 《/ 插件系统 》  =====================================================
pub struct MySite {
    name: &'static str,
    /* ... */
    site: Box<dyn TargetSite>,
    // site2: Box<dyn TargetSite + Send + Sync>
}

impl MySite {
    pub const fn new(name: &'static str,site:  Box<dyn TargetSite>) -> Self {
        Self {  name ,site}
    }
}

// inventory::collect!(MySite);

// 😭 上面的不行哦


use anymap::AnyMap;

// @see https://users.rust-lang.org/t/function-type-registration-at-startup/23458/11
pub struct Initializer {
   // pub f: fn(&mut AnyMap),
   pub f: fn(&mut HashMap<String,Box<dyn TargetSite>>),
}

impl Initializer {
    pub const  fn for_type<T:TargetSite +  Default  + 'static >() -> Self {
        Self {
         f: |map|{
             // map.insert::<T>(T::default()) ;
             let site = T::default();
            map.insert(site.get_name(), Box::new(site));
         }
        }
    }
}

inventory::collect!(Initializer);


// =====================================《 插件系统 /》====================================================

use std::sync::Arc;
use crate::sites::SiteRavenel;
// use anyhow::Debug;

async fn print_the_name(from: Arc<dyn TargetSite>) {
    let name = from.get_name();
    println!("The name is {name}");
}

#[test]
fn test_4hashmap() {
    use std::collections::HashMap;
    // see: https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/async-traits.html
    // let targets : HashMap<String,Box<dyn TargetSite>> = HashMap::new();
    let targets : Vec<Box<dyn TargetSite>> = vec![];

    for target in targets {
        println!("{}", target.get_name());
    }
}

// 无限滚动 或者 loadmore 情况可据此更改！
fn js_load_more_tpl(){
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
}