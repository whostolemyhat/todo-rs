// http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
extern crate rustc_serialize;
extern crate rusqlite;

use std::io;
use rustc_serialize::json;
use rusqlite::Connection;

const DB_PATH: &'static str = "todos.db";

#[derive(Debug)]
struct Todo {
  id: i32,
  title: String,
  completed: bool,
  deleted: bool
}

fn db_setup(db: &Connection) {
  let del = "DROP TABLE todos";
  let setup = "CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    completed INTEGER,
    deleted INTEGER
  )";

  let insert = "INSERT INTO todos (title, completed, deleted) VALUES('Create todos', 0, 0)";
  let insert2 = "INSERT INTO todos (title, completed, deleted) VALUES('Create table', 1, 0)";

  db.execute(del, &[]).unwrap();
  db.execute(setup, &[]).unwrap();
  db.execute(insert, &[]).unwrap();
  db.execute(insert2, &[]).unwrap();
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
  let new_id = todos.len() as i32 + 1;
  todos.push(Todo {
    id: new_id,
    title: title.to_string(),
    completed: false,
    deleted: false
  });
}

fn remove_todo(todos: &mut Vec<Todo>, todo_id: i32) {
  if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
    todo.deleted = true;
  }
}

fn complete_todo(todos: &mut Vec<Todo>, todo_id: i32) {
  if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
    todo.completed = true;
  }
}

fn print_todos(todos: &Vec<Todo>) {
  println!("\n\nTodo List:\n ---------------------- ");
  for todo in todos {
    if !todo.deleted {
      let done = if todo.completed { "✔" } else { " " };
      println!("[{}] {} {}", done, todo.id, todo.title);
    }
  }
}

fn invalid_command(command: &str) {
  println!("Invalid command: {}", command);
}

fn main() {
  // connect to db
  let db = Connection::open(DB_PATH).expect("Failed to connect to db");
  db_setup(&db);

  let mut read_todos = db.prepare("SELECT id, title, completed, deleted FROM todos").unwrap();
  let mut todos_iter = read_todos.query_map(&[], |row| {
    Todo {
      id: row.get(0),
      title: row.get(1),
      completed: row.get(2),
      deleted: row.get(3)
    }
  }).unwrap();

  let mut todos: Vec<Todo> = Vec::new();
  for todo in todos_iter {
    // println!("Found todo {:?}", todo.unwrap());
    todos.push(todo.unwrap());
  }

  print_todos(&todos);

  loop {
    let mut command = String::new();
    io::stdin()
      .read_line(&mut command)
      .expect("Failed to read line");

    let command_parts: Vec<&str> = command.split_whitespace().collect();
    match command_parts.len() {
      0 => invalid_command(&command),
      // len is 1 = command
      1 => match command_parts[0] {
        "list" => print_todos(&todos),
        _ => invalid_command(&command)
      },
      // len > 1 - add x y z etc
      _ => {
        match command_parts[0] {
          "add" => add_todo(&mut todos, &command_parts[1..].join(" ")),
          // remove :todo_id
          "remove" => if let Ok(num) = command_parts[1].parse::<i32>() {
            remove_todo(&mut todos, num)
          },
          "done" => if let Ok(num) = command_parts[1].parse::<i32>() {
            complete_todo(&mut todos, num)
          },
          _ => invalid_command(&command)
        }
      }
    }

    print_todos(&todos);
  }
}
