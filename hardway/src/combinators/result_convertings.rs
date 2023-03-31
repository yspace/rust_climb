use anyhow::Ok;

#[test]
fn test_converting() {
    let result = anyhow::Ok("some value".to_owned());
    let option_ok = result.ok();
    assert_eq!(option_ok, Some("some value".to_owned()));

    let result = anyhow::Ok(1);
    let option_err = result.err();

    // let result = anyhow::Result::Err(anyhow::Error::from(0));
    let result = Ok(2);
    // 一个result 可以通过map_err 转化为另一种错误 当然仍可以是自身
    let result2 = result.map_err(|x|{
        // anyhow::Error::new(x)
        // anyhow::Error::from(x)
        // anyhow::Error::
        format!("error code: {x}") // 字符串也可以作为Result 枚举中的E类型！
    });

}
