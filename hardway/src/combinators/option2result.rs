use anyhow::Ok;


#[test]
fn test_converting(){
    let opt = Some(2);
    let result = opt.ok_or(0);


    let rslt = Ok(10);
    let opt = rslt.ok() ;
    let r = opt.ok_or(anyhow::Error::msg(format!("error {}","blablabla")));
}