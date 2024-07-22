use clap::{command, Arg, Command};
use rs_todo_cli::todo;

fn main() {
    let mut todo_list = todo::List::new();

    let add = Command::new("add")
        .arg(
            Arg::new("TODO")
                .required(true)
                .help("name for the todo item"),
        )
        .about("add todo into the list");

    let view = Command::new("view").about("show current to do list");

    let matches = command!().subcommand(add).subcommand(view).get_matches();

    if let Some(add_match) = matches.subcommand_matches("add") {
        let todo_title = match add_match.get_one::<String>("TODO") {
            Some(arg) => arg,
            None => return,
        };

        todo_list.add(todo_title.as_str());
        println!("added {} to the list", todo_title);
        println!("total {} items in the list", todo_list.len());
    }

    if let Some(_) = matches.subcommand_matches("view") {
        if todo_list.len() <= 0 {
            println!("no items in the list");
            return;
        }

        println!("{todo_list}");
    }
}
