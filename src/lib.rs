use pyo3::prelude::*;
use std::fs;
use std::io::stdin;
use std::io::Result as IoResult;

/// Create a task
#[pyfunction]
pub fn create_task() -> PyResult<()> {
    let mut data = read_file();
    let task = vec![
        read_input_string("Task name?").unwrap(),
        read_input_string("Task description?").unwrap(),
    ];

    data.push(task.join("$"));

    write_file(data);
    Ok(())
}

/// Read a line from input, truncating the trailing newline.
fn read_input_string(prompt: &str) -> IoResult<String> {
    println!("{}", prompt);
    let mut buffer = String::new();
    let stdin = stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim_end().into())
}

fn write_file(data: Vec<String>) {
    fs::write("data.txt", data.join("\n")).unwrap();
}

fn read_file() -> Vec<String> {
    let mut out = Vec::new();
    let t = fs::read_to_string("data.txt");
    match t {
        Ok(s) => {
            if !s.is_empty() {
                out.extend(s.split("\n").map(str::to_string));
            }
        }
        Err(_) => {
            fs::OpenOptions::new().write(true).create_new(true).open("data.txt").unwrap();
        }
    }

    out
}

/// Print the tasks
#[pyfunction]
pub fn print_tasks() -> PyResult<()> {
    let data = read_file();
    println!("{:<20} {:<20}", "Name", "Description");
    for task in &data {
        let fields: Vec<_> = task.split("$").collect();
        println!("{:<20} {:<20}", fields[0], fields[1]);
    }
    Ok(())
}

/// Modify a task
#[pyfunction]
pub fn modify_task() -> PyResult<()> {
    let mut data = read_file();

    print_tasks().unwrap();
    println!();

    let to_modify = read_input_string("What task to modify?").unwrap();

    let new_name = read_input_string("New name?").unwrap();
    let new_desc = read_input_string("New description?").unwrap();

    let i = data.iter().position(|t| t.split("$").next().unwrap() == to_modify).unwrap();

    data[i] = format!("{}${}", new_name, new_desc);

    write_file(data);

    Ok(())
}

/// Delete a task
#[pyfunction]
pub fn delete_task() -> PyResult<()> {
    let mut data = read_file();

    print_tasks().unwrap();
    println!();

    let to_delete = read_input_string("What task to delete?").unwrap();

    let i = data.iter().position(|t| t.split("$").next().unwrap() == to_delete).unwrap();

    data.remove(i);

    write_file(data);

    Ok(())
}

/// Helper functions for todo_list.py
#[pymodule]
fn tdl_helper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_task, m)?)?;
    m.add_function(wrap_pyfunction!(print_tasks, m)?)?;
    m.add_function(wrap_pyfunction!(modify_task, m)?)?;
    m.add_function(wrap_pyfunction!(delete_task, m)?)?;
    Ok(())
}

