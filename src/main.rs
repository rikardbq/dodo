use chrono::Local;
use std::env;
use std::fs;
use std::path::Path;

use dodo::get_file_content;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args length = {}", args.len());
    if args.len() >= 2 {
        let date = Local::now().format("%Y%m%d").to_string();
        let path_string = format!("dodos/{date}");
        let done_path_string = format!("{path_string}/done");
        match args[1].as_str() {
            "new" => {
                if args.len() == 3 {
                    let task: Vec<&str> = args[2].split("#").collect();
                    if task.len() < 1 {
                        panic!("Task must have at least a title");
                    }
                    let mut stringy = String::new();
                    task.iter().enumerate().for_each(|(i, x)| {
                        if i == 0 {
                            stringy = format!("{stringy}{}", *x);
                        } else {
                            stringy = format!("{stringy}\r\n#####\r\n{}", *x);
                        }
                    });
                    let path = Path::new(&done_path_string);
                    if !path.is_dir() {
                        fs::create_dir_all(path).expect("Folder for task could not be created!");
                    }
                    fs::write(format!("{path_string}/{}", task[0]), stringy)
                        .expect("Failed to write to file!");
                } else {
                    panic!(
                        "[ new ] command only accepts 1 argument! Format is [ title#optional description#key,words ]"
                    );
                }
            }
            "done" => {
                if args.len() == 3 {
                    let title = args[2].as_str();
                    if let Some(file) = fs::read_dir("dodos")
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .find_map(|e| {
                            println!("{:?}", e.file_name());
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
                            .expect("Failed to move task to done!");
                        fs::remove_file(file.path()).expect("Failed to remove file from old path!");
                    }
                } else {
                    panic!(
                        "[ new ] command only accepts 1 argument! Format is [ title#optional description#key,words ]"
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
