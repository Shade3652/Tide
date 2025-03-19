use crate::ERROR;
use crate::parser::Token;

fn raise(error: String, line: Vec<Token>, line_number: i64, ch: i64, args: Vec<&str>, line: Vec<Token>) -> Vec<ERROR> {
    if error = "Argument Error" {
        [println!("Argument error at line {}:\n{} argument(s) expected, {} were given", line_number, args[0], args[1])];
    }

}