use tokio::time::sleep;

use web_scraper3::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     test_download::main().await?;
     return Ok(());
    // // ----------
    let conn = save2sqlite::establish_connection().await?;
    save2sqlite::init_db(conn.clone()).await?;

    let rslt_latest_project = save2sqlite::get_latest_project(conn.clone()).await;
    match rslt_latest_project {
        Ok((id, project)) => {
            println!("latest: {:?}", project);

            // 下载
            let save_to_folder = "downloads";
            let save_to_name = base64::encode(format!("{}_{}", id, project.title));
            let download_url = format!(
                "https://theme.npm.edu.tw/opendata/{}",
                &project.download_url
            );

            let download_rslt = download::download_file(
                download_url.as_str(),
                save_to_folder,
                save_to_name.as_str(),
            )
            .await;
            match download_rslt {
                Ok(path) => {
                    println!("save to {}", path);
                }
                Err(err) => println!("{}", err),
            }

            // 更新任务进度
            let _r = save2sqlite::project_mark_as_downloaded(conn.clone(), id, project).await?;

            // 继续下个轮回
        }
        Err(err) => {
            println!("done!")
        }
    }

    Ok(())
}

mod test_download {
    use fantoccini::{ClientBuilder, Locator,Client};
    use fantoccini::{wd::Capabilities};
    use std::time::Duration;
    use tokio::time::sleep;

    fn init_capabilities() {
        // 不需要这个类 可以用fantoccini::wd 包下的

        //use webdriver::capabilities::Capabilities;
        use fantoccini::{wd::Capabilities};

        use serde_json::json ;
        let mut cap = Capabilities::new();
        // let arg = serde_json::from_str("{\"args\": [\"-headless\"]}").unwrap();
        let arg = json!({"args": ["-headless"]});
        
        cap.insert("moz:firefoxOptions".to_string(), arg);

        // Client::with_capabilities_and_connector(webdriver, cap, connector)
        //let c = Client::with_capabilities("http://localhost:4444", cap);
        let c =  ClientBuilder::native().capabilities( cap);
    }

    pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::json ;
        use std::path::{PathBuf, Path};
        // @see https://github.com/jonhoo/fantoccini/issues/44
        // 评论区有人给出了可用的capabilities 地址
        let mut cap = Capabilities::new();
        // let arg = serde_json::from_str("{\"args\": [\"-headless\"]}").unwrap();
        // let arg = json!({"args": ["-headless"]});
        // cap.insert("moz:firefoxOptions".to_string(), arg);

        let mut download_path = std::env::current_dir().unwrap(); //push("downloads");
        download_path.push("downloads");
        //download_path.as_path();
        // let arg = json!({"download.default_directory":"/directory/path"});
        // let arg = json!({"download.default_directory":download_path.as_path().to_str()});
        let arg = json!({"prefs": {"download.default_directory":download_path.as_path().to_str()}});
        //cap.insert("prefs".to_string(), arg);
        cap.insert("goog:chromeOptions".to_string(), arg);


        // Connect to webdriver instance that is listening on port 4444
        let client = ClientBuilder::native()
        .capabilities(cap)
            .connect("http://localhost:4444")
            .await?;

        let mut url = format!(
            "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
            1
        );

        // Go to the website.
        client.goto(&url).await?;

        // 等候某元素被加载.
        let el_next_page = client
            .wait()
            .for_element(Locator::Css(r#"a.next-page"#))
            .await?;
        let nex_page_url = el_next_page.attr("href").await?.unwrap();
        println!("next page url{}", nex_page_url);

        let index = 1;
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

        client.wait().for_element(Locator::Css("body")).await?;

        let download_btn = client
            .wait()
            //r#"a[href="/learn/get-started"]"#,
            .for_element(Locator::XPath(r#"//a[@class="download-btn"]"#))
            .await?;
        download_btn.click().await?;

        sleep(Duration::from_millis(30000)).await;

        // 下载文件中 别关闭呀
        //  client.close().await?;

        Ok(())
    }
}
