use chrono::Local;
use dodo::Command;
use dodo::find_files;
use dodo::move_file;
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

                let files = find_files(&val, false);
                if files.len() > 1 {
                    panic!(
                        "Found multiple files with name {val}\nRun \"dodo done <path_to_task>\"\n{:?}",
                        files
                            .iter()
                            .map(|x| { x.path().to_string_lossy().replacen("dodos/", "", 1) })
                            .collect::<Vec<_>>()
                    )
                }
                
                let name_string = format!("dodos/{}", &val);
                let name_path = Path::new(&name_string);
                if name_path.is_file() {
                    let _ = move_file(&name_path.to_path_buf());
                } else if files.len() == 1 {
                    let _ = move_file(&files[0].path());
                } else {
                    panic!("No tasks with that name found!")
                }
            }
            Command::Remove(val) => {
                if args.len() > 3 {
                    panic!(
                        "[ remove | rm ] command only accepts 1 argument! Format is [ dodo rm \"name\" ]"
                    );
                }
                let files = find_files(&val, true);
                if files.len() > 1 {
                    panic!(
                        "Found multiple files with name {val}\nRun \"dodo rm <path_to_task>\"\n{:?}",
                        files
                            .iter()
                            .map(|x| { x.path().to_string_lossy().replacen("dodos/", "", 1) })
                            .collect::<Vec<_>>()
                    )
                }
                
                let name_string = format!("dodos/{}", &val);
                let name_path = Path::new(&name_string);
                if name_path.is_file() {
                    fs::remove_file(name_path).expect("Failed to remove file!")
                } else if files.len() == 1 {
                    fs::remove_file(files[0].path()).expect("Failed to remove file!");
                } else {
                    panic!("No tasks with that name found!")
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
