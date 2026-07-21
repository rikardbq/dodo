use chrono::Local;
use dodo::Command;
use dodo::find_file;
use std::env;
use std::fs;
use std::path::Path;

use dodo::{get_file_content, parse_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args length = {}", args.len());
    if args.len() >= 2 {
        let date = Local::now().format("%Y%m%d").to_string();
        let path_string = format!("dodos/{date}");
        let done_path_string = format!("{path_string}/done");
        let arguments = parse_args(&args);
        match arguments.command.unwrap() {
            Command::New(flags) => {
                let name = flags.name.expect("Task must have at least a name!");
                let path = Path::new(&done_path_string);
                if !path.is_dir() {
                    fs::create_dir_all(path).expect("Folder for task could not be created!");
                }
                fs::write(
                    format!("{path_string}/{}", name),
                    format!(
                        "name={}\r\ndesc={}\r\nkeys={}\r\n",
                        name,
                        flags.desc.unwrap_or_default(),
                        flags.keys.unwrap_or_default().join(",")
                    ),
                )
                .expect("Failed to write to file!");
            }
            Command::Done(val) => {
                if args.len() > 3 {
                    panic!(
                        "[ done ] command only accepts 1 argument! Format is [ dodo done \"name\" ]"
                    );
                }
                if let Some(file) = find_file(&val) {
                    let move_path = Path::new(file.path().parent().unwrap())
                        .to_path_buf()
                        .join("done");
                    fs::copy(file.path(), move_path.join(file.file_name()))
                        .expect("Failed to move file to done folder!");
                    fs::remove_file(file.path())
                        .expect("Failed to remove file from old path folder!");
                }
            }
            Command::Remove(val) => {
                if args.len() > 3 {
                    panic!(
                        "[ remove | rm ] command only accepts 1 argument! Format is [ dodo rm \"name\" ]"
                    );
                }
                if let Some(file) = find_file(&val) {
                    fs::remove_file(file.path())
                        .expect("Failed to remove file from old path folder!");
                }
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
        "NAME={}\nDESC={}\nKEYWORDS={}",
        file_string_split[0], file_string_split[1], file_string_split[2]
    );
    let keywords_split = file_string_split[2].split(",").collect::<Vec<_>>();
    println!("KEYWORDS_SPLIT: {keywords_split:?}");
    println!("todo: create dodo, make no mistakes!");
}
