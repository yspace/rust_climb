// use scraper ;

mod utils;

fn main0() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let url = "https://www.baidu.com";
    // let url = format!("https://www.ssa.gov/oact/NOTES/as120/LifeTables_Tbl_7_{}.html", year);

    let url = format!(
        "https://theme.npm.edu.tw/opendata/DigitImageSets.aspx?Key=^^11&pageNo={}",
        82
    );

    let body = utils::do_throttled_request(&url)?;

    println!("content: {}", body);

    // jquery4crab::query_HTML_markup();
    jquery4crab::query(body.as_str());

    Ok(())
}

fn main() {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}

mod jquery4crab {
    use crabquery::Document;

    pub fn query_HTML_markup() {
        let doc = Document::from(
            "<div class='container'>
       <a class='link button' id='linkmain'>
         <span>text hi there</span>
       </a>
     </div>",
        );

        let sel = doc.select("div.container > a.button.link[id=\"linkmain\"]");
        let el = sel.first().unwrap();

        assert_eq!(el.attr("id").unwrap(), "linkmain");

        let sel = doc.select("div > a > span");
        let el = sel.first().unwrap();

        assert_eq!(el.text().unwrap(), "text hi there");
    }

    pub fn query(doc_text: &str) {
        let doc = Document::from(
            doc_text 
        );

        // let sel = doc.select("div.container > a.button.link[id=\"linkmain\"]");
        // let el = sel.first().unwrap();

        // assert_eq!(el.attr("id").unwrap(), "linkmain");

        // let sel = doc.select("div > a > span");
        // let el = sel.first().unwrap();
        let sel = doc.select("li.list > a");

        for el in sel {
            // let el = sel.first().unwrap();

            let attr_href = el.attr("href").unwrap();
            println!("{:?}",attr_href);
        }
       

    }
}
