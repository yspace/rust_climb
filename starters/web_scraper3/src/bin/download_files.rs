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
            let save_to_name =
            //  = base64::encode(
            //     format!("dwonloads/{}_{}.zip",id, project.title)
            // );
                format!("{}_{}.zip",id, project.title);

            let mut save_to_path = std::path::Path::new("downloads").join(save_to_name);

            
            let download_url = format!("https://theme.npm.edu.tw/opendata/{}",&project.download_url);
            // let download_rslt = download::download_file(download_url.as_str(),save_to_folder,save_to_name.as_str()).await;
            // match download_rslt{
            //     Ok(path) => { 
            //         println!("save to {}",path);
            //     }
            //     Err(err) => println!("{}",err) ,
            // }
            let rslt = download::fetch_url(download_url,   save_to_path.to_str().unwrap().to_string()).await ;
            match rslt {
                Ok(rslt) => println!("download ok!" ),
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