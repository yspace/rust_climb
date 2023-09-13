/**
 * Always use Option for values that can be absent;
 * never fall back to using sentinel values (-1, nullptr, …) to try to express the same concept in-band.
 * 
 * 缺失值的表示：
 * - Option
 * - 哨兵
 * - 集合类型 的缺失 可以是空集合 Vec<SomeThing> 或者 Option<SomeCollection<...>>
 * - 字符串缺失： Option<String> 或者 ""    空值 前者语义更明确些
 */

#[derive(Debug)]
struct Person{
    name: String,
    id_number: Option<String>,
    // 可以是流浪汉 或者一堆房产的人
    // NOTE: Option<Vec<String>> 或许对于sql null字段比较好映射
    home_addresses: Vec<String> // 对于集合类 空集合可以表示None

}

#[test]
fn test_person(){
    let person = Person{
        name:"qing".into(),
        id_number:None,
        home_addresses: vec![],
    };

    println!("{:?}", person);
    assert_eq!(person.id_number, None);
}