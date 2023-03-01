fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    // bool 类型来作为过滤条件
    let _positive_numbers: Vec<i32> = v.filter(|x: &i32| x.is_positive()).collect();
}

fn inspect() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();
}

fn map() {
    let v = vec!["Hello", "World", "!"].into_iter();
    let w: Vec<String> = v.map(String::from).collect();
}

// 效果类 链式调用filter 然后map ，用Option类型作为中间帮手
fn filter_map() {
    let v = vec!["Hello", "World", "!"].into_iter();
    let w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(w, vec!["Hello".to_string(), "World".to_string()]);
}

// chain 合并两个迭代器
fn chain() {
    let x = vec![1, 2, 3, 4, 5].into_iter();
    let y = vec![6, 7, 8, 9, 10].into_iter();
    let z: Vec<u64> = x.chain(y).collect();
    assert_eq!(z.len(), 10);
}

// 展平 集合的集合
fn flatten() {
    let x = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]].into_iter();
    let z: Vec<u64> = x.flatten().collect();
    assert_eq!(z.len(), 10);
}

// Composing combinators 组合使用
// It has the advantage of working with immutable data and thus reduces the probability of bugs.

#[test]
fn combinators() {
    let a = vec![
        "1",
        "2",
        "-1",
        "4",
        "-4",
        "100",
        "invalid",
        "Not a number",
        "",
    ];
    let _only_positive_numbers: Vec<i64> = a
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .filter(|x| x > &0)
        .collect();
}
