mod parser;
use std::fs;
use serde_json::Value;
use std::env;
use colored::Colorize;
use convert_case::{Case, Casing};

#[path = "lang/function_find.rs"]
mod function_find;

#[path = "lang/keywords.rs"]
mod keywords;




fn main() {
    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //let line: String = String::from(" L bozo (3 / (45 * 678)) - 9.0 + 12.3 //[skib && 69] 7 sigma \" lol + sussy\" {what 3 || 3.14} () [] {} eee3 420.69 69.420.gg sussy\\\" \" fellas in paris // 3.14\" 's' \"'k\" '\"';");
    let line: String = fs::read_to_string(current_path.to_string() + "/src/testing.tde").expect("Couldn't find or load that file.");
    let parsed: (Vec<Vec<parser::Token>>, Vec<parser::AST>, Vec<parser::PErr>, Vec<Vec<i64>>, Vec<Vec<parser::Token>>, Vec<Vec<i64>>, Vec<i64>)= parser::parse(&line);

    let mut variables: Vec<VAR> = Vec::new();

    let lines: Vec<Vec<parser::Token>> = parsed.0;  //Basic Lines in no scopes
    let mut asts: Vec<parser::AST> = parsed.1;  //Groups of anything in parentheses, brackets, or braces
    let errors: Vec<parser::PErr> = parsed.2;   //Any Errors from the parser
    let line_asts: Vec<Vec<i64>> = parsed.3;    //A list of indexes of the ASTs that are in each line
    let scopes: Vec<Vec<parser::Token>> = parsed.4;     //Lines in braces
    let scope_line_asts: Vec<Vec<i64>> = parsed.5;      //Line ASTS but for the scopes
    let newlines: Vec<i64> = parsed.6;   //Indexes of newlines

    

    if errors.len() != 0 {

        let parser_errors = fs::read_to_string(current_path.to_string() + "/src/Errors/Parsing.json").expect("Couldn't find or load errors file.");
        let parser_errors: Value = serde_json::from_str(&parser_errors).expect("Couldn't parse the errors file.");

        println!("{} at charter {}", parser_errors[errors[0].error.to_string()]["message"].as_str().unwrap().to_ascii_uppercase().red().bold(), errors[0].char);

        println!("{}", line);

        for _i in 0..errors[0].char {

            print!("{}", line.chars().nth(_i as usize).unwrap_or_default())
        }

        println!("{}", "^ Here".yellow().bold());

        return;
    }
    

    for (line_num, i) in lines.clone().into_iter().enumerate() {

        println!("{}", (line_num as i64).to_string().blue());
        println!("{:?}", line_asts[line_num]);

        for j in i {
            println!("{} : {}", j.token_type, j.value);
        }
            
    }

    println!("{}", "___________________________");

    variables = execute(asts.clone(), line_asts[0].clone(), lines[0].clone(), variables, 0);    //TO-DO: Replace 0 With the Line #

    variables = execute(asts, line_asts[1].clone(), lines[1].clone(), variables, 0);    //TO-DO: Replace 0 With the Line #
}


fn execute(mut asts: Vec<parser::AST>, line_asts: Vec<i64>, mut line: Vec<parser::Token>, mut variables: Vec<VAR>, depth: i64) -> Vec<VAR> {
    
    for (token_num, token) in line.clone().into_iter().enumerate() {
        for i in &variables {
            if token.value == i.name {
                line[token_num].value = i.value.clone();
            }
        }
    }

    for i in &line_asts {
        let solved = solve_ast(asts[*i as usize].clone());

        asts[*i as usize] = solved;
    }

    variables = keywords::var_check(variables, &line, &asts, &line_asts, &depth);

    return variables;
    
}


