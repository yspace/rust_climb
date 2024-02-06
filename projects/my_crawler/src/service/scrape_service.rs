// 此类仿照rbatis 推荐的代码结构来做的 还可以放到 //domain/scrape/service 里

use crate::service::ApplicationConfig;

pub struct ScrapeService {}

impl ScrapeService {

    pub fn new(cfg: &ApplicationConfig) -> Self /*Result<Self>*/ {

        // Ok(Self{})
        Self{}
    }

}