mod todo;
mod todo_conf;
mod todo_item;
mod todo_parser;

use crate::todo::{add_item, init, list_all, list_filter};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Todo")
        .version("0.1")
        .author("David S <david.seruyange@gmail.com>")
        .about("Rust port of Todo.sh")
        .arg(
            Arg::with_name("ADD")
                .short("a")
                .long("add")
                .value_name("ADD")
                .help("Add an item to todo list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("REMOVE")
                .short("r")
                .long("remove")
                .value_name("REMOVE")
                .help("Remove an item to todo list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LIST")
                .short("l")
                .long("list")
                .value_name("LIST")
                .help("Displays filtered items on todo list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LISTALL")
                .short("f")
                .long("listall")
                .value_name("LISTALL")
                .help("Displays all items on todo list")
                .takes_value(false),
        )
        .get_matches();

    let todo_conf = init().unwrap();

    if let Some(new_item) = matches.value_of("ADD") {
        let line_item = new_item.to_owned() + "\n";
        add_item(line_item.as_str(), &todo_conf);
    }

    if let Some(larg) = matches.value_of("LIST") {
        list_filter(larg, &todo_conf);
    }

    if let Some(_larg) = matches.value_of("LISTALL") {
        list_all(&todo_conf);
    }

    println!("I have finished");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}