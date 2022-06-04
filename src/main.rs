use std::{env, fs, process, thread};
use std::error::Error;
use std::sync::mpsc;

use course::basic::{control_flow, println_int, statement_expression, sum, variables};
use course::common_collections::{hashmap, string, vector};
use course::enums_pattern_matching::{enums, pattern_match};
use course::error_handling::error_handling;
use crate::example::random_access_source::{FileRandomAccessSouce, RandomAccessSource};
use course::generic_type::generic_type;
use course::guessing_game::guess_game::guessing_number;
use course::lifetimes::lifetimes;
/// rust中的包(crate)代表了一系列源代码文件的集合
use course::ownership::variables_scope;
use course::references_borrowing::references_borrowing;
use course::slice_type::slice_type;
use course::struct_example::calculate_area;
use course::structs::{init_user, tuple_struct, user_demo};
use crate::course::r#trait::trait_practice;
use crate::search_text::search_text::{Config, run};

mod example;
mod course;
mod search_text;
mod directory_size;

fn basic_program() {
    //guessing_number();

    variables();
    println_int(1234);
    let se = statement_expression();
    println!("The sum of this variables is {}", sum(160, 120));

    control_flow();

    variables_scope();
    references_borrowing();
    slice_type();

    user_demo();
    tuple_struct();

    calculate_area();

    enums();
    pattern_match();

    vector();
    string();
    hashmap();

    error_handling();

    generic_type();
    trait_practice();

    lifetimes();
}

fn search_test() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Proble parsing arguments: {}.", error);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}.", err);
        process::exit(1);
    }
}

fn main() {}