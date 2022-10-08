use std::fs;
use std::time::Duration;
use tokio::time::sleep;

use web_scraper3::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = save2sqlite::establish_connection().await?;
    save2sqlite::init_db(conn.clone()).await?;

    loop {
        let rslt_latest_project = save2sqlite::get_latest_project_rev(conn.clone()).await;
        if rslt_latest_project.is_err() {
            // 没东西了
            break;
        }

        // 完成此次下载任务
        match rslt_latest_project {
            Ok((id, project)) => {
                println!("latest: {:?}", project);

                // 下载
                let save_to_folder = "downloads";
                let save_to_name =
                    //  = base64::encode(
                    //     format!("dwonloads/{}_{}.zip",id, project.title)
                    // );
                        format!("{}_{}.zip",id, project.title);

                let mut save_to_path = std::path::Path::new("downloads").join(save_to_name);

                let download_url = format!(
                    "https://theme.npm.edu.tw/opendata/{}",
                    &project.download_url
                );
                // let download_rslt = download::download_file(download_url.as_str(),save_to_folder,save_to_name.as_str()).await;
                // match download_rslt{
                //     Ok(path) => {
                //         println!("save to {}",path);
                //     }
                //     Err(err) => println!("{}",err) ,
                // }

                let mut try_times = 0;

                loop {
                    if save_to_path.as_path().exists() {
                        println!("path existed");
                        // 看下文件大小是不是0 上次因为某种原因为下载完成的文件大小会是0的
                        let matedata = fs::metadata(save_to_path.clone())?;
                        if matedata.len() == 0 {
                            // 删掉它
                            fs::remove_file(save_to_path.clone()).expect("could not remove file");
                            println!("file is removed");
                        } else {
                            // 文件存在且大小不为0 证明已经下载成功了
                        }
                    }

                    let rslt = download::fetch_url(
                        download_url.clone(),
                        save_to_path.to_str().unwrap().to_string(),
                    )
                    .await;
                    match rslt {
                        Ok(rslt) => {
                            println!("download ok!");
                            // 跳出循环
                            break;
                        }
                        Err(err) => {
                            println!("download err: {}", err);
                            // 错误的话重试几次 3吧 毕竟事不过三   然后睡会
                            try_times = try_times + 1;
                            if try_times > 3 {
                                // 试了三次还下载不成功 就退出 继续下个任务 并更新状态标记为某个值

                                save2sqlite::project_update_status(conn.clone(), id, 3).await?;

                                break;
                            }
                        }
                    }
                }

                let matedata = fs::metadata(save_to_path.clone())?;
                if matedata.len() == 0 {
                    println!("empty file！") ;
                    save2sqlite::project_update_status(conn.clone(), id, 3).await?;
                } else {
                    println!("successful download!") ;
                    // ### 更新任务进度
                    let _r =
                        save2sqlite::project_mark_as_downloaded(conn.clone(), id, project).await?;
                }

                sleep(Duration::from_millis(300)).await;
                // 继续下个轮回
            }

            Err(err) => {
                println!("done!");
            }
        }
    }

    println!("all done!");

    Ok(())
}
