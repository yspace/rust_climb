#[derive(Debug)]
struct Location{
    x: i32,
    y: i32,
}

// 类型实现 完备考虑 常常同时针对T &T &mut T 。或者如这里的就只针对T
impl IntoIterator for Location{
    type Item = i32;

    // 该类型必须是一个结构体 不能是一个trait哦！
    // 但是dyn <Some_Trait> 没有试过是否可行
    type IntoIter = std::array::IntoIter<Self::Item,2>;

    // 这里拿走了所有权 做了move了！
    fn into_iter(self) -> Self::IntoIter {
        // todo!()
        // [self.x, self.y].into_iter()
        std::array::IntoIter::new([self.x, self.y])
    }
}
impl IntoIterator for &Location{
    type Item = i32;
 
    type IntoIter = std::array::IntoIter<Self::Item,2>;

    // 这里self是&Location 
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.x, self.y])
    }
}

fn collect_as_strings<T>(collection: T) -> Vec<String> 
where T: IntoIterator,
T::Item: std::fmt::Debug
{
    collection.into_iter()
    .map(|item| format!("{:?}", item))
    .collect()
}

pub fn run() {

    let location = Location{
        x:1, 
        y: 2 ,
    };

    for loc in &location{
        println!("{}", loc) ;
    }
    // dbg!(location) ; // 

    for loc in location{
        println!("{}", loc) ;
    }
}