
use std::{env, fs};
use std::path::{Path, PathBuf};


 

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );
    let path = current_dir.join("downloads") ;

    for entry in fs::read_dir(path)? {
      
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        // if last_modified < 24 * 3600 && metadata.is_file() {
        //     println!(
        //         "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
        //         last_modified,
        //         metadata.permissions().readonly(),
        //         metadata.len(),
        //         path.file_name().ok_or("No filename")?
        //     );
        // }else{
            println!("no changes happend !") ;
            println!("{:?}", path.file_name().ok_or("no filename")) ;
            println!(" ext: {:?}",path.extension().unwrap());

            if metadata.len() > 0 && path.extension().unwrap().eq("zip"){
                println!("enter zip section !");
                zip_file_info(path.as_path()) ;
            }
            // println!("{:?}", path) ;
        // }
        
    }

    Ok(())
}


fn zip_file_info(path: &Path) {
    use std::io::BufReader;

    let fname = path ; // std::path::Path::new(path);
    let file = fs::File::open(&fname).unwrap();
    let reader = BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => {
                println!("Entry {} has a suspicious path", file.name());
                continue;
            }
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("Entry {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!(
                "Entry {} is a directory with name \"{}\"",
                i,
                outpath.display()
            );
        } else {
            println!(
                "Entry {} is a file with name \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
        }
    }
}