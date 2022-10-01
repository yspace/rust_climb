use tokio::time::sleep;



use web_scraper3::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let conn = save2sqlite::establish_connection().await?;
    save2sqlite::init_db(conn.clone()).await?;

    let rslt_latest_project  = save2sqlite::get_latest_project(conn.clone()).await;
    match rslt_latest_project {
        Ok((id,project)) => {
            println!("latest: {:?}", project);

            // 下载
            let save_to_folder = "downloads" ;
            let save_to_name = base64::encode(
                format!("{}_{}",id, project.title)
            );
            let download_url = format!("https://theme.npm.edu.tw/opendata/{}",&project.download_url);
            let download_rslt = download::download_file(download_url.as_str(),save_to_folder,save_to_name.as_str()).await;
            match download_rslt{
                Ok(path) => { 
                    println!("save to {}",path);
                }
                Err(err) => println!("{}",err) ,
            }

            // 更新任务进度
           let _r = save2sqlite::project_mark_as_downloaded(conn.clone(), id, project).await?;

           // 继续下个轮回
        },
        Err(err) => { 
            println!("done!")
        },
    }

    Ok(())
}