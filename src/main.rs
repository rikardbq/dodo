use chrono::Local;
use std::env;
use std::fs;
use std::path::Path;

use dodo::{get_file_content, parse_flags};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args length = {}", args.len());
    if args.len() >= 2 {
        let date = Local::now().format("%Y%m%d").to_string();
        let path_string = format!("dodos/{date}");
        let done_path_string = format!("{path_string}/done");
        match args[1].as_str() {
            "new" => {
                let flags = parse_flags(args.clone());
                let title = flags.title.expect("Task must have at least a title!");
                let path = Path::new(&done_path_string);
                if !path.is_dir() {
                    fs::create_dir_all(path).expect("Folder for task could not be created!");
                }
                fs::write(
                    format!("{path_string}/{}", title),
                    format!(
                        "title={}\r\ndesc={}\r\nkeys={}\r\n",
                        title,
                        flags.desc.unwrap_or_default(),
                        flags.keys.unwrap_or_default().join(",")
                    ),
                )
                .expect("Failed to write to file!");
            }
            "done" => {
                if args.len() == 3 {
                    let title = args[2].as_str();
                    if let Some(file) = fs::read_dir("dodos")
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .find_map(|e| {
                            fs::read_dir(format!("dodos/{}", e.file_name().to_string_lossy()))
                                .unwrap()
                                .filter_map(|ie| ie.ok())
                                .find(|ie| ie.file_name() == title)
                        })
                    {
                        let move_path = Path::new(file.path().parent().unwrap())
                            .to_path_buf()
                            .join("done");
                        fs::copy(file.path(), move_path.join(file.file_name()))
                            .expect("Failed to move file to done folder!");
                        fs::remove_file(file.path())
                            .expect("Failed to remove file from old path folder!");
                    }
                } else {
                    panic!(
                        "[ done ] command only accepts 1 argument! Format is [ dodo done \"title\" ]"
                    );
                }
            }
            _ => {
                println!("unknown command sry!");
            }
        }
        println!("{} {}", args[0], args[1]);
    }
    let file_string = String::from_utf8_lossy(&get_file_content("example")).replace("\r\n", " ");
    let file_string_split = file_string
        .split("#####")
        .map(|x| x.trim())
        .collect::<Vec<_>>();
    println!("BEFORE: {file_string:?}");
    assert!(file_string_split.len() == 3);
    println!("AFTER: {file_string_split:?}");
    println!(
        "TITLE={}\nDESC={}\nKEYWORDS={}",
        file_string_split[0], file_string_split[1], file_string_split[2]
    );
    let keywords_split = file_string_split[2].split(",").collect::<Vec<_>>();
    println!("KEYWORDS_SPLIT: {keywords_split:?}");
    println!("todo: create dodo, make no mistakes!");
}
