use std::{
    fs::{self, DirEntry, File}, io::Read, path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct New {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Command {
    New(New),
    Done(String),
    Remove(String),
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
    cli_args.len() == idx + 1
}

pub fn parse_args(cli_args: &Vec<String>) -> Arguments {
    let mut args = Arguments::default();
    cli_args.iter().enumerate().for_each(|(i, x)| {
        if x.starts_with("-") {
            let formatted_flag = x.replace("-", "");
            if arg_has_val(cli_args, i) {
                panic!("Flag {x} has no value!");
            }
            let flag_val = &cli_args[i + 1];
            if let Some(cmd) = &mut args.command {
                match cmd {
                    Command::New(f) => match formatted_flag.as_str() {
                        "name" | "n" => f.set_name(flag_val.clone()),
                        "desc" | "d" => f.set_desc(flag_val.clone()),
                        "keys" | "k" => {
                            f.set_keys(flag_val.split(" ").map(|x| x.to_string()).collect())
                        }
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
                    if arg_has_val(cli_args, i) {
                        panic!("Command {x} has no value!");
                    }
                    args.set_command(Command::Done(cli_args[i + 1].clone()));
                }
                "remove" | "rm" => {
                    if arg_has_val(cli_args, i) {
                        panic!("Command {x} has no value!");
                    }
                    args.set_command(Command::Remove(cli_args[i + 1].clone()));
                }
                _ => {}
            };
        }
    });

    args
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

pub fn find_files(name: &str, full: bool) -> Vec<DirEntry> {
    fs::read_dir("dodos")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            fs::read_dir(format!("dodos/{}", e.file_name().to_string_lossy()))
                .unwrap()
                .filter_map(|ie| ie.ok())
                .map(|ie| {
                    if full && ie.metadata().unwrap().is_dir() {
                        fs::read_dir(format!(
                            "dodos/{}/{}",
                            e.file_name().to_string_lossy(),
                            ie.file_name().to_string_lossy()
                        ))
                        .unwrap()
                        .filter_map(|iie| iie.ok())
                        .find(|iie| iie.file_name() == name)
                    } else {
                        Some(ie)
                    }
                })
                .find(|ie| ie.is_some() && ie.as_ref().unwrap().file_name() == name)
        })
        .map(|e| e.unwrap())
        .collect()
}

pub fn move_file(file_path: &PathBuf) -> bool {
    let move_path = Path::new(file_path.parent().unwrap())
        .to_path_buf()
        .join("done");
    if let Ok(_) = fs::copy(file_path, move_path.join(file_path.file_name().unwrap())) {
        let remove = fs::remove_file(file_path);

        return remove.is_ok();
    }

    false
}
