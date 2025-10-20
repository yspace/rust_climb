use std::{path::PathBuf, borrow::Borrow};

use fantoccini::{ClientBuilder, Locator};
use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::{Regex, Captures};

use web_scraper3::*;

use std::time::Duration;

use visdom::types::BoxDynError;
use visdom::Vis;

use sanitize_filename;


lazy_static! {
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}
const DL_DIR: &'static str = "__downloads";

// ⚠️ 这个网站用了 shadowDom ：[Web Components详解-Shadow DOM基础](https://developer.aliyun.com/article/1557247)

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");



    // https://www.chanliu.com/product-tag/%e6%ba%a5%e5%84%92%ef%bc%88%e6%ba%a5%e5%bf%83%e7%95%ac%ef%bc%89/
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BA%A5%E5%84%92";
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BA%A5%E5%84%92";
    // 陈少梅
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E5%B0%91%E6%A2%85";
    // 唐寅
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%94%90%E5%AF%85";
    // 黄公望
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%BB%84%E5%85%AC%E6%9C%9B";
    // 王蒙
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%8E%8B%E8%92%99";
    // 倪瓒
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%80%AA%E7%93%92";
    // 高克恭
   // let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%AB%98%E5%85%8B%E6%81%AD";
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E9%95%87";
    // 巨然
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%B7%A8%E7%84%B6";
    // 马远
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%A9%AC%E8%BF%9C";
    // 夏圭
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%A4%8F%E5%9C%AD";
    // 李成
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9D%8E%E6%88%90";
    // 范宽
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E8%8C%83%E5%AE%BD";
    // 文征明
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%96%87%E5%BE%81%E6%98%8E";
    // 燕文贵
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%87%95%E6%96%87%E8%B4%B5";
    // 林良
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9E%97%E8%89%AF";
    // 蒋廷锡
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E8%92%8B%E5%BB%B7%E9%94%A1";
    // 陈洪绶
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E6%B4%AA%E7%BB%B6";
    // 任伯年
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E4%BB%BB%E4%BC%AF%E5%B9%B4";
    // 陈淳
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E6%B7%B3";
    // 徐渭
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%BE%90%E6%B8%AD";

    // 项元汴
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%A1%B9%E5%85%83%E6%B1%B4";
    // 项圣谟
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%A1%B9%E5%9C%A3%E8%B0%9F";
    // 马骀
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%A9%AC%E9%AA%80";
    // 石涛
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%9F%B3%E6%B6%9B";

    // 恽南田 恽寿平
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%81%BD%E5%8D%97%E7%94%B0";
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%81%BD%E5%AF%BF%E5%B9%B3";

    // 梅清
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%A2%85%E6%B8%85";
    // 弘仁
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%BC%98%E4%BB%81";
    // 八大山人
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%85%AB%E5%A4%A7%E5%B1%B1%E4%BA%BA";
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%87%91%E5%9F%8E";

    // 刘松年
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%88%98%E6%9D%BE%E5%B9%B4";
    // 潘天寿
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BD%98%E5%A4%A9%E5%AF%BF";
    // 吴镜汀
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E9%95%9C%E6%B1%80";
    // 吴湖帆
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E6%B9%96%E5%B8%86";
    // 萧朗
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E8%90%A7%E6%9C%97";

    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E5%8D%8A%E4%B8%81";
    // 江寒汀
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%B1%9F%E5%AF%92%E6%B1%80";

    // 华新罗
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%8D%8E%E6%96%B0%E7%BD%97";

    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9D%8E%E9%B9%A4%E7%AD%B9";

    // 马晋
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%A9%AC%E6%99%8B";
    // 陈之佛
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E4%B9%8B%E4%BD%9B";

    // 李可染
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9D%8E%E5%8F%AF%E6%9F%93";

    // 黎雄才
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%BB%8E%E9%9B%84%E6%89%8D";

    // 傅抱石
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%82%85%E6%8A%B1%E7%9F%B3";
    // 亚明
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E4%BA%9A%E6%98%8E";
    //
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%92%B1%E6%9D%BE%E5%B2%A9";
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E6%B9%96%E5%B8%86";
    // 吴华源
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E5%8D%8E%E6%BA%90";

    // 仇英
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E4%BB%87%E8%8B%B1";
    // 赵叔孺
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E8%B5%B5%E5%8F%94%E5%AD%BA";
    // 郭味渠
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%83%AD%E5%91%B3%E8%95%96";
    // 林风眠
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9E%97%E9%A3%8E%E7%9C%A0";
    // 吴作人
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E4%BD%9C%E4%BA%BA";
    // 张大千
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%BC%A0%E5%A4%A7%E5%8D%83";
    // 胡佩衡
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E8%83%A1%E4%BD%A9%E8%A1%A1";
    // 陈半丁
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%99%88%E5%8D%8A%E4%B8%81";
    // 金城
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%87%91%E5%9F%8E";
    // 孔小瑜
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%AD%94%E5%B0%8F%E7%91%9C";
    // 四溥之 溥伒
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%BA%A5%E4%BC%92";
    // 石鲁
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%9F%B3%E9%B2%81";
    // 刘海粟
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%88%98%E6%B5%B7%E7%B2%9F";
    // 程十发
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%A8%8B%E5%8D%81%E5%8F%91";
    // 王一亭（几与吴昌硕齐名）
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E7%8E%8B%E4%B8%80%E4%BA%AD";
    // 吴弗之
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%90%B4%E8%8C%80%E4%B9%8B";
    // 黄永玉 （未完...）
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%BB%84%E6%B0%B8%E7%8E%89";
    // 杜滋龄
     let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%9D%9C%E6%BB%8B%E9%BE%84";
    // 刘旦宅
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%88%98%E6%97%A6%E5%AE%85";

    // 文征明
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%96%87%E5%BE%81%E6%98%8E";
    // 沈周
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E6%B2%88%E5%91%A8";
    // 唐伯虎
   let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%94%90%E4%BC%AF%E8%99%8E";

    // 齐白石 （）
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%BD%90%E7%99%BD%E7%9F%B3";

    // 孙天牧
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%AD%99%E5%A4%A9%E7%89%A7";

    // 徐操
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%BE%90%E6%93%8D";
    // 邓石如
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E9%82%93%E7%9F%B3%E5%A6%82";
    // 卢坤峰
    let target_url = "https://www.cguardian.com/auctions/search?searname=%E5%8D%A2%E5%9D%A4%E5%B3%B0";
    let target_url = "https://www.cguardian.com/auctions/search?searname=荷花";
    let target_url = "https://www.cguardian.com/auctions/search?searname=松树";
    let target_url = "https://www.cguardian.com/auctions/search?searname=兰花";
    let target_url = "https://www.cguardian.com/auctions/search?searname=梅花";
    let target_url = "https://www.cguardian.com/auctions/search?searname=菊花";
    let target_url = "https://www.cguardian.com/auctions/search?searname=竹子";
    let target_url = "https://www.cguardian.com/auctions/search?searname=牡丹";
    let target_url = "https://www.cguardian.com/auctions/search?searname=祝枝山";
    let target_url = "https://www.cguardian.com/auctions/search?searname=葡萄";
    let target_url = "https://www.cguardian.com/auctions/search?searname=吴石仙";
    let target_url = "https://www.cguardian.com/auctions/search?searname=陈师曾";
    let target_url = "https://www.cguardian.com/auctions/search?searname=龚贤";
    //  邵逸轩 是傅抱石的老师
    let target_url = "https://www.cguardian.com/auctions/search?searname=邵逸轩";
    //  邵幼轩 是章金生的花鸟画老师
    let target_url = "https://www.cguardian.com/auctions/search?searname=邵幼轩";
    let target_url = "https://www.cguardian.com/auctions/search?searname=虚谷";
    let target_url = "https://www.cguardian.com/auctions/search?searname=钱舜举";
    let target_url = "https://www.cguardian.com/auctions/search?searname=邹传安";
    let target_url = "https://www.cguardian.com/auctions/search?searname=周之冕";
    let target_url = "https://www.cguardian.com/auctions/search?searname=叶浅予";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=王孟白";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=葉醉白";
    let target_url = "https://www.cguardian.com/auctions/search?searname=丰子恺";
    // 浙派五老
    // let target_url = "https://www.cguardian.com/auctions/search?searname=方增先";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=李振坚";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=周昌谷";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=顾生岳";
    // let target_url = "https://www.cguardian.com/auctions/search?searname=宋忠元";

    let target_url = "https://www.cguardian.com/auctions/search?searname=任重";
    let target_url = "https://www.cguardian.com/auctions/search?searname=陈子庄";
    let target_url = "https://www.cguardian.com/auctions/search?searname=王子武";
    let target_url = "https://www.cguardian.com/auctions/search?searname=钱瘦铁";
    let target_url = "https://www.cguardian.com/auctions/search?searname=溥伒";
    let target_url = "https://www.cguardian.com/auctions/search?searname=张大壮"; // todo ...
    // let target_url = "https://www.cguardian.com/auctions/search?searname=吴镜汀";





    c.goto(target_url).await?;

    // let last_page_link = c.wait().for_element(Locator::XPath("//center/a[last()-1]")).await?;
    let last_page_link = c.wait().for_element(Locator::XPath("//a[last()-1]")).await?;
    let last_page_href = last_page_link.attr("href").await ? ;
    // println!("last_page_href:{}", last_page_href.unwrap()) ;
    let last_page_num = get_page_num(last_page_href.unwrap().as_str()) ;

    let mut pre_url:Option<Url> = None;

    // 按人名搜索时可以不要这个 ; 按照主题时最好打开
    // let mut cate_flag = true ;
    let mut cate_flag = false ;

    loop {
        println!("enter_loop");

        let url = c.current_url().await?;


        #[allow(deprecated)]
        c.wait_for(move |_| {
            std::thread::sleep(Duration::from_secs(7));
            async move { Ok(true) }
        })
            .await?;


        println!("获取html...！>>>");
        let el_items = c
            .wait()
            .for_element(Locator::Css(".wujie_iframe"))
            .await?;
        println!("获取html...！");

        // -----------------------------执行异步js--------------------------------------------------
        const JS0: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
             const [ callback] = arguments;

             setTimeout(function(){
             // alert("哈哈哈😄");
             console.log("---->>>----");

             var js_ruslts = "some category";

             var root_ele = document.querySelector("wujie-app").shadowRoot;

                // 此处只选择书画类 即第一个分类元素
             const categoryEle = root_ele.querySelector("div.cat-item-list >div");
             if(categoryEle){
                   // console.log(categoryEle);
                    console.log("🤔 有分类呀");
                    // 点击分类
                    // 点击分页
                     categoryEle.click();
             }

              callback( js_ruslts );

             },1000) ;
        "#;

        if cate_flag {
            let mut js_result = c.execute_async(JS0, vec![]).await?;
            println!("[👀是否存在分类情况] js result here :{:?}", js_result);
            #[allow(deprecated)]
            c.wait_for(move |_| {
                std::thread::sleep(Duration::from_secs(5));
                async move { Ok(true) }
            })
                .await?;

            cate_flag = false;
        }


        const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！

            // const [date, callback] = arguments;
            const [ callback] = arguments;

             setTimeout(function(){
             // alert("哈哈哈😄");
             console.log("---->>>----");
            //  document.addEventListener("DOMContentLoaded", function() {
            //     // DOM树加载完毕后执行这段代码
            //     // var element = document.getElementById("yourElementId");
            //     // console.log(element);
            //
            //      console.log("----dom 🌲----");
            //     console.log(document.getElementById("child-app-container").innerHtml);
            // });

            // var text = document.querySelector("wujie-app").shadowRoot.querySelector("html").outerHtml;
            // var text = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage").outerHTML;
            // var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
            // var text = document.querySelector("wujie-app").shadowRoot.querySelector(".itemList").innerHTML;
            // document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
            // console.log(text);
            console.log("---->>>----");

            function addSuffixForDuplicates(arr) {
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

            // var my_f = function(){}
            function getCurrentPageImages()
            {
                var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("img.cgImage");
                // var ele_images = document.querySelector("wujie-app").shadowRoot.querySelectorAll("div.sudokuItem");
                var js_ruslts = [] ;
                for (let element of ele_images) {
                  // console.log(element);

                  let s_img_src = element.src;
                  let l_img_src = s_img_src.replace("/s.jpg", "/l.jpg");

                  js_ruslts.push(l_img_src);
                }

                  var ele_infos = document.querySelector("wujie-app").shadowRoot.querySelectorAll("div.sudokuItem .infoname");
                var js_ruslts2 = [] ;
                for (let element of ele_infos) {
                  // console.log(element);
                  var title = element.textContent ;
                  console.log(title);

                  js_ruslts2.push(title);
                }

                js_ruslts2 =  addSuffixForDuplicates(js_ruslts2);
                js_ruslts = zip(js_ruslts,js_ruslts2);


                return js_ruslts;
            }

             var js_ruslts =
             {
               items: getCurrentPageImages(),
               need_page: false
             };
              // getCurrentPageImages() ;

             // 分页逻辑
             console.log('分页逻辑开始....');

             var cond_need_page = true ;
             // TODO 这里逻辑好像有问题 😄
             while(cond_need_page == true) {
               var root_ele = document.querySelector("wujie-app").shadowRoot;

                const currentPage = root_ele.querySelector('li.is-active.number');
                const nextPage = currentPage.nextElementSibling;
                console.log(nextPage);

                var pageNumber = currentPage.innerText;
                js_ruslts['page_number'] = pageNumber ;

                // 检查是否存在下一个兄弟元素节点
                if (nextPage !== null) {
                     //var attributeValue = nextSibling.getAttribute('属性名');
                     // console.log(attributeValue);
                     console.log('模拟下点击分页');

                      cond_need_page = false ;
                     js_ruslts['need_page'] = true ;
                     // 点击分页
                     nextPage.click();

                } else {
                    // 分页结束
                    console.log('不存在下一个兄弟元素节点');
                    cond_need_page = false ;

                    // 通知rust

                    // callback('END');
                    js_ruslts['need_page'] = false ;
                }
             }

                 // callback( "hi heree" )
              callback( js_ruslts );

             },1000) ;
        "#;

        let mut js_result = c.execute_async(JS, vec![]).await?;
         println!("js result here :{:?}", js_result);
        // -------------------------------------------------------------------------------
        // 检查是否为数组
        // https://docs.rs/serde_json/latest/serde_json/enum.Value.html
        use serde_json::Value;

        let mut need_page = true;
        /// js_result = js_result["items"].as_array().unwrap() ;

        // 页码
        let mut page_number ;


        // while js_result.is_array() {
        while need_page {

            page_number = js_result["page_number"].as_str().unwrap().to_string();

            need_page = js_result["need_page"].as_bool().unwrap();


            for value in js_result["items"].as_array().unwrap() {
                match value {
                    Value::String(s) => {
                        // println!("the src is :{}", s) ;

                        let parts: Vec<&str> = s.split("|").collect();
                        let img_url = parts[0];
                        let img_title = parts[1];
                        println!("the src is :{} and title is : {}", img_url, img_title) ;
                        // =================================== << 下载 =====================

                        let img_src = img_url;

                        // https://stackoverflow.com/questions/43973219/is-there-a-safe-sanitised-filename-function-in-rust
                        // https://docs.rs/filenamify/latest/filenamify/
                        // https://www.npmjs.com/package/sanitize-filename
                        // https://docs.rs/sanitise-file-name/latest/sanitise_file_name/index.html
                        let mut img_save_to_name =  /* img_title; */
                        sanitize_filename::sanitize(img_title);
                        img_save_to_name.push_str(format!("_p{}_", page_number).as_str() ) ;
                        img_save_to_name.push_str(".jpg");

                        let url2 = url.clone().to_string();
                        // let url_segments = url2.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
                        // let folder_name = url_segments.get(url_segments.len()-0).unwrap();
                        //
                        let parts: Vec<&str> = url2.split("=").collect();
                        let folder_name = urlencoding::decode(parts[1]).unwrap();

                        let folder_name:&str = folder_name.borrow();

                        // let img_url = url.join(img_save_to_name.as_str())?;
                        let img_url = img_src;


                        // 下载
                        let mut download_dir = PathBuf::from(DL_DIR);
                        download_dir.push(folder_name);
                        let save_to_folder = download_dir.as_path() ;

                        if !std::path::Path::new(save_to_folder).is_dir(){
                            std::fs::create_dir_all(save_to_folder).expect("can't create download folder!");
                        }

                        let mut save_to_path = std::path::Path::new(save_to_folder)
                            // .join(folder_name)
                            .join(img_save_to_name) ;
                        println!("save to path: {}", save_to_path.display());

                        let mut download_flag = false;

                        if save_to_path.exists(){
                            let matedata = std::fs::metadata(save_to_path.clone())?;
                            if matedata.is_file() {
                                if   matedata.len() == 0 {
                                    // 文件存在且是空文件
                                    download_flag = true;

                                }else{
                                    // 已经下载过了
                                    println!("文件已下载! 不需要重复下载");
                                }
                            }
                        }else{
                            // 不存在
                            download_flag = true;
                        }

                        if download_flag {

                            println!("now will download it! {}",save_to_path.display()) ;

                            // TODO 下载老出现超时 😢 https://docs.rs/tokio/latest/tokio/macro.select.html 可以尝试加入超时机制
                            let rslt = download::fetch_url2(
                                img_url.to_string(),
                                save_to_path.to_str().unwrap().to_string(),
                            )
                                .await;
                            if rslt.is_err() {
                                println!("some error happened :{:?}", rslt.err());
                            }else{

                                println!("download ok!") ;
                            }
                        }

                        // =================================== 下载 >> ======================


                    },
                    _ => println!("Not a string"),
                }
            }
            // 继续下一页
            if need_page {
                println!("next page ⛽️⛽️" );
                js_result = c.execute_async(JS, vec![]).await?;
                println!("js result here :{:?}", js_result);
            } else{
                println!("this is the last page! 😄" );
            }


            #[allow(deprecated)]
            c.wait_for(move |_| {
                std::thread::sleep(Duration::from_secs(10));
                async move { Ok(true) }
            })
                .await?;
        }
        // }if js_result.is_array() {
        //     for value in js_result.as_array().unwrap() {
        //         match value {
        //             Value::String(s) => println!("the src is :{}", s),
        //             _ => println!("Not a string"),
        //         }
        //     }
        // } else {
        //     println!("Not an array");
        // }

        // let source = c.source().await?;
        // let source = js_result.as_str().unwrap();
        // println!("js result here :{:?}", source);

        // 在rust端处理元素 析取数据
        // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
        // load html
        // let root = Vis::load(source).unwrap();
        // let my_img = root.find("img");
        // // let lis = root.find("#header li");
        // // let my_text = my_img.text();
        // println!("how many image 😄 :{}", my_img.length());
        // let href = links.attr("href").unwrap().to_string();
        //
        // let dom_detail = root.find("div.project-detail");
        // let ele_title = dom_detail.find("h3");
        // let title = ele_title.text();
        // let detail = dom_detail.html();
        // println!("获取source...！{}", source);

        // let el_items_html = el_items.html(false).await?;
        // let el_items_html = el_items.text().await?;
        // let el_items_html = el_items.html(false).await?;
        //
        //  println!("html: {}", el_items_html);

        println!("ele_imgs >>");




        let current_page_num = get_page_num(url.as_str());
        
        if current_page_num > last_page_num {
            println!("last page reached");
                break;
        }
        /* 
        if pre_url.is_some() {
            //  let pre_url2 = pre_url.unwrap() ;
           
             if url == pre_url.unwrap() {
                println!("last page reached");
                break;
            }
           
            // println!("{}: {}", pre_url2.as_str(), url.as_str());
        }*/



        // xpath 选择表达式 ;
        let selector = "//center//a[last()]"; //  双斜杠，表示不管层级关系
                                              // xpath 有大量内置函数 可以协助提取信息如 text()
        let button = c.wait().for_element(Locator::XPath(selector)).await?;

        // let source_html = c.source().await? ;
        // println!("{}", source_html);






       // NOTE: 双斜杠 取到img哪里去了 所以一直有问题！

       let button = c.wait().for_element(Locator::XPath("//center/a[last()]")).await?;
        // button.click().await?;
        let next_page_href = button.attr("href").await?.unwrap() ;
        println!("next href: {}", next_page_href.as_str());
        let next_page_num = get_page_num(next_page_href.as_str()) ;
        let next_page_url = get_page_url( url.as_str(),next_page_num); 
        c.goto(next_page_url.as_str()).await ;
       // c.goto(next_page_href.as_str()).await?;
        pre_url = Some(url);
    }

    // // click "Foo (disambiguation)"
    // c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // // click "Foo Lake"
    // c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    // let url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}

async fn _main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // click "Foo (disambiguation)"
    c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // click "Foo Lake"
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}

fn get_page_num(item: &str) -> usize {
    if PAGE_REGEX.is_match(item) {
        println!("OK:{:#?}", item);
        let cap_names = PAGE_REGEX.captures(item).unwrap();
        println!("{:#?}", cap_names); // 这个结构有点像hash_map 呀
                                      // let page = cap_names.unwrap().name("n");
                                      // println!("{:#?}", page);

        let page_num = cap_names.name("n").unwrap();
        //println!("{:#?}", page_num.as_str());
        let page: i32 = page_num.as_str().parse::<i32>().unwrap();
        return page as usize;
    }
    0
}
fn get_page_url(item: &str , page: usize) -> String {
    if PAGE_REGEX.is_match(item) {
        println!("OK:{:#?}", item);
        let k = PAGE_REGEX.replace(item, |caps: &Captures| {
            format!("page={}", page) });
        return k.as_ref().to_string();
    }else{
        return "".to_owned() ;
    }
     
}

#[test]
fn test_get_page_url() {
    let url = "index.asp?page=10";
    let url2 = get_page_url(url,12);
    assert_eq!(url2,"index.asp?page=12")
}