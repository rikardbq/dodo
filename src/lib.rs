use std::{
    fmt,
    fs::{self, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
};

pub enum DodoErrorKind {
    General,
    DuplicateFile,
    FileNotFound,
}

pub struct DodoError {
    pub kind: Option<DodoErrorKind>,
    pub message: Option<String>,
}

impl DodoError {
    pub fn new(kind: DodoErrorKind) -> Self {
        Self {
            kind: Some(kind),
            message: Some(String::from("Something went wrong!")),
        }
    }

    pub fn with_message(self, message: &str) -> Self {
        Self {
            message: Some(String::from(message)),
            ..self
        }
    }

    pub fn create_panic(self) {
        panic!("{self}");
    }
}

impl fmt::Display for DodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = match self.kind.as_ref().unwrap() {
            DodoErrorKind::General => "Error:General=",
            DodoErrorKind::FileNotFound => "Error:FileNotFound=",
            DodoErrorKind::DuplicateFile => "Error:DuplicateFile=",
        };
        write!(f, "{kind_str}{}", self.message.clone().unwrap_or_default())
    }
}

#[derive(Clone, Debug)]
pub struct New {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct List {
    pub all: bool,
    pub done: bool,
    pub filter: Option<String>,
    pub search: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Command {
    New(New),
    Done(String),
    Remove(String),
    List(List),
}

// impl fmt::Display for Command {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Command::New(_) => write!(f, "new"),
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

trait Flags {
    fn default() -> Self;
}

impl Flags for New {
    fn default() -> Self {
        Self {
            name: None,
            desc: None,
            keys: None,
        }
    }
}

impl Flags for List {
    fn default() -> Self {
        Self {
            all: false,
            done: false,
            filter: None,
            search: None,
        }
    }
}

impl New {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn set_desc(&mut self, desc: String) {
        self.desc = Some(desc);
    }
    pub fn set_keys(&mut self, keys: Vec<String>) {
        self.keys = Some(keys);
    }
}

impl List {
    pub fn set_all(&mut self, all: bool) {
        self.all = all;
    }
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
    pub fn set_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }
    pub fn set_search(&mut self, search: String) {
        self.search = Some(search);
    }
}

pub struct Arguments {
    pub command: Option<Command>,
}

impl Arguments {
    pub fn default() -> Self {
        Self { command: None }
    }
    pub fn set_command(&mut self, command: Command) {
        self.command = Some(command);
    }
}

pub fn arg_has_val(cli_args: &Vec<String>, idx: usize) -> bool {
    let mut predicate = cli_args.len() != idx + 1;
    if predicate {
        let val = cli_args[idx + 1].clone();
        predicate = predicate && !val.starts_with("-");
    }

    predicate
}

pub fn get_flag_val(cli_args: &Vec<String>, idx: usize) -> String {
    if !arg_has_val(cli_args, idx) {
        panic!("Flag {} has no value!", cli_args[idx]);
    }
    cli_args[idx + 1].clone()
}

pub fn parse_args(cli_args: &Vec<String>) -> Arguments {
    let mut args = Arguments::default();
    cli_args.iter().enumerate().for_each(|(i, x)| {
        if x.starts_with("-") {
            let formatted_flag = x.replace("-", "");
            if let Some(cmd) = &mut args.command {
                match cmd {
                    Command::New(f) => match formatted_flag.as_str() {
                        "name" | "n" => f.set_name(get_flag_val(cli_args, i)),
                        "desc" | "d" => f.set_desc(get_flag_val(cli_args, i)),
                        "keys" | "k" => f.set_keys(
                            get_flag_val(cli_args, i)
                                .split(" ")
                                .map(|x| x.to_string())
                                .collect(),
                        ),
                        _ => panic!("Unknown flag! {formatted_flag}"),
                    },
                    Command::List(f) => match formatted_flag.as_str() {
                        "filter" | "f" => f.set_filter(get_flag_val(cli_args, i)),
                        "search" | "s" => f.set_search(get_flag_val(cli_args, i)),
                        "all" | "a" => {
                            if arg_has_val(cli_args, i) {
                                panic!("The \"-all\" flag does not accept an argument!");
                            }
                            f.set_all(true);
                        }
                        "done" | "d" => {
                            if arg_has_val(cli_args, i) {
                                panic!("The \"-done\" flag does not accept an argument!");
                            }
                            f.set_done(true);
                        },
                        _ => panic!("Unknown flag! {formatted_flag}"),
                    },
                    _ => {}
                }
            }
        } else {
            match x.as_str() {
                "new" => {
                    args.set_command(Command::New(New::default()));
                }
                "done" => {
                    if !arg_has_val(cli_args, i) {
                        panic!("Command {x} has no value!");
                    }
                    args.set_command(Command::Done(cli_args[i + 1].clone()));
                }
                "remove" | "rm" => {
                    if !arg_has_val(cli_args, i) {
                        panic!("Command {x} has no value!");
                    }
                    args.set_command(Command::Remove(cli_args[i + 1].clone()));
                }
                "list" | "ls" => {
                    args.set_command(Command::List(List::default()));
                }
                _ => {}
            };
        }
    });

    args
}

pub fn list_tasks() -> Vec<DirEntry> {
    fs::read_dir("dodos")
        .unwrap()
        .filter_map(|e| e.ok())
        .flat_map(|e| fs::read_dir(e.path()).unwrap().filter_map(|ie| ie.ok()))
        .filter(|e| e.metadata().unwrap().is_file())
        .collect()
}

pub fn get_file_content(path: &str) -> Vec<u8> {
    let mut file_buf = Vec::<u8>::new();
    let _ = (match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("{:?}", err)
        }
    })
    .read_to_end(&mut file_buf);

    file_buf
}

