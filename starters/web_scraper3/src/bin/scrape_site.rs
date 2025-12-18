use std::{path::PathBuf, borrow::Borrow, usize};
use std::ops::Deref;
use fantoccini::{ClientBuilder, Locator};
use fantoccini::actions::{Actions, InputSource, KeyAction, KeyActions, MouseActions, NullActions, PointerAction, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_RIGHT};
use fantoccini::key::Key;

use url::{Host, ParseError, Url};

use lazy_static::lazy_static;
use regex::{Regex, Captures};

use web_scraper3::*;

use std::time::Duration;
use reqwest::Client;
use visdom::types::BoxDynError;
use visdom::Vis;

use sanitize_filename;
use tokio::time::sleep;
use webdriver::capabilities::Capabilities;

// use std::collections::HashSet;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use sled::{Db, Config};
use cacache;

use argh::FromArgs;

use global_registry::{TypeRegistry, register};
use tokio::sync::Mutex;

// 👀： cargo run -p web_scraper3 --bin artron -- --user puru --debug true -h true -c true -s 1431 --resumption

// @see https://docs.rs/argh/latest/argh/
#[derive(FromArgs, Debug)]
/// Reach new heights.
struct AppArgs {
    // /// whether or not to jump
    // #[argh(switch, short = 'j')]
    // jump: bool,

    /// whether  remove cache
    #[argh(switch,  short = 'r' )]
    rm_cache:bool,

    /// 中断进程恢复 (Interrupted Process Resumption) :
    #[argh(switch,  short = '0' )]
    resumption:bool,

    /// reverse order 逆向遍历 前提条件是缓存存在:
    #[argh(switch )]
    rev_order:bool,

    /// how high to go
    #[argh(option)]
    height: Option<usize>,

    /// an optional username for searching
    #[argh(option)]
    user: Option<String>,

    /// an optional headless flag
    #[argh(option, short = 'h', default = "false" )]
    headless:bool,

    /// an optional headless flag
    #[argh(option, short = 'c', default = "false" )]
    cache:bool,



    /// an optional headless flag
    #[argh(option, short = 's', default = "0" )]
    skip :usize,

    // #[argh(switch, short = 'h', default = "false" )]
    // headless: Option<bool>,

    #[argh(option, short = 'd' , default = "false", description = "debug or not")]
    debug: bool,
}

lazy_static! {
    // url中 当前页码 正则匹配
    static ref PAGE_REGEX: Regex = Regex::new(r"page=(?P<n>\d+)").unwrap();
}

// const vs static : Use const if you don't have a reason to use static.
const DL_DIR: &'static str = "__downloads";
pub(crate) const PROGRAM_NAME: &str = "artron";
pub(crate) const PROGRAM_PREFIX: &str = "program___site_name___";
pub(crate) const PROGRAM_DB: &str = "__data/__site_name__";
pub(crate) const PROGRAM_SITE_TPL: &str = "__site_name__";


fn get_prog_db() -> String {

    // Retrieve and use the registered type
    let config = TypeRegistry::global()
        .get_cloned::<AppContext>()
        .unwrap();


    let  target_site = config.target_site;

    let mut db_path = String::from(PROGRAM_DB);
    db_path = db_path.replace(PROGRAM_SITE_TPL,target_site.get_name().as_str());
    println!("[🐞db_path:] {db_path} ");
    return db_path ;
}

fn get_db_path() -> String {
    // Retrieve and use the registered type
    let config = TypeRegistry::global()
        .get_cloned::<AppContext>()
        .unwrap();

    return config.site_name.clone();

}

/// 获取缓存地址
fn get_cache_dir()-> String {


    let cache_dir = format!("{}/cache" ,get_db_path());
    return cache_dir ;
}
fn build_progress_cache_key(factor: &str)-> String {
    let key = format!("progress_{}", factor, );
    return key ;
}

lazy_static! {
    pub static ref ROOT_DB: sled::Db = Config::default()
        .path(PROGRAM_DB)
        .open()
        .expect("Unable to open root database");
}

