use crate::todo_conf::TodoConf;
use crate::todo_item::TodoItem;
use chrono::Utc;
use nanoid::nanoid;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::todo_file::TodoFile;

pub fn init() -> Result<TodoConf, &'static str> {
    let home_dir_unwrap = dirs::home_dir().unwrap();
    let home_dir = home_dir_unwrap.to_str();
    let home_dir_string = home_dir.unwrap().to_string() + "/todo.conf";
    let home_dir_todo = home_dir.unwrap().to_string() + "/todo.json";

    let b = std::path::Path::new(home_dir_string.as_str()).exists();
    if b {
        let contents = fs::read_to_string(home_dir_string).expect("Something went wrong reading the file");
        let deserialized: TodoConf = serde_json::from_str(&contents).unwrap();

        Ok(deserialized)
    } else {
        let new_conf = TodoConf {
            owner: "Anon".to_string(),
            conf_path: home_dir_string,
            list_path: home_dir_todo,
        };
        save_conf(&new_conf);

        Ok(new_conf)
    }
}

fn save_conf(conf: &TodoConf){
    let serialized = serde_json::to_string(conf).unwrap();
    let conf_path = &conf.list_path;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(conf_path.as_str())
        .expect("cannot open file");
    file.write_all(serialized.as_bytes()).expect("write failed!");
}

fn build_todo_item(item: &str) -> Result<TodoItem, &'static str> {
    let contexts = parse_todo_context(item);
    let projects = parse_todo_projects(item);

    let todo = TodoItem {
        id: generate_id(),
        create: Utc::now().to_rfc3339(),
        entry: item.to_string(),
        context_info: contexts.unwrap(),
        project_info: projects.unwrap(),
    };
    Ok(todo)
}

fn generate_id() -> String {
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let id = nanoid!(4, &alphabet);
    id
}

fn parse_todo_context(item: &str) -> Result<Vec<String>, &'static str> {
    let mut context_data: Vec<String> = vec![];
    let re = Regex::new(r"@(\w+)").unwrap();
    for cap in re.captures_iter(item) {
        let cap_ref = &cap[0].to_string();
        let cap_ref_string = cap_ref.to_string();
        context_data.push(cap_ref_string);
    }
    Ok(context_data)
}

fn parse_todo_projects(item: &str) -> Result<Vec<String>, &'static str> {
    let mut context_data: Vec<String> = vec![];
    let re = Regex::new(r"p:(\w+)").unwrap();
    for cap in re.captures_iter(item) {
        let cap_ref = &cap[0].to_string();
        let cap_ref_string = cap_ref.to_string();
        context_data.push(cap_ref_string);
    }
    Ok(context_data)
}

fn write_to_file(item: &str, _conf: &TodoConf) {
    let fp = String::from("/Users/David.Seruyange/temp/todo.txt");
    let b = std::path::Path::new(fp.as_str()).exists();
    if b {
        let mut file = OpenOptions::new()
            .append(true)
            .open(fp.as_str())
            .expect("cannot open file");
        file.write_all(item.as_bytes()).expect("write failed!");
    } else {
        let mut file = std::fs::File::create(fp.as_str()).expect("create failed");
        file.write_all(item.as_bytes()).expect("write failed");
    }
}

fn read_from_file(_conf: &TodoConf) -> String {
    // let fp = String::from("/Users/David.Seruyange/temp/todo.txt");
    let fp = _conf.list_path.clone();
    let b = std::path::Path::new(fp.as_str()).exists();
    if b {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");

        let mut line_no = 0;
        let lines: Vec<&str> = contents.split("\n").collect();

        let mut todo_list: Vec<String> = Vec::new();
        for line in lines {
            todo_list.push(format!("{}-{}", line_no, line.to_string()));
            line_no += 1;
        }

        todo_list.join("\n")
    } else {
        "".to_string()
    }
}

pub fn _get_file(conf: &TodoConf) -> TodoFile{
    let fs_content = read_from_file(conf);
    serde_json::from_str(&fs_content).unwrap()

}

pub fn add_item(item: &str, conf: &TodoConf) {
    let todo = build_todo_item(item).unwrap();
    let serialized = serde_json::to_string(&todo).unwrap();
    write_to_file(&serialized, conf);

    println!("Added new item: {}", item);
}

pub fn list_all(conf: &TodoConf) {
    print!("{}", read_from_file(conf));
}

pub fn list_filter(larg: &str, conf: &TodoConf) {
    println!("Got a filtered list {} - {}", larg, &conf.list_path);
}
