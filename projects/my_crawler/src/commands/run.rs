use std::sync::Arc;
use std::time::Duration;

use seahorse::error::FlagError;
use seahorse::{Command, Context, Flag, FlagType};

use crate::crawler::Crawler;
use crate::error::Error;
use crate::settings::Settings;
use crate::{spiders, CONFIG};

pub fn build_command(config: Settings) -> Command {
    Command::new("run")
        .description("Run a spider")
        .alias("r")
        .usage("cli run")
        .flag(Flag::new("spider", FlagType::String).description("cli run  --spider"))
        .action(|c: &Context| {
            println!("Hello from run cmd, {:?}", c.args);
            if let Some(app_data) = c.extensions_mut().get::<Settings>() {
                println!("app-data: {:#?}", app_data);
            }

            match c.string_flag("spider") {
                Ok(spider_name) => {
                    let spider_name = spider_name.as_str();
                    println!("spider name is :{}", spider_name);

                    //   let future = async {

                    //   };

                    use tokio::runtime::Runtime;
                    let rt = Runtime::new().unwrap();
                    let r = rt.block_on(async {
                        println!("in async block!");
                        _run(spider_name).await;
                    });

                    // let rt = tokio::runtime::Builder::new_current_thread()
                    //     .enable_all()
                    //     .build()
                    //     .unwrap();

                    // let res = rt.block_on(async { something_async(&args).await });
                }
                Err(e) => match e {
                    FlagError::TypeError => println!("spider flag type error"),
                    FlagError::ValueTypeError => println!("value type error"),
                    FlagError::Undefined => println!("undefined spider flag"),
                    FlagError::ArgumentError => println!("spider flag argument error"),
                    FlagError::NotFound => println!("not found spider flag"),
                },
            }
        })
}

use rbatis::RBatis;
// use rbdc_sqlite::driver::SqliteDriver;
pub const PG_URL: &'static str = "postgresql://postgres:123456@abyssii:35432/ac";
pub const MYSQL_URL: &'static str = "mysql://yiqing:yiqing@127.0.0.1:3306/test";
struct RBatisConnection {
    // postgres: RBatis,
    mysql: RBatis,
}

async fn _run(spider_name: &str) -> Result<(), anyhow::Error> {
    let crawler = Crawler::new(Duration::from_millis(200), 1, 500);
    match spider_name {
        // "cvedetails" => {
        //     let spider = Arc::new(spiders::cvedetails::CveDetailsSpider::new());
        //     crawler.run(spider).await;
        // }
        "github" => {
            let spider = Arc::new(spiders::github::GitHubSpider::new());

            crawler.run(spider).await;
        }
        "quotes" => {
            let spider = spiders::quotes::QuotesSpider::new().await?;
            let spider = Arc::new(spider);
            crawler.run(spider).await;
        }
        "shanghai" => {
            print!("--- shanghaimuseum ---");

            use crate::service::CONTEXT;
            use crate::domain::table;

            //database
            CONTEXT.init_database().await;
            table::sync_tables(&CONTEXT.rb).await;
            table::sync_tables_data(&CONTEXT.rb).await;


            let spider = spiders::shanghai_mus::ShanghaiMusSpider::new().await?;
            let spider = Arc::new(spider);
            crawler.run(spider).await;
        },
        "shanghai2" => {
            print!("--- shanghaimuseum2 ---");

            use crate::service::CONTEXT;
            use crate::domain::table;

            //database
            // CONTEXT.init_database().await;
            // table::sync_tables(&CONTEXT.rb).await;
            // table::sync_tables_data(&CONTEXT.rb).await;


            let spider = spiders::shanghai_mus2::ShanghaiMusSpider::new().await?;
            let spider = Arc::new(spider);
            crawler.run(spider).await;
        }
        _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
    };
    Ok(())
}
