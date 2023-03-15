// map_error 方法在railway oriented programing（rop）中有很重要的角色 不可忽视

#[test]
fn test_map_error() {
    // 把u32错误 转化为String类型的错误
    fn stringify(x: u32) -> String {
        format!("error code: {}", x)
    }
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));
    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}
