use std::collections::HashMap;

pub enum Command {
    List,
    New {name: Option<String>, user: Option<String>, pass: Option<String>},
    Edit {item: Option<usize>, name: Option<String>, user: Option<String>, pass: Option<String>},
    Delete(Option<usize>),
    Get(Option<usize>),
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

        let database_name = match args.next().as_deref() {
            Some("new") => {
                match args.next() {
                    Some(name) => {
                        return Ok(Config { database_name: name, command: Command::None });
                    },
                    None => return Err("No database name was entered for the `new` command"),
                }
            },
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get a database file name"),
        };

        // Sort command inputs into a Command enum
        let command = match args.next() {
            Some(arg) => match arg.as_str() {
                "list" => Command::List,
                "new" => Command::New { name: args.next(), user: args.next(), pass: args.next() },
                "edit" => {
                    let item = args.next().unwrap().parse::<usize>();

                    let mut edit_args = HashMap::new();

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
                        name: edit_args.get("-n").unwrap().clone(), 
                        user: edit_args.get("-u").unwrap().clone(), 
                        pass: edit_args.get("-p").unwrap().clone() 
                    }
                },
                "delete" => {
                    let item = args.next().unwrap().parse::<usize>();

                    Command::Delete(item.ok())
                },
                "get" => {
                    let item = args.next().unwrap().parse::<usize>();

                    Command::Get(item.ok())
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