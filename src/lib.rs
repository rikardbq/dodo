use std::{collections::HashMap, fs::File, io::Read};

// example structs
#[derive(Clone)]
pub struct Flags {
    pub title: Option<String>,
}

#[derive(Clone)]
pub enum Command {
    New(Flags),
}

impl Flags {
    pub fn default() -> Self {
        Self { title: None }
    }
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
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

pub fn parse_args() -> Arguments {
    let mut arg = Arguments::default();
    arg.set_command(Command::New(Flags::default()));
    let a = match arg.command.clone().unwrap() {
        Command::New(f) => {
            if f.title.unwrap() == "asd".to_string() {}
            Some("")
        }
        _ => None,
    };

    arg
}
// example structs end

const ACCEPTED_FLAGS: [&str; 6] = ["title", "desc", "keys", "t", "d", "k"];
pub struct CliFlags {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub keys: Option<Vec<String>>,
}

impl CliFlags {
    pub fn default() -> Self {
        Self {
            title: None,
            desc: None,
            keys: None,
        }
    }
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

pub fn parse_flags(cli_args: Vec<String>) -> CliFlags {
    let mut flags = CliFlags::default();
    let mut flag_vals: HashMap<String, String> = HashMap::new();
    cli_args.iter().enumerate().for_each(|(i, x)| {
        if x.starts_with("-") {
            let formatted_flag = x.replace("-", "");
            if ACCEPTED_FLAGS.contains(&formatted_flag.as_str()) {
                if cli_args.len() == i + 1 {
                    panic!("Flag {x} has no value!");
                }
                let _ = flag_vals.insert(formatted_flag.clone(), cli_args[i + 1].to_string());
            } else {
                panic!("Unknown flag! {x}");
            }
        }
    });
    if let Some(title) = match flag_vals.get("title") {
        Some(v) => Some(v),
        None => flag_vals.get("t"),
    } {
        flags.set_title(title.clone());
    }
    if let Some(desc) = match flag_vals.get("desc") {
        Some(v) => Some(v),
        None => flag_vals.get("d"),
    } {
        flags.set_desc(desc.clone());
    }
    if let Some(keys) = match flag_vals.get("keys") {
        Some(v) => Some(v),
        None => flag_vals.get("k"),
    } {
        let k: Vec<String> = keys.split(" ").map(|x| x.to_string()).collect();
        flags.set_keys(k);
    }

    flags
}
