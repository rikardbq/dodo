use chrono::Local;
use dodo::Command;
use dodo::DodoError;
use dodo::DodoErrorKind;
use dodo::check_files_list_for_error;
use dodo::clean_entry_path;
use dodo::find_file;
use dodo::get_file_content;
use dodo::list_tasks;
use dodo::move_file;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use dodo::parse_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args length = {}", args.len());
    if args.len() >= 2 {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let path_string = format!("dodos/{date}");
        let done_path_string = format!("{path_string}/done");
        let arguments = parse_args(&args);
        match arguments.command.unwrap() {
            Command::New(flags) => {
                let name = flags.name.expect("Task must have at least a name!");
                let done_path = Path::new(&done_path_string);
                if !done_path.is_dir() {
                    fs::create_dir_all(done_path).expect("Folder for task could not be created!");
                }
                let files = find_file(&name, true);
                if files.len() > 0 {
                    DodoError::new(DodoErrorKind::DuplicateFile)
                        .with_message(&format!("Task with name {name} already exist!"))
                        .create_panic();
                }

                fs::write(
                    format!("{path_string}/{name}"),
                    format!(
                        "name={}\r\ndesc={}\r\ntags={}\r\n",
                        name,
                        flags.desc.unwrap_or_default(),
                        flags.tags.unwrap_or_default().join(",")
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

                let files = find_file(&val, false);
                check_files_list_for_error(&val, &files);
                move_file(&files[0]);
            }
            Command::Remove(val) => {
                if args.len() > 3 {
                    panic!(
                        "[ remove | rm ] command only accepts 1 argument! Format is [ dodo rm \"name\" ]"
                    );
                }

                let files = find_file(&val, true);
                check_files_list_for_error(&val, &files);
                fs::remove_file(files[0].clone()).expect("Failed to remove file!");
            }
            Command::List(flags) => {
                /*
                struct FilterPredicate {
                    or: bool,
                    and: bool,
                    not: bool,
                }
                while something {
                    fn parse_and_filter(filter_part: &str, tasks: Vec<Vec<&str>>)
                    
                    iterate and keep composing bools to or / and / not predicate struct fields
                    
                    store the filter parts that have been parsed and iterated so that a final filtering of the whole task list may happen?
                }
                flags filter precedence
                let mut predicate = true;
                // pseudo code -> {
                    if filter contains "and" && (idx - 1) == " " && (idx + 1) == " "
                    split_filter_and
                    &mut predicate = split_filter_and[0] && split_filter_and[1];    
                    if filter contains "or" && (idx - 1) == " " && (idx + 1) == " "
                    split_filter_or
                    &mut predicate = predicate || handle_and(split_filter_or)
                }
                
                return predicate;
                */
                let extract_and_print_task = |x: &DirEntry| {
                    let path = x.path();
                    let task = x.file_name();
                    let parent = path.parent().unwrap().to_string_lossy();
                    let file_content = get_file_content(&path.to_string_lossy());
                    let file_string = String::from_utf8_lossy(&file_content);
                    let file_split: Vec<&str> = file_string
                        .split("\r\n")
                        .filter(|l| l.trim().len() > 0)
                        .collect();
                    if file_split.len() != 3 {
                        DodoError::new(DodoErrorKind::MalformedTask)
                            .with_message(&format!(
                                "Task \"{}\" row count is wrong, expected 3 but was {}",
                                task.to_string_lossy(),
                                file_split.len()
                            ))
                            .create_panic();
                    }
                    let tags = file_split[2].split("=").collect::<Vec<&str>>()[1];
                    println!(
                        "[{}][{}] - {}",
                        clean_entry_path(&parent),
                        tags,
                        task.to_string_lossy()
                    );
                };

                list_tasks().iter().for_each(extract_and_print_task);
            }
        }
        println!("{} {}", args[0], args[1]);
    }
}
