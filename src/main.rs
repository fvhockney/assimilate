use std::env;
use std::env::VarError;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    here: bool,
    #[structopt(short, long)]
    export: bool,
    #[structopt(short, long)]
    alias: bool,
    #[structopt(short, long)]
    save: bool,
    #[structopt(short, long)]
    name: String,
    item: Vec<String>,
}

#[derive(Debug)]
enum MyError {
    FileNotFound(PathBuf),
    EnvNotFound(VarError),
    DualMethod,
}

impl Error for MyError {}

impl From<VarError> for MyError {
    fn from(err: VarError) -> MyError {
        MyError::EnvNotFound(err)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            MyError::FileNotFound(file) => {
                write!(f, "{} not found", file.to_str().unwrap_or_else(|| "file"))
            }
            MyError::EnvNotFound(_) => write!(f, "Please export EXPORT_FILE or ALIAS_FILE"),
            MyError::DualMethod => write!(f, "You may only use alias or export, not both"),
        }
    }
}

fn get_file_path(method: &Method) -> Result<PathBuf, MyError> {
    let path = match method {
        Method::Alias => PathBuf::from(env::var("ALIAS_FILE")?),
        Method::Export => PathBuf::from(env::var("EXPORT_FILE")?),
    };
    Ok(path)
}

fn check_for_file(file: &PathBuf) -> Result<(), MyError> {
    match file.is_file() {
        true => Ok(()),
        false => Err(MyError::FileNotFound(file.to_path_buf())),
    }
}

fn save(command: &String, file_name: PathBuf) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_name)
        .expect("file does not exist");
    file.write_all(command.as_bytes()).expect("failed to write");
    write!(file, "\n").expect("failed to write");
}

enum Method {
    Alias,
    Export,
}

fn build_command(method: &Method, name: String, item: Vec<String>) -> String {
    let mut command = String::new();
    let method = match method {
        Method::Alias => "alias ",
        Method::Export => "export ",
    };
    command.push_str(method);
    command.push_str(&name);
    command.push_str("='");
    let mut iter = item.iter();
    if let Some(x) = iter.nth(0) {
        command.push_str(x);
    }
    iter.for_each(|x| {
        command.push_str(" ");
        command.push_str(x);
    });
    command.push_str("'");
    command
}

fn determine_method(alias: bool, export: bool) -> Result<Method, MyError> {
    if alias && export {
        Err(MyError::DualMethod)
    } else if alias {
        Ok(Method::Alias)
    } else {
        Ok(Method::Export)
    }
}

fn run() -> Result<String, MyError> {
    let args = Cli::from_args();
    let method = determine_method(args.alias, args.export)?;
    let command = build_command(&method, args.name, args.item);
    if args.save {
        let file_path = get_file_path(&method)?;
        check_for_file(&file_path)?;
        save(&command, file_path);
    }
    if args.here {
        Ok(command)
    } else {
        Ok("success".to_string())
    }
}

fn main() {
    match run() {
        Ok(res) => println!("{}", res),
        Err(err) => {
            println!("{}", err);
            exit(127)
        }
    }
}
