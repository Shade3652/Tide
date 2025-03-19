use convert_case::{Case, Casing};
use crate::parser::{Token, AST};
use crate::VAR;

pub fn keyword_execute() {

}

pub fn var_check(mut variables: Vec<VAR>, line: &Vec<Token>, asts: &Vec<AST>, line_asts: &Vec<i64>, depth: &i64) -> Vec<VAR> {

    if line[0].token_type == "KEYWORD" && line.len() == 5 && line[0].value == "let"{

        if true {

            if line[1].token_type == "CHARSTR" && line[2].token_type == "AST" && line[3].token_type == "EQUAL" {

                if line[4].token_type.to_case(Case::Lower) == asts[line_asts[0].clone() as usize].children[0].value {
                    let var = VAR {
                        var_type: asts[line[2].value.parse::<usize>().unwrap_or_else(|_| panic!("This is a VERY weird error that shouldn't happen. Congrats for finding it! Please open a github issue with your code attatched telling me that you got this error."))].children[0].value.clone(),
                        name: line[1].value.clone(),
                        value: line[4].value.clone(),
                        var_depth: depth.clone(),
                    }; 

                    variables.push(var);

                    //println!("{} : {} : {}", variables[0].var_type, variables[0].name, variables[0].value);

                }

                else {
                    //ERROR: Variable type does not match the type of the value
                }
            }

            else {
                //One of the things that the let keyword requires is missing
            }
        }

        else {

        }
    }

    return variables;
}