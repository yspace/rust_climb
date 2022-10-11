use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
   _image::main() ; return Ok(());
   
    // ==
    _infer::main();
    return Ok(());
    // ==
    _args::main();
    return Ok(());
    // ==
    _soland::main().unwrap();
    return Ok(());
    // ==

    match _notify2::main() {
        Ok(()) => {
            println!("done!");
        }
        Err(err) => println!("{}", err),
    }
    return Ok(());
    // ==
    _notify::main();
    return Ok(());
    // ==
    usecase_error::run().unwrap();
    // play mp3 file
    audio::main();
    return Ok(());
    // ==
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );
    let path = current_dir.join("downloads");

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
        println!("no changes happend !");
        println!("{:?}", path.file_name().ok_or("no filename"));
        println!(" ext: {:?}", path.extension().unwrap());

        if metadata.len() > 0 && path.extension().unwrap().eq("zip") {
            println!("enter zip section !");
            zip_file_info(path.as_path());
        }
        // println!("{:?}", path) ;
        // }
    }

    Ok(())
}

fn zip_file_info(path: &Path) {
    use std::io::BufReader;

    let fname = path; // std::path::Path::new(path);
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

mod audio {
    // https://github.com/Kingtous/RustPlayer  这个程序是一个国人写的播放器例子
    use std::io::BufReader;

    pub fn main() {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = std::fs::File::open("assets/music.mp3").unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        sink.sleep_until_end();
    }
}

mod usecase_error {

    fn main() {
        match run() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
    pub fn run(/* some arguments goes here */) -> Result<(), anyhow::Error> {
        println!("running ...");

        Ok(())
    }
}

mod _notify {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::path::Path;

    pub fn main() {
        let path = std::env::args()
            .nth(1)
            .expect("Argument 1 needs to be a path");
        println!("watching {}", path);
        if let Err(e) = watch(path) {
            println!("error: {:?}", e)
        }
    }

    fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => println!("changed: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }
}

mod _notify2 {
    use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
    use std::path::Path;

    pub fn main() -> Result<()> {
        // Automatically select the best implementation for your platform.
        let mut watcher = notify::recommended_watcher(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        })?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

        Ok(())
    }

    fn main2() -> Result<()> {
        fn event_fn(res: Result<notify::Event>) {
            match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        let mut watcher1 = notify::recommended_watcher(event_fn)?;
        // we will just use the same watcher kind again here
        let mut watcher2 = notify::recommended_watcher(event_fn)?;

        Ok(())
    }
}

mod _soland {
    use soloud::*;

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        // fltk 作者的库  一堆好玩的东西 😄！
        let mut sl = Soloud::default()?;

        let mut wav = audio::Wav::default();

        wav.load(&std::path::Path::new("assets/music.mp3"))?;

        sl.play(&wav); // calls to play are non-blocking, so we put the thread to sleep
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("playing is over!");
        Ok(())
    }
}

mod _args {
    pub fn main() {
        let strings: Vec<String> = std::env::args().collect();

        println!("args {:?}", strings);
        println!("len: {:?}", strings.len());
        // std::env::args().  这个args() 返回支持很多方法 是一个迭代器

        if strings.len() == 1 {
            println!("this first argument is {}", strings[0]);
        } else if strings.len() == 2 {
            let arg = &strings[1];
            println!("this second argument is {}", arg);
        } else if strings.len() == 3 {
            println!("decision by len of the arguments");
            //
            let low = strings[1].parse::<i32>().unwrap();
            let high = strings[2].parse::<i32>().unwrap();

            println!("{low} - {high}");
        } else {
            println!("最多支持两个额外参数！");
        }
    }
}

mod _infer {
    use infer;
    pub fn main() {
        let kind = infer::get_from_path("assets/music.mp3")
            .expect("file read successfully")
            .expect("file type is known");

        println!("mime: {}", kind.mime_type());
        println!("extension: {}", kind.extension());
    }
}

mod _image {
    // @see https://lib.rs/crates/image
    use image::GenericImageView;

    pub fn main() {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let img = image::open("assets/my_image.tif").unwrap();

        // The dimensions method returns the images width and height.
        println!("dimensions {:?}", img.dimensions());

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        img.save("assets/test.png").unwrap();
    }
}
