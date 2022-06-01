use crate::basic::{control_flow, println_int, statement_expression, sum, variables};
use crate::common_collections::{hashmap, string, vector};
use crate::enums_pattern_matching::{enums, pattern_match};
use crate::error_handling::error_handling;
use crate::generic_type::generic_type;
use crate::guessing_game::guess_game::guessing_number;
use crate::lifetimes::lifetimes;
/// rust中的包(crate)代表了一系列源代码文件的集合
use crate::ownership::variables_scope;
use crate::r#trait::trait_practice;
use crate::references_borrowing::references_borrowing;
use crate::slice_type::slice_type;
use crate::struct_example::calculate_area;
use crate::structs::{init_user, tuple_struct, user_demo};

mod guessing_game;
mod basic;
mod ownership;
mod references_borrowing;
mod slice_type;
mod structs;
mod struct_example;
mod enums_pattern_matching;
mod common_collections;
mod error_handling;
mod generic_type;
mod r#trait;
mod lifetimes;

fn main() {

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