pub fn find_file<'a>(name: &str, full: bool) -> Vec<PathBuf> {
    let files: Vec<PathBuf> = fs::read_dir("dodos")
        .unwrap()
        .filter_map(|e| e.ok())
        .flat_map(|e| {
            fs::read_dir(e.path())
                .unwrap()
                .filter_map(|ie| ie.ok())
                .map(|ie| {
                    if full && ie.metadata().unwrap().is_dir() {
                        fs::read_dir(ie.path())
                            .unwrap()
                            .filter_map(|iie| iie.ok())
                            .find(|iie| iie.file_name() == name)
                    } else {
                        Some(ie)
                    }
                })
                .filter(|ie| ie.is_some() && ie.as_ref().unwrap().file_name() == name)
        })
        .map(|e| e.unwrap().path())
        .collect();

    let name_string = format!("dodos/{}", name);
    let name_path = Path::new(&name_string);
    if name_path.is_file() {
        return vec![name_path.to_path_buf()];
    }

    files
}

pub fn move_file(file_path: &PathBuf) -> bool {
    let move_path = Path::new(file_path.parent().unwrap())
        .to_path_buf()
        .join("done");
    if let Ok(_) = fs::copy(&file_path, move_path.join(file_path.file_name().unwrap())) {
        let remove = fs::remove_file(file_path);

        return remove.is_ok();
    }

    false
}

pub fn clean_entry_path(entry: &str) -> String {
    entry
        .replacen("dodos", "", 1)
        .replacen("\\", "", 1)
        .replacen("/", "", 1)
}

pub fn check_files_list_for_error(name: &str, files: &Vec<PathBuf>) {
    if files.len() > 1 {
        DodoError::new(DodoErrorKind::DuplicateFile)
            .with_message(&format!(
                "Found multiple files with name {name}.\n----\n{:?}",
                files
                    .iter()
                    .map(|x| { clean_entry_path(&x.to_string_lossy()) })
                    .collect::<Vec<_>>()
            ))
            .create_panic();
    } else if files.len() == 0 {
        DodoError::new(DodoErrorKind::FileNotFound)
            .with_message("No tasks with that name found!")
            .create_panic();
    }
}
