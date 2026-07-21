use std::{fs::File, io::Read};

#[derive(Clone, Debug)]
pub struct New {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub enum Command {
    New(New),
    Done(String),
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
            title: None,
            desc: None,
            keys: None,
        }
    }
}

impl New {
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
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
                        "title" | "t" => f.set_title(flag_val.clone()),
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
                },
                "done" => {
                    if arg_has_val(cli_args, i) {
                        panic!("Command {x} has no value!");
                    }
                    args.set_command(Command::Done(cli_args[i + 1].clone()));
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