// #[derive(Debug)]
// #[derive(Clone, Debug)]
#[derive(Clone)]
pub struct AppContext {
    // tip: String,
    site_name: String,
    db: sled::Db,
    // target_site: Box<dyn TargetSite>,
    target_site: Arc<dyn TargetSite>,
}


mod v2{
    use super::*;
    use tokio::task::JoinHandle;
    use tokio::time::{sleep, Duration};
    use std::error::Error;
    use anyhow::{Context, Result};

    // 示例任务函数，可能失败
    async fn risky_task(id: u64) -> Result<()> {
        println!("任务 {} 开始执行", id);
        // 模拟可能失败的操作
        // if id % 2 == 0 {
        //     return Err(anyhow::anyhow!("模拟错误: 任务 {} 失败", id));
        // }
        // sleep(Duration::from_millis(100)).await;

        let _rslt = real_main().await?;

        println!("任务 {} 成功完成", id);
        Ok(())
    }

    // 重启逻辑：捕获错误并重试
    async fn restartable_task(id: u64, max_retries: u32) -> Result<()> {
        let mut attempt = 0;
        loop {
            let handle = tokio::spawn(async move {
                risky_task(id).await
            });

            // 等待任务完成并捕获结果
            match handle.await {
                Ok(Ok(())) => {
                    println!("任务 {} 成功，无需重启", id);
                    return Ok(());
                }
                Ok(Err(e)) => {
                    attempt += 1;
                    if attempt >= max_retries {
                        eprintln!("任务 {} 失败，已达到最大重试次数: {}", id, e);
                        return Err(e);
                    }
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1)); // 指数退避
                    println!("任务 {} 失败 (尝试 {}/{}): {}. 等待 {:?} 后重试...",
                             id, attempt, max_retries, e, delay);
                    sleep(delay).await;
                }
                Err(join_err) => {
                    attempt += 1;
                    if attempt >= max_retries {
                        eprintln!("任务 {} panic: {}", id, join_err);
                        return Err(anyhow::anyhow!("任务崩溃: {}", join_err));
                    }
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                    println!("任务 {} panic (尝试 {}/{}): {}. 等待 {:?} 后重试...",
                             id, attempt, max_retries, join_err, delay);
                    sleep(delay).await;
                }
            }
        }
    }

    // 主函数示例
    // #[tokio::main]
    pub async fn main() -> Result<()> {
        // 启动多个重启任务
        let mut handles = vec![];

        for i in 1..=1 {
            let handle = tokio::spawn(async move {
                restartable_task(i, 3).await
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        Ok(())
    }

}

// ⚠️ 这个网站用了 shadowDom ：[Web Components详解-Shadow DOM基础](https://developer.aliyun.com/article/1557247)

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {

    let _result = v2::main().await ;

    Ok(())
}

// ⚠️ 最初版本这个方法就是主方法 为了支持重试 才在外侧包围了一些其他逻辑 如果👆方法有问题可以回到这个原始版本来
async fn real_main() -> Result<(), fantoccini::error::CmdError> {
    // 命令行参数解析
    let app_args: AppArgs = argh::from_env();

    println!("[debug] 🐞 app args: {:?}", app_args);

    // let mut registry = anymap::AnyMap::new();
    let mut registry : HashMap<String,Box<dyn TargetSite>> = HashMap::new();
    // let mut registry = BTreeMap::new();
    // Access the registry
    for site_initr in inventory::iter::<crate::Initializer> {
        // registry.insert(applet.name, applet);
        // 转移到map去
        (site_initr.f)(&mut registry);
    }

    // TODO 应该根据传递的命令参数做选择！
    // 此处根据用户选择 挑选对应的实现
    for (some_k , some_site) in registry {
        println!("😊🐂 some site: {}", some_site.get_name());
    }

    // TODO 应该根据传递的命令参数做选择！
    // let target_site = crate::sites:: SiteRavenel::default();
    let target_site = crate::sites:: SiteLNMuseum::default();

     let mut db_path = String::from(PROGRAM_DB);
     db_path = db_path.replace(PROGRAM_SITE_TPL,target_site.get_name().as_str());
    println!("[🐞db_path:] {db_path} ");

    // Register a type globally
    let registry = TypeRegistry::global();

    if ! registry.is_registered::<AppContext>() {
        register!(AppContext, AppContext {
               db: Config::default()
                .path(&db_path) // ⚠️ 在加了重试逻辑后 这个方法变为可多次调用 就会产生锁的问题！！！😺
                .open()
                .expect("Unable to open root database"),
                site_name: target_site.get_name(),
                // target_site: Box::new(target_site),
                target_site: Arc::new(target_site.clone()),
       }).unwrap();
    }else{
        println!("😔😔 这是重入了");
    }





    // 清理缓存
    if app_args.rm_cache {
        let cache_dir = get_cache_dir() ;
        println!("🐘 remove cache dir : {:?}", cache_dir);

        // Clean up the data!
       let result = cacache::clear(&cache_dir).await;
        if result.is_err() {
            if let Err(err) = result {
                println!("remove dir failed! 😭 :{}" ,err);
            }

        }

        return Ok(()) ;
    }
    // return Ok(()) ; // --------- 调试用 🤔

    let settings = config::Conf::new();

    // Print out our settings
    println!("[debug] 🐞 应用配置 :{settings:?}");

    println!("😄：just for learning ；scrap images from {}",target_site.get_title());

    // println!("{:#}",RO);
    // let tree = ROOT_DB.open_tree("my_tree");

    // 无头配置
    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
     )
        .unwrap();

    let mut cb =  ClientBuilder::native();
    if app_args.headless {
        cb.capabilities(cap);
    }

    let c =
         // ClientBuilder::native()
        // .capabilities(cap) // ⚠️ 无头配置如果看不到情况 可以注释掉此行变为有头 看效果
         cb.connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");




    let search_user = "溥儒";
    let search_user = "八大山人";
    let search_user = "張大千";
    // let search_user = "燕文贵";
    // let search_user = "溥心畬";
    // let search_user = "黄君璧";
    // let search_user = "陈少梅";
    // let search_user = "石涛";
    // let search_user = "黄公望";
    // let search_user = "沈周";
    // let search_user = "林良";
    // let search_user = "陈道复";
    // let search_user = "董源";
    // 😺：宋代名画家 这个网站竟然这么多宋代名家作品
    // let search_user = "王诜";
    // let search_user = "赵国经";
    // // let search_user = "王美芳";
    // // let search_user = "齐白石";
    // let search_user = "丰子恺";
    // let search_user = "徐燕孙";
    // let search_user = "黄均";
    // let search_user = "王时敏";
    // let search_user = "陆俨少"; // 没有
    // let search_user = "陈云彰"; // 陈少梅
    // let search_user = "沈周";
    // let search_user = "八大山人";
    // let search_user = "林风眠";
    // build|make target_url
    // let page = 1;
    // let target_url = format!("https://artso.artron.net/auction/search_auction.php?keyword={}&", search_user) ;
    let target_url = target_site.start_url(search_user) ;
    println!("👨👩: search items for user: {}",search_user);


    // 陈少梅



    c.goto(&target_url).await?;


    loop {
        println!("enter_loop");

        let url = c.current_url().await?;
        let page_num = get_page_num(url.to_string().as_str()) ;
        // println!("{}", url);
        println!("page: {}", page_num);
        // let tmp_path = "/somePath/hi";
        // println!("{}", url.join(tmp_path)?);

        #[allow(deprecated)]
        c.wait_for(move |_| {
            std::thread::sleep(Duration::from_secs(7));
            async move { Ok(true) }
        })
            .await?;


        // -----------------------------执行异步js--------------------------------------------------

        let js_pagination = target_site.get_pagination_js() ;


        //
        // let mut js_result = c.execute_async(JS, vec![]).await?;
        // println!("js result here :{:?}", js_result);
        // // -------------------------------------------------------------------------------
        // 检查是否为数组
        // https://docs.rs/serde_json/latest/serde_json/enum.Value.html
        use serde_json::Value;

        // let mut need_page = true;
        // let mut js_result_items = js_result["items"].as_array().unwrap() ;
        //
        // println!("js result length :{:?}", js_result_items.len());
        // for item in js_result_items {
        //     println!("{}", item.as_str().unwrap());
        // }


        //

        // let mut items_from_js: Vec<&str> = Vec::new();
        let mut items_from_js  = Vec::new();

        // TODO 想加入缓存功能！
        // - https://github.com/foyer-rs/foyer
        // - https://github.com/zkat/cacache-rs?tab=readme-ov-file#example

        // let cache_dir = format!("{}/cache" ,PROGRAM_DB);
        let cache_dir = format!("{}/cache" ,get_prog_db());
        // let cache_key = format!("{}:search4:{}", PROGRAM_NAME,search_user);
        let cache_key = format!("{}:search4:{}", target_site.get_name(),search_user);

        let mut flag_do_loop = true ;

        if app_args.cache {
            let data = cacache::read(&cache_dir, &cache_key).await;
            if data.is_ok(){
                let  cached_value: Vec<String> = crate::utils::my_bincode::deserialize(&data.unwrap()) ;
                println!("🐒 [load data from cache] search4: {} 数据条目 : {}",search_user ,cached_value.len());

                items_from_js.extend(cached_value);

                flag_do_loop = false ;

            }else{
                println!("search4 not found");
            }
        }


        if (flag_do_loop) {
            loop {
                // 循环体代码
                // let mut js_result = c.execute_async(JS_SCROLL_FOR_LOAD_MORE, vec![]).await?;
                // let mut js_result = c.execute_async(JS_PAGINATION, vec![]).await?;
                let mut js_result = c.execute_async(target_site.get_pagination_js(), vec![]).await?;
                println!(" 🈲️[🐞debug:] js result here :{:?}", js_result);

                // TODO 在这里收集当前页面的链接
                let mut js_result_items = js_result["items"].as_array().unwrap();

                println!("js result length :{:?}", js_result_items.len());
                // for item in js_result_items {
                //     println!("{}", item.as_str().unwrap());
                // }

                let my_items: Vec<_> = js_result_items.into_iter().map(|item| item.as_str().unwrap().to_string()).collect();
                items_from_js.extend(my_items);

                let mut js_result_load_more = js_result["load_more"].as_bool().unwrap();

                if js_result_load_more == false {
                    break;
                }

                // sleep_random(&c).await ;
                // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                tokio::time::sleep(Duration::from_secs(10)).await; // 模拟耗时任务
            }
        }
        if items_from_js.len() > 0 {
            // println!("🈲️[debug:] all items from js here :{:?}", items_from_js);
        }

        // 存入缓存
         if app_args.cache && flag_do_loop {
            // Write some data!
          let encoded_items = crate::utils::my_bincode::serialize(&items_from_js);
          let cache_result =  cacache::write(&cache_dir, &cache_key, &encoded_items).await;
            if cache_result.is_ok(){
                println!("🐂 [do cache!] cache result is ok! ");
            }else {
                println!("cache result is err!");
            }
        }

        // ---------------------------------👆是js获取当前页的待处理元素的逻辑 跟👇下面逻辑类似 可以任选一个修改逻辑         -------------------------------------------

        let mut handled_items = BTreeSet::new();

        if items_from_js.len() > 0 {
            // ⚠️注意into_iter 跟 iter 是不同的 🤔

            if app_args.rev_order {
                // TODO 这里不管用的 从无序变有序 跟rev无关
                // handled_items = items_from_js.into_iter().rev().collect();
                handled_items = items_from_js.into_iter().collect();
            }else {
                handled_items = items_from_js.into_iter().collect();
            }

        }else{
            // 🐯用rust来收集当前页面元素
            // 页码
            let mut page_number =0;
            // # 使用XPath表达式选择具有特定类的元素
            // elements = root.xpath('//*[@class="my-class"]')
            // 获取本页需要处理的图片
            // let selector = r#"//div[@class="swiper-slide"]/a"#;
            let selector = r#"//div[starts-with(@class, "SearchResultsGrid_lotCardWrapper__")]//a"#;
            let eles = c.find_all(Locator::XPath(selector)).await?;
            // ⚠️ 注意这里面有重复的a链接 想办法去重
            println!("🤔==== 当前页面有元素 > {:?} ", eles.len());
            // let mut handled_items = HashSet::new();
            let window_1 = c.window().await?;
            // 遍历元素
            for elem in eles {


                let item_path = elem.attr("href").await?.unwrap();
                let item_url = url.join(&item_path);

                let item_url2 = item_url.clone().unwrap();
                let path_segments =item_url2.path_segments().map(|c| c.collect::<Vec<_>>());
                let mut path_segments = path_segments.unwrap();
                // println!("🐶：{:?}",path_segments);

                let target_item_url = item_url.unwrap().to_string() ;

                if handled_items.contains(&target_item_url) {
                    // println!("🐼： 已经处理过啦！");
                    continue
                }else{
                    let tmp_item_url = target_item_url.clone() ;
                    handled_items.insert(tmp_item_url);
                }

            }
        }



        let item_count = handled_items.len() ;
        println!("🐯元素个数:{}",handled_items.len());

        // 😄🐲: 存到有序集合或者db中 然后就可以安全跳过某个区域 避免每次都重新运行 根据上次异常断点位置继续开始
        let mut skip_from_number = app_args.skip;

        // 💾 如果由此选项 则从缓存中加载上一次进度数字
        if app_args.resumption {

            let tree = db_tree();
            // let key = format!("progress_{}", search_user, ); // TODO 多次使用应该提取成为函数
            let key =  if app_args.rev_order{
                let key_factor = format!("_rev_{}", search_user, );
                println!(" reverse order! ");
                build_progress_cache_key(&key_factor)

            }else {
                build_progress_cache_key(search_user)
            };
            match tree.get(key) {
                Ok(Some(value)) => {
                    // 将字节序列转换回整数并打印出来
                    let int_value: usize = usize::from_le_bytes(value.as_ref().try_into().unwrap()); // 使用 little-endian 字节序转换回整数
                    println!("Retrieved integer: {}", int_value);
                    // 🤔看看存入的跟取出的一样不？
                    skip_from_number = int_value -1;
                },
                Ok(None) => println!("No value found for key"),
                Err(e) => println!("Error retrieving integer: {}", e),
            }
        }

        // 使用 for 循环遍历集合
        // for item_url in handled_items {
        // let enu = if app_args.rev_order {
        //     handled_items.into_iter().rev().enumerate()
        // } else {
        //     handled_items.into_iter().enumerate()
        // } ;

        for (idx, item_url) in  handled_items.into_iter().enumerate(){
            println!("🐢🐰🏃‍♀️进度: {}/{} => {}",idx,item_count , item_url);

             if idx < skip_from_number {
                continue ;
             }


            c.new_window(true).await?;
            // let window_2 = c.window().await?;

            let mut all_windows = c.windows().await?;
            // let new_window = all_windows
            //     .into_iter()
            //     .find(|handle| handle != &window_1)
            //     .expect("Should find a differing window handle");
            // let last_window = all_windows.last().unwrap();
            // let window_size = all_windows.len();
            // let last_window = all_windows.get(window_size - 1).unwrap();
            let last_window = all_windows.pop().unwrap();
            c.switch_to_window(last_window).await?;
            // c.goto(String::from(item_url.unwrap()).as_str()).await?;
            c.goto(item_url.as_str()).await?;


            // 点击图片
            // let selector = r#"a.pointer.image-holder"#;
            // 🤔：可以使用js 端技术 https://developer.mozilla.org/zh-CN/docs/Web/API/Document/querySelector
            // TODO: 这里有多余逻辑 懒得改了 用的其他网站逻辑
            // let selector = r#"li.slide.previous"#;
            let selector = r#"body"#;

            let ele_result = c.wait().for_element(Locator::Css(selector)).await;

            if ele_result.is_ok(){
                // 点击一下
                // ele_result.unwrap().click().await?;
                // println!("[debug:]<{:?}", ele_result);
                sleep_random(&c).await ;

                let img_urls = get_image_urls(&c).await;

                let img_info = get_image_info(&c).await;

                let img_url_count = img_urls.len();

                for (idx2 ,img_url) in img_urls.iter().enumerate() {
                    // println!("{:?}",img_info);
                    println!("img_url {:?}",img_url);
                    // let parts = img_url.rsplit('=');
                    // ⚠️ 这里有不通用逻辑 可以针对每个网站不同策略 TODO
                    // let splits: Vec<&str> = img_url.rsplitn(2, '=').collect();
                    // let target_url = splits.first().unwrap() ;
                    // // https://www.iamwawa.cn/urldecode.html 在线解码工具
                    // let decoded_url = urlencoding::decode( target_url).unwrap() ;
                    let decoded_url = img_url ;

                    println!("splits 👍 {:?}",decoded_url);
                    println!("😷: {:?}",sanitize_filename::sanitize(img_info.clone()));

                    // let folder_name = format!("{}_{}",search_user,PROGRAM_NAME);
                    let folder_name = format!("{}_{}",search_user,target_site.get_name());
                    let mut img_save_to_name =sanitize_filename::sanitize(img_info.clone());
                    if img_url_count > 1{
                         img_save_to_name = format!("{}_{}",img_save_to_name,idx2);
                    }else{
                         // img_save_to_name = format!("{}_{}",img_save_to_name,idx2);
                    }


                    download_image(&folder_name,&img_save_to_name,&decoded_url).await ;

                    // 保存下载进度
                    let tree = db_tree();
                    let key = format!("progress_{}", search_user, );
                    // tree.insert(key, sled::IVec::from(&[idx;1]));
                    // 将整数转换为字节序列（使用二进制表示），因为 sled 需要字节序列来存储值
                    // let value_bytes = value.to_le_bytes(); // 使用 little-endian 字节序
                    tree.insert(key, &idx.to_le_bytes());
                    println!("💾 将进度保存起来 {}", idx);

                }

                if img_url_count == 0{
                    println!(" 😺：没有图呀！ ");
                }

                // let selector = r#"//ul[@class="slider"]/li[@class="slide"]//img"#;
                // // class="slider-wrapper axis-horizontal"
                // let selector = r#"//div[@class="slider-wrapper"]/ul/li[@class="slide"]/img"#;
                // // let selector = r#"//li[@class="slide"]/img"#;
                // // let selector = r#"//div[@class="swiper-slide"]//img"#;
                // let eles = c.find_all(Locator::XPath(selector)).await?;
                // println!("🤔==== 相册元素 > {:?} ", eles.len());
                // for ele in eles {
                //     println!("💪 图片路径: {}", ele.attr("src").await?.unwrap());
                // }

                // 注意可能出现gallery的情况 TODO ：
                // ele.click().await?;
                //

                sleep_random(&c).await ;
                println!(" 😄 a single item is done 🤔") ;
            }else{
                println!("no image! 🤔") ;
            }


            // 关闭当前窗口
            c.close_window().await?;


            let mut  all_windows = c.windows().await?;
            all_windows.split_off(1) ; // 这步代码如果不人为干扰也可以不要 😄
            c.switch_to_window(all_windows.pop().unwrap()).await?
        }

        println!("🤔 接下来寻找分页链接");
        // xpath 选择表达式 ;
        let selector = r#"//li[contains(@class, 'pagination-next') and contains(@class, 'disabled')]"#; //  双斜杠，表示不管层级关系
        let selector = r#"//li[contains(@class, 'pagination-next') ]/a"#; //  双斜杠，表示不管层级关系
        // xpath 有大量内置函数 可以协助提取信息如 text()
        // let next_page_button = c.find(Locator::XPath(selector)).await?;

        let button_result = c.wait().for_element(Locator::XPath(selector)).await;

        if button_result.is_ok(){
            let button = button_result.unwrap();

            // // 查看是否含有 disable属性
            // let ele_attr =  button.attr("class").await?.unwrap();
            // println!("[ele_attr:] {}", ele_attr);
            println!("[next:]");
            // je.executeScript("arguments[0].scrollIntoView(true);", element);
            c
                .execute_async(
                    " setTimeout(() =>{arguments[0].scrollIntoView(true);arguments[0].click();arguments[1]( 1);},Math.floor(Math.random() * 100))",
                    // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
                    vec![serde_json::to_value(button.clone())?],
                )
                .await;
        // button.click().await?;
        }else{
            println!("[ 🐮 all  done! ]") ;
            break ;
        }


    }


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

pub fn make_item_url(s: &str) -> String {
    // match s {
    //     "firefox" => "http://localhost:4444",
    //     "chrome" => "http://localhost:9515",
    //     browser => unimplemented!("unsupported browser backend {}", browser),
    // }
    String::new()
}

pub async  fn sleep_random(c:&fantoccini::Client) {
    // println!("<< enter: sleep_random");
    let rslt =  c
        .execute_async(
            " setTimeout(() => arguments[1](arguments[0] + 1),Math.floor(Math.random() * 1000))",
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            vec![1_u32.into()],
        )
        .await ; // 返回值是2 搞测试而已
        // .await.unwrap().as_u64(); // 返回值是2 搞测试而已

    if rslt.is_err() {
        // TODO : 出现下载失败可以保存到某个数据库 然后在重新执行尝试下载
        println!("some error happened :{:?}", rslt.err());

    }else{

        println!("sleep_random ok!") ;
    }

    // println!("[sleep_random->]{:#?}", rslt);
}

pub async  fn get_image_url(c:&fantoccini::Client) -> String {
    println!("<< enter: get_image_url");

    const JS: &'static str = r#"
            // 😄回调函数就叫 callback 可以给你用来返回任何东东哦！
            // ⚠️多参数时 callback回调函数总是出现在尾部

            // const [date, callback] = arguments;
            const [ callback] = arguments;

            // @see https://blog.darkthread.net/blog/js-delayexecute/
            //Parameters:
            //check      : a function to return true/false
            //proc       : a function which won't run until check() return true
            //chkInterval: optional, checking interval, default value is 500ms
            function delayExecute(check, proc, chkInterval) {
                //default interval = 500ms
                var x = chkInterval || 500;
                var hnd = window.setInterval(function () {
                    //if check() return true,
                    //stop timer and execute proc()
                    if (check()) {
                        window.clearInterval(hnd);
                        proc();
                    }
                }, x);
            }

             // var js_ruslts =
             // {
             //   items: getCurrentPageImages(),
             //   need_page: false
             // };
             //  callback( js_ruslts );

             var selectors = ".pointer.image-holder";

             delayExecute(
              function() {

                return document.querySelector(selectors) !== null ;
              },
              function() {
                // alert("Executed!");
                console.log("Executed!");

                const el = document.querySelector(selectors);
                el.addEventListener("click", function(e) {
                  // 获取大图地址 并异步返回给rust

                  setTimeout(function(){

                        const el = document.querySelector("\#zoom-img img");
                        let attribute = el.getAttribute("src");
                        callback(attribute);

                  }
                  , 6000); // 给6秒加载图片 应该差不多了😄
                  // callback(666); // 这里就是凑形式
                });

                el.click();
              }
            );

        "#;

    let rslt =  c
        .execute_async(
            JS,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await.unwrap(); // 返回值是2 搞测试而已
    // rslt.as_str() ;// as_xxx 方法是做转型的

    println!("[get_image_url->]{:#?}", rslt);
    rslt.to_string()
}
pub async  fn get_image_info(c:&fantoccini::Client) -> String {
    println!("<< enter: get_image_info");

    let config = TypeRegistry::global()
        .get_cloned::<AppContext>()
        .unwrap();

    let js = config.target_site.get_image_info_js() ;

    let rslt =  c
        .execute_async(
            js,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await; // 返回值是2 搞测试而已
    // rslt.as_str() ;// as_xxx 方法是做转型的
    if rslt.is_err() {
        println!(" 😺：没有信息！get_image_info ");
        return String::from("");
    }

    println!("[get_image_info->]{:#?}", rslt);
    rslt.unwrap().to_string()
}
// pub async  fn get_image_urls(c:&fantoccini::Client) -> String {
pub async  fn get_image_urls(c:&fantoccini::Client) -> Vec<String> {
// pub async  fn get_image_urls(c:&fantoccini::Client) -> Vec<&'static str> {
    println!("<< enter: get_image_urls");

    // Retrieve and use the registered type
    let config = TypeRegistry::global()
        .get_cloned::<AppContext>()
        .unwrap();

   let js = config.target_site.get_image_urls_js() ;

    let rslt =  c
        .execute_async(
            js,
            // " setTimeout(() => alert('😄'),Math.floor(Math.random() * 3000))",
            // vec![1_u32.into()], // ⚠️ 这里传参与否 或者传的多少都会改变callback的位置
            vec![],
        )
        .await ;
    if rslt.is_err() {
        println!(" 😺：没有图呀！get_image_urls ");
        return Vec::new();
    }
    let rslt = rslt .unwrap(); // 返回值是2 搞测试而已

    println!("[get_image_urls->]{:#?}", rslt);
    let items = rslt.as_array().unwrap() ;

    // println!("[🐯图片总数->]{:#?}", items.len());
    let mut results = vec![] ;
    for item in items {
         //println!("[🐷get_image_url->]{:#?}", item) ;
        // results.push(item.to_string()) ;
        results.push(item.as_str().unwrap().to_string() ) ;
        // results.push(item.as_str().unwrap() ) ;
    }
    // println!(" 🌹 [get_image_urls->]{:#?}", results);
    // // rslt.as_str() ;// as_xxx 方法是做转型的
    // //
    // // println!("[get_image_urls->]{:#?}", rslt);
    // // rslt.to_string()
    // String::from("test")

    return results ;


}

pub async fn download_image(folder_name:&str,img_save_to_name:&str,img_url:&str,){

    use chrono::{DateTime, Utc, Duration};
    let start = Utc::now();

    // 临时测试下
    // let img_url = "\"https://www.polyauctionhk.com/lib/uploads/165598976462b4660473f07/424/424.jpg\"";
    let img_url = img_url.trim_matches('"');
    if img_url.ends_with(".svg"){
        println!("💢 没图片么！");
        return () ;
    }

    let img_save_to_name2 = img_save_to_name.clone();

    let img_save_to_name = format!("{}.jpg",img_save_to_name);
    println!("[debug 🤔]: {folder_name}_{img_save_to_name} from 【{img_url}】") ;
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
        let matedata = std::fs::metadata(save_to_path.clone()).unwrap();
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
            // TODO : 出现下载失败可以保存到某个数据库 然后在重新执行尝试下载
            println!("some error happened :{:?}", rslt.err());

            let tree = db_tree();
            let key = format!("{}:{}",save_to_folder.display(),img_save_to_name2);
            tree.insert(key, img_url);

        }else{

            println!("download ok!") ;
            let duration = Utc::now().signed_duration_since(start);
            println!("[🕙debug 🥱]Time elapsed: {:?} 秒", duration.as_seconds_f32());

            use std::fs;
            use std::path::Path;
            let img_path = &save_to_path.to_str().unwrap().to_string();
            let path = Path::new(&img_path);

            match fs::metadata(path) {
                Ok(metadata) => {
                    let size = metadata.len();
                    println!(" 🏠: 文件大小: {} 字节", format_file_size(size));
                },
                Err(error) => {
                    println!("无法获取文件信息: {}", error);
                }
            }
        }
    }

    // =================================== 下载 >> ======================
}

// 定义一个函数来格式化文件大小
fn format_file_size(bytes: u64) -> String {
    if bytes > 1_000_000_000_000 {
        return format!("{:.2} TB", bytes as f64 / 1_000_000_000_000.0)
    } else if bytes > 1_000_000_000 {
        return format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes > 1_000_000 {
        return format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes > 1_000 {
        return format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        return format!("{} B", bytes);
    }
}

fn db_tree()->sled::Tree{
    // Retrieve and use the registered type
    let config = TypeRegistry::global()
        .get_cloned::<AppContext>()
        .unwrap();
    // ROOT_DB.open_tree("my_tree_space").unwrap()
    config.db.open_tree("my_tree_space").unwrap()
}

#[test]
fn test_get_page_url() {
    let url = "index.asp?page=10";
    let url2 = get_page_url(url,12);
    assert_eq!(url2,"index.asp?page=12")
}