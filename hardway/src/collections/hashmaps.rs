pub fn run() {
    crud();
    entry();
}

fn crud() {
    use std::collections::HashMap;

    // 声明
    let mut come_from = HashMap::new();

    // 插入
    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S");
    come_from.insert("Mike", "HuoGuo");

    // 查找key
    if !come_from.contains_key(&"elton") {
        println!(
            "Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归",
            come_from.len()
        );
    }

    come_from.remove("Mike");
    println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

    // get 判断元素是否存在
    let who = ["MoGu", "Marisa"];
    for p in &who {
        match come_from.get(p) {
            Some(loc) => println!("{}来自 {}", p, loc),
            None => println!("{} 也无家可归", p),
        }
    }

    // 遍历输出
    println!("那么， 所有人呢?");
    for (name, loc) in &come_from {
        println!("{} 来自: {}", name, loc);
    }
}

fn entry() {
    // 这个api 用来就地更新
    use std::collections::HashMap;

    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        // entry 出来的Entry类型（这个类型持有原有HashMap的引用）即可对原数据进行修改
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);
}