fn solve_ast(mut ast: parser::AST) -> parser::AST {

    let solved_ast: parser::AST;

    //for mut ast in to_solve {
        for (token_num, token) in ast.children.clone().iter().enumerate() {
            match token.token_type.as_str() {

                "PLUS" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {

                        if ast.children[token_num - 1].token_type == "STRING" {
                            ast.children[token_num].value = format!("{}{}", ast.children[token_num - 1].value, ast.children[token_num + 1].value);

                            ast.children.remove(token_num + 1);
                            ast.children.remove(token_num - 1);
                        }


                        else {
                            if ast.children[token_num + 1].token_type == "INT" || ast.children[token_num + 1].token_type == "FLOAT" {
                                
                                let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                                    // Handle the error here, e.g., return a default value or log the error
                                    println!("Hey, That's not a number!");
                                    0.0
                                });

                                let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                                    // Handle the error here, e.g., return a default value or log the error
                                    println!("Hey, That's not a number!");
                                    0.0
                                });

                                ast.children[token_num].value = (left_value + right_value).to_string();
                                
                                if  ast.children[token_num].value.contains(".") {
                                    ast.children[token_num].token_type = "FLOAT".to_string();
                                }
                                else {
                                    ast.children[token_num].token_type = "INT".to_string();
                                }

                                ast.children.remove(token_num + 1);
                                ast.children.remove(token_num - 1);
                            }
                        }
                    }
                }

                "MINUS" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });
                        
                        ast.children[token_num].value = (left_value - right_value).to_string();
                        
                        if  ast.children[token_num].value.contains(".") {
                            ast.children[token_num].token_type = "FLOAT".to_string();
                        }
                        else {
                            ast.children[token_num].token_type = "INT".to_string();
                        }

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                "MUL" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });
                        
                        ast.children[token_num].value = (left_value * right_value).to_string();
                        
                        if  ast.children[token_num].value.contains(".") {
                            ast.children[token_num].token_type = "FLOAT".to_string();
                        }
                        else {
                            ast.children[token_num].token_type = "INT".to_string();
                        }

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                "DIV" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });
                        
                        ast.children[token_num].value = (left_value / right_value).to_string();
                        
                        if  ast.children[token_num].value.contains(".") {
                            ast.children[token_num].token_type = "FLOAT".to_string();
                        }
                        else {
                            ast.children[token_num].token_type = "INT".to_string();
                        }

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                "FDIV" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });
                        
                        ast.children[token_num].value = ((left_value / right_value) as i64).to_string();
                        
                        if  ast.children[token_num].value.contains(".") {
                            ast.children[token_num].token_type = "FLOAT".to_string();
                        }
                        else {
                            ast.children[token_num].token_type = "INT".to_string();
                        }

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                "MOD" => {

                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });
                        
                        ast.children[token_num].value = (left_value % right_value as f64).to_string();
                        
                        ast.children[token_num].token_type = "INT".to_string();

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                "TTPO" => {
                    if token_num == 0 || token_num == ast.children.len() {
                        //TO-DO: add Error: Expected a number or variable before and after the operator
                    }

                    else {
                        let left_value: f64 = ast.children[token_num - 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        let right_value: f64 = ast.children[token_num + 1].value.parse::<f64>().unwrap_or_else(|_| {
                            // Handle the error here, e.g., return a default value or log the error
                            println!("Hey, That's not a number!");
                            0.0
                        });

                        ast.children[token_num].value = (left_value.powf(right_value)).to_string();

                        if  ast.children[token_num].value.contains(".") {
                            ast.children[token_num].token_type = "FLOAT".to_string();
                        }
                        else {
                            ast.children[token_num].token_type = "INT".to_string();
                        }

                        ast.children.remove(token_num + 1);
                        ast.children.remove(token_num - 1);
                    }
                }

                _ => {
                    //do nothing
                }
            }
        }
        solved_ast = ast;
    //}
    
    
    return solved_ast;
}


struct VAR {
    var_type: String,
    name: String,
    value: String,
    var_depth: i64,
}

struct ERROR {
    error: String,
    char: i64,
    line: i64,
    args: Vec<String>,
}