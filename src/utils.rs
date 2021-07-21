use console::Term;
use console::style as TermStyle;

pub fn print_divider(term: &Term) {
    let (_, term_max_columns) = term.size();
    println!("{}", TermStyle("-".repeat(term_max_columns as usize)).bright());
}