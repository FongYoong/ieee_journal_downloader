use std::io::Write;
use std::fs::OpenOptions;
use console::Term;
use console::style as TermStyle;

pub fn print_divider(term: &Term) {
    let (_, term_max_columns) = term.size();
    println!("{}", TermStyle("-".repeat(term_max_columns as usize)).bright());
}

pub fn write_to_file(file_path: &str, content: &str) {
    let path = std::path::Path::new(file_path);
    let parent_path = path.parent().unwrap();
    std::fs::create_dir_all(parent_path).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).truncate(true)
    .open(file_path)
    .unwrap();
    format!("{}", content);
    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn append_to_file(file_path: &str, content_to_append: &str) {
    let mut file = OpenOptions::new().create(true).write(true)
    .append(true)
    .open(file_path)
    .unwrap();

    if let Err(e) = writeln!(file, "{}", content_to_append) {
        eprintln!("Couldn't append to file: {}", e);
    }
}