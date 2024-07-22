use std::{env, process::exit};

use clap::{command, Arg, Command};
use rs_todo_cli::{get_date_str, get_save_file, save_to_file, todo};

fn main() {
    let save_path = match get_save_file() {
        Ok(spath) => spath,
        Err(e) => {
            eprintln!("failed to get save path: {}", e);
            return;
        }
    };

    let mut todo_list = match todo::List::from_file(&save_path) {
        Ok(list) => list,
        Err(_) => todo::List::new(),
    };

    let add = Command::new("add")
        .arg(
            Arg::new("TODO")
                .required(true)
                .help("name for the todo item"),
        )
        .about("add todo into the list");

    let delete = Command::new("delete")
        .arg(
            Arg::new("item_num")
                .required(true)
                .help("item number to be deleted"),
        )
        .about("delete item from the list");

    let view = Command::new("view").about("show current to do list");

    let update = Command::new("update")
        .arg(
            Arg::new("title")
                .long("title")
                .short('t')
                .help("title of the todo item"),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .short('s')
                .help("status: 1 for done, 0 for in progress"),
        )
        .arg(
            Arg::new("item_num")
                .required(true)
                .help("list number of the todo item"),
        )
        .about("update todo items: update <id> --title <title> --status <0 or 1>");

    let clear = Command::new("clear").about("clear current list");

    let matches = command!()
        .subcommand(add)
        .subcommand(view)
        .subcommand(update)
        .subcommand(clear)
        .subcommand(delete)
        .get_matches();

    if let Some(add_match) = matches.subcommand_matches("add") {
        let todo_title = match add_match.get_one::<String>("TODO") {
            Some(arg) => arg,
            None => return,
        };

        todo_list.add(todo_title.as_str());

        println!("added \"{}\" to the list", todo_title);
        println!("total {} items in the list", todo_list.len());
    }

    if let Some(del_match) = matches.subcommand_matches("delete") {
        let item_num = match del_match.get_one::<String>("item_num") {
            Some(arg) => {
                let inum = match arg.parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{e}");
                        return;
                    }
                };

                inum
            }

            None => return,
        };

        todo_list.delete(item_num);
    }

    if let Some(_) = matches.subcommand_matches("view") {
        let date_str = get_date_str();

        println!("{date_str}\n");

        if todo_list.len() <= 0 {
            println!("no items in the list");
            return;
        }

        println!("{todo_list}");
    }

    if let Some(upd_match) = matches.subcommand_matches("update") {
        let item_num = match upd_match.get_one::<String>("item_num") {
            Some(v) => {
                let iv = match v.to_string().parse::<usize>() {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("failed to get item from the list: {}", e);
                        return;
                    }
                };

                iv
            }
            None => {
                eprintln!("failed to get item from the list");
                return;
            }
        };

        if let Some(title) = upd_match.get_one::<String>("title") {
            if let Err(e) = todo_list.update_item_title(item_num, title) {
                eprintln!("{e}");
                return;
            };
        }

        if let Some(status) = upd_match.get_one::<String>("status") {
            let status = match status.as_str().trim() {
                "1" => true,
                "0" => false,
                _ => {
                    eprintln!("failed to parse status correctly");
                    return;
                }
            };

            if let Err(e) = todo_list.update_item_status(item_num, status) {
                eprintln!("{e}");
                return;
            };
        }
    }

    if let Some(_) = matches.subcommand_matches("clear") {
        todo_list.clear();
    }

    // write all into file before ending the program
    let todo_str = todo_list.to_string();

    match save_to_file(&save_path, todo_str.as_str()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("something went wrong while saving todo list: {}", e);
            exit(1);
        }
    }
}
