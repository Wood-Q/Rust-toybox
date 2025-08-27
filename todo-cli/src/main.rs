//制作一个todo的命令行工具
//需要使用的包：clap和serde
//功能规划
//add：添加todo；mv：移除todo；list：列出todo
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(ValueEnum, Clone)]
enum Operation {
    Add,
    Remove,
    List,
    Quit,
    Mark,
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser)]
    operation: Operation,
    //可选参数
    #[clap(short, long, value_parser)]
    content: Option<String>,
    //可选参数
    #[clap(short, long, value_parser)]
    id: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    content: String,
    completed: bool,
}

fn load_todos() -> Result<Vec<Todo>, serde_json::Error> {
    //读取data下的todo.json文件，并反序列化
    let file = File::open("data/todo.json").unwrap();
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader).unwrap();
    Ok(todos)
}

fn add_todo(todo: Todo) -> Result<(), serde_json::Error> {
    let mut todos = load_todos()?;
    let new_todo = todo.clone();
    todos.push(new_todo);
    let file = File::create("data/todo.json").unwrap();
    let writer = BufWriter::new(file);
    //to_writer_pretty是serde_json提供的，用于将结构体序列化为pretty的json格式
    serde_json::to_writer_pretty(writer, &todos).unwrap();
    Ok(())
}

fn remove_todo(id: u32) -> Result<(), serde_json::Error> {
    let mut todos = load_todos()?;
    //retain用于过滤掉不满足条件的元素，这里过滤掉id不等于id的元素
    todos.retain(|todo| todo.id != id);
    //File::create用于创建文件，如果文件不存在，则创建文件，如果文件存在，则覆盖文件
    let file = File::create("data/todo.json").unwrap();
    //BufWriter是用于缓冲写入的，可以提高写入效率
    //和普通写入的区别是，普通写入会直接写入文件，而BufWriter会先写入缓冲区，然后一次性写入文件，这样可以提高写入效率
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &todos)?;
    Ok(())
}

fn list_todos() -> Result<(), serde_json::Error> {
    let todos = load_todos()?;
    for todo in todos {
        println!("{}", serde_json::to_string(&todo).unwrap());
    }
    Ok(())
}

fn mark_finished(id: u32) -> Result<(), serde_json::Error> {
    let mut todos=load_todos()?;
    for todo in todos.iter_mut(){
        if todo.id==id{
            todo.completed=true;
        }
    }
    let file=File::create("data/todo.json").unwrap();
    let writer=BufWriter::new(file);
    serde_json::to_writer_pretty(writer,&todos)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    loop {
        match args.operation {
            Operation::Add => {
                let todo = Todo {
                    id: args.id.unwrap(),
                    content: args.content.unwrap(),
                    completed: false,
                };
                add_todo(todo).unwrap();
                println!("添加成功");
                break;
            }
            Operation::Remove => {
                remove_todo(args.id.unwrap()).unwrap();
                println!("移除成功");
                break;
            }
            Operation::List => {
                list_todos().unwrap();
                break;
            }
            Operation::Mark => {
                mark_finished(args.id.unwrap()).unwrap();
                println!("标记完成");
                break;
            }
            Operation::Quit => {
                println!("退出程序");
                break;
            }
        }
    }
}
