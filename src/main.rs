use crate::basic::{control_flow, println_int, statement_expression, sum, variables};
/// rust中的包(crate)代表了一系列源代码文件的集合
use crate::guessing_game::guessing_number;
use crate::ownership::variables_scope;
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
}
