use std::collections::HashMap;

pub enum Command {
    List,
    New {name: Option<String>, user: Option<String>, pass: Option<String>},
    Edit {item: Option<usize>, name: Option<String>, user: Option<String>, pass: Option<String>},
    Delete(Option<usize>),
    Get(Option<usize>),
    ChangeMaster(Option<String>),
    Help,
    None
}

pub struct Config {
    pub database_name: String,
    pub command: Command,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        // For operations on a database file, the next arg will be the database name,
        // otherwise handle the command and return with an appropriate config early
        let database_name = match args.next().as_deref() {
            Some("new") => {
                match args.next() {
                    Some(name) => {
                        return Ok(Config { database_name: name, command: Command::None });
                    },
                    None => return Err("No database name was entered for the `new` command"),
                }
            },
            Some("help") | Some("-h") => {
                return Ok(Config { database_name: String::from(""), command: Command::Help});
            },
            Some(arg) => arg.to_string(),
            None => return Err("No command options entered"),
        };

        // Sort command inputs into a Command enum
        let command = match args.next() {
            Some(arg) => match arg.as_str() {
                "list" => Command::List,
                "new" => Command::New { name: args.next(), user: args.next(), pass: args.next() },
                "edit" => {
                    let item = args.next().unwrap().parse::<usize>();

                    let mut edit_args: HashMap<&str, Option<String>> = HashMap::new();

                    loop {
                        let arg = args.next();
                        if arg.is_none() {
                            break;
                        }
                        match arg.unwrap().as_str() {
                            "-n" => edit_args.insert("-n", args.next()),
                            "-u" => edit_args.insert("-u", args.next()),
                            "-p" => edit_args.insert("-p", args.next()),
                            _ => None,
                        };
                    }

                    Command::Edit { 
                        item: item.ok(), 
                        name: match edit_args.get("-n") {
                            Some(name) => name.clone(),
                            None => None,
                        },
                        user: match edit_args.get("-u") {
                            Some(user) => user.clone(),
                            None => None,
                        }, 
                        pass: match edit_args.get("-p") {
                            Some(pass) => pass.clone(),
                            None => None,
                        },
                    }
                },
                "delete" => {
                    let item = args.next().unwrap().parse::<usize>();

                    Command::Delete(item.ok())
                },
                "get" => {
                    let item = args.next().unwrap().parse::<usize>();

                    Command::Get(item.ok())
                },
                "updatepass" => {
                    let new_pass = args.next();

                    Command::ChangeMaster(new_pass)
                }
                _ => return Err("Command option does not exist"),
            },
            None => return Err("Didn't get a command"),
        };

        Ok(Config {
            database_name: String::from(database_name),
            command,
        })
    }
}