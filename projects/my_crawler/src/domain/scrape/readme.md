
~~~rust

#[derive(Debug)]
pub struct WorkPosition {
    pub page_url: String,
    pub item_index: i32,
    pub item_xpath: String,
}
~~~
scrap_item{
    id
    page_url
    content_hash
    extra_data // some encoded data ; can express any things
    pub create_date: Option<DateTime>, // created_at
}