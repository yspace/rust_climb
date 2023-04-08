#[test]
fn test_once() {
    use std::iter;

    // one is the loneliest number
    let mut one = iter::once(1);

    assert_eq!(Some(1), one.next());

    // just one, that's all we get
    assert_eq!(None, one.next());
}

#[test]
fn test_chain_once() {
    use std::fs;
    use std::iter;
    use std::path::PathBuf;

    // let dirs = fs::read_dir(".foo").unwrap();
    let dirs = fs::read_dir(".").unwrap();

    // we need to convert from an iterator of DirEntry-s to an iterator of
    // PathBufs, so we use map
    let dirs = dirs.map(|file| file.unwrap().path());

    // now, our iterator just for our config file
    let config = iter::once(PathBuf::from(".foorc"));

    // chain the two iterators together into one big iterator
    let files = dirs.chain(config);

    // this will give us all of the files in .foo as well as .foorc
    for f in files {
        println!("{:?}", f);
    }
}
