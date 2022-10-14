use fantoccini::{ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

use visdom::types::BoxDynError;
use visdom::Vis;

pub mod download;
pub mod save2sqlite;
pub mod utils;

// [tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //=========== 测试
    //  get_download_item_info().await;
    //  return Ok(());
    //=========== ...

    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    let conn = save2sqlite::establish_connection().await?;
    save2sqlite::init_db(conn.clone()).await?;

    // NOTE: key换一个就可以下载不同类别下的东西了
    let mut url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
        1
    );
    let mut url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^99&pageNo={}",
        // "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^15&pageNo={}",
        // "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^14&pageNo={}",
        // "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^13&pageNo={}",
        // "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^12&pageNo={}",
        // "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
        1
    );

    let mut latest_work_position: Option<save2sqlite::WorkPosition> = None;
    let mut last_run_index = 0;
    // 获取上次的进度位置 如果有的话

    let work_position = save2sqlite::get_latest_work_position(conn.clone()).await;
    match work_position {
        Ok(work_position) => {
            // 上次曾经运行过！
            println!(
                "continue from the latest work position: {:?}",
                work_position
            );
            let page_url = &work_position.page_url;
            url = page_url.as_str().to_string();
            last_run_index = work_position.item_index;

            latest_work_position = Some(work_position);
            println!("current url: {:?}", url);
        }
        Err(err) => {
            // 没查到上次最后一条进度记录 那么啥都不干
            println!("this is your first time to run the program");
        }
    }
    // return Ok(());

    // Go to the website.
    client.goto(&url).await?;
    loop {
        // 等候某元素被加载.
        let el_next_page = client
            .wait()
            .for_element(Locator::Css(r#"a.next-page"#))
            .await?;
        let nex_page_url = el_next_page.attr("href").await?.unwrap();
        println!("next page url{}", nex_page_url);

        let el_items = client
            .wait()
            .for_element(Locator::Css("ul.painting-list"))
            .await?;
        println!("分页加载了！");

        let el_items_html = el_items.html(false).await?;

        //  println!("html: {}", el_items_html);

        // 在rust端处理元素 析取数据
        // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
        // load html
        let root = Vis::load(el_items_html).unwrap();
        let lis = root.find("a");

        // TODO: 这里判断是否是最后一页的条件有问题！当下一页链接跟当前页链接一样时也是满足最后一页的
        if lis.length() > 0 {
            let mut index = 0;

            for li in lis {
                index = index + 1;
                // 如果存在上次运行的索引位置 那么跳过小于它的
                if index < last_run_index {
                    continue;
                }
                /*
                let lis_text = li.html();
                println!("{}", lis_text);
                */
                let href = li.get_attribute("href").unwrap();
                let href_url = href.to_string();
                println!("{}", href_url);
                // 存db啦
                // 获取每个图片的下载链接 这比较诡异 必须通过浏览器才行
                let page_url = client.current_url().await?;
                // println!("{}", page_url);

                // 试验性
                let selector = format!("//ul[@class=\"painting-list\"]/li[{}]/a", index);
                // println!("{selector}") ;
                // Click the img.
                let button = client
                    .wait()
                    //r#"a[href="/learn/get-started"]"#,
                    .for_element(Locator::XPath(selector.as_str()))
                    .await?;
                button.click().await?;
                // 等下 不然一会就退回了

                client.wait().for_element(Locator::Css("body")).await?;
                // sleep(Duration::from_millis(6000)).await;
                //sleep(Duration::from_millis(300)).await;
                // 爬下载链接呀

                // 当前进度 page>item_index
                // println!("current item index: {}", index);
                let work_position = save2sqlite::WorkPosition {
                    page_url: page_url.to_string(),
                    item_index: index,
                    item_xpath: selector,
                };
                println!(">> current position: {:?}", work_position);
                save2sqlite::add_work_position_task(conn.clone(), work_position)
                    .await
                    .unwrap();
                // ----------< download-page ---
                let ele_download_content =
                    client.wait().for_element(Locator::Css("section")).await?;

                let html = ele_download_content.html(false).await?;

                // 在rust端处理元素 析取数据
                // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
                // load html
                let root = Vis::load(html).unwrap();
                let links = root.find("a.download-btn");
                let href = links.attr("href").unwrap().to_string();

                let dom_detail = root.find("div.project-detail");
                let ele_title = dom_detail.find("h3");
                let title = ele_title.text();
                let detail = dom_detail.html();

                // println!("href:{} title:{} detail:{}", href, title,detail);

                save2sqlite::add_project_task(
                    conn.clone(),
                    save2sqlite::Project {
                        title: title.to_string(),
                        detail: detail,
                        download_url: href,
                        status: 0,
                    },
                )
                .await
                .unwrap();

                // ---------- download-page />

                // 看看 跳没
                client.back().await?;
            }
            println!("当前页处理完成！");
            // 最后一页的链接是 javascript:;
            if nex_page_url.starts_with("javascript") {
                // 最后一页了
                println!("last page! done!");
                break;
            }
            // url = nex_page_url ;
            // NOTE: 可能会出现一直点击下一页 但总在原地循环现象！
            println!(" 模拟用户点击链接! ");
            el_next_page.click().await?; // 跳转到下一页
                                         // FIXME: 最后一页时有bug 哦 最好还是直接获取url那种判断页面是否有内容的好 这个网站最后一页href 是：javascript:;
        } else {
            // 最后一页了
            println!("last page! done!");
            break;
        }
    }

    println!("要睡觉了！");
    // Let the user marvel at what we achieved.
    // sleep(Duration::from_millis(6000)).await;
    // Then close the browser window.
    client.close().await?;

    Ok(())
}

async fn v0(client: fantoccini::Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
        1
    );

    loop {
        // Go to the website.
        client.goto(&url).await?;

        // 等候某元素被加载.
        let el_next_page = client
            .wait()
            .for_element(Locator::Css(r#"a.next-page"#))
            .await?;
        let nex_page_url = el_next_page.attr("href").await?.unwrap();
        println!("next page url{}", nex_page_url);

        let el_items = client
            .wait()
            .for_element(Locator::Css("ul.painting-list"))
            .await?;
        println!("分页加载了！");

        let el_items_html = el_items.html(false).await?;

        //  println!("html: {}", el_items_html);

        // 在rust端处理元素 析取数据
        // 文档： https://github.com/fefit/visdom/wiki/%E4%B8%AD%E6%96%87API%E6%96%87%E6%A1%A3
        // load html
        let root = Vis::load(el_items_html).unwrap();
        let lis = root.find("a");

        if lis.length() > 0 {
            for li in lis {
                /*
                let lis_text = li.html();
                println!("{}", lis_text);
                */
                let href = li.get_attribute("href").unwrap();
                let href_url = href.to_string();
                println!("{}", href_url);
            }
            println!("当前页处理完成！");
            url = nex_page_url;
        } else {
            // 最后一页了
            println!("last page! done!");
            break;
        }
    }

    Ok(())
}

struct ItemInfo {}

pub async fn get_download_item_info() -> Result<(), Box<dyn std::error::Error>> /* -> Result<ItemInfo, String> */
{
    let mut url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?sNo={}&Key=^^11&pageNo=157",
        04034362
    );
    println!("url:{}", url);
    let res_text = reqwest::get(url)
        .await
        .map_err(|_| "网络错误")?
        .text()
        .await
        .map_err(|_| "网络错误")?;

    // println!("page content: {}", res_text);
    let root = Vis::load(res_text).unwrap();
    let lis = root.find(".project-detail");

    // let detail = lis.get(0).unwrap() ;
    // println!("detail: {}", detail.html());
    // println!("{}",root.html());

    Ok(())
}
