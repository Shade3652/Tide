//use std::collections::HashMap;


pub fn parse(text: &String) -> (Vec<Vec<Token>>, Vec<AST>, Vec<PErr>, Vec<Vec<i64>>, Vec<Vec<Token>>, Vec<Vec<i64>>, Vec<i64>) {

    let mut string: String = String::from("");  //Number vars
    let mut num: String = String::from("");
    let mut num_point: bool  = false;
    let mut d_num_point: bool = false;
    let mut s_point_char: i64 = 0;

    let mut dquote: bool = false;
    let mut squote: bool = false;

    let mut esc_char_last = false;
    let mut newline_in_string = false;

    let keywords: String = "let true false if".to_string();

    let mut tokens: Vec<Token> = Vec::new();    //Token vars
    let mut asts: Vec<AST> = Vec::new();
    let mut lines: Vec<Vec<Token>> = Vec::new();

    let mut lbraces: Vec<Lstore> = Vec::new();   //Brace vars
    let mut brace_sets: Vec<ParPairs> = Vec::new();

    let mut lpars: Vec<Lstore> = Vec::new();     //Paren vars
    let mut paren_sets: Vec<ParPairs> = Vec::new();

    let mut lbrackets: Vec<Lstore> = Vec::new(); //Bracket vars
    let mut bracket_sets: Vec<ParPairs> = Vec::new();

    let mut newlines: Vec<i64> = Vec::new();
    let mut errors: Vec<PErr> = Vec::new();

    //static DOT: LazyLock<String> = LazyLock::new(|| String::from(".")); //OLD CODE FOR OLD DOT

    let mut char_num: i64 = 0;

    for char in text.chars() {

        if char == '\n' {
            newlines.push(char_num.clone());
        }


        if ("1234567890".contains(char) && string.len() == 0) || char == '.'{   //Checks to see if the number being parsed has 2 decimal points

            if num_point && char == '.' {

                d_num_point = true;
                s_point_char = char_num;
            } 


            else {  //Otherwise...

                if char == '.' {    //Corrects number types
                    if num == "" {
                        tokens.push(Token {token_type: "DOT".to_string(), value: ".".to_string(), start: char_num.clone()});
                    }

                else {
                        num_point = true;
                    }
                }
            }

            num.push(char);
        
        }

        else {
            if "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm".contains(char) {
                string = string + (num.clone()).as_str();
                d_num_point = false;
                num_point = false;
                num = String::from("");
            }

            if !(num == "") {
                if d_num_point {
                    errors.push(PErr{error:0, char: s_point_char});    //ERROR
                    break;
                }
                if num_point {
                    tokens.push(Token {token_type: "FLOAT".to_string(), value: num.clone(), start: (char_num - string.len() as i64)});
                }

                else {
                    tokens.push(Token {token_type: "INT".to_string(), value: num.clone(), start: (char_num - string.len() as i64)});
                }
                num_point = false;

                num = String::from("");
            }
        }


        if ("QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm".contains(char) || ("1234567890".contains(char) && string.len() != 0)) && !dquote && !squote {
            string.push(char);
        }

        else {
            if !(string == "") && !dquote && !squote {

            if keywords.contains(&string) {

                tokens.push(Token {token_type: "KEYWORD".to_string(), value: string.clone(), start: (char_num - string.len() as i64)}); //Puts the keyword in the tokens list
            }

            else {

                    if string.len() == 1 {
                        tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone(),start: (char_num - string.len() as i64)});
                    }
                    else {
                        tokens.push(Token {token_type: "CHARSTR".to_string(), value: string.clone(),start: (char_num - string.len() as i64)});
                    }
                }
                string = "".to_string();
            }
        }



        if char == '+' {
            tokens.push(Token {token_type: "PLUS".to_string(), value: "+".to_string(), start: char_num.clone()});
        }

        if char == '-' {
            tokens.push(Token {token_type: "MINUS".to_string(), value: "-".to_string(), start: char_num.clone()});
        }

        if char == '*' {
            tokens.push(Token {token_type: "MUL".to_string(), value: "*".to_string(), start: char_num.clone()});
        }

        if char == '%' {
            tokens.push(Token {token_type: "MOD".to_string(), value: "%".to_string(), start: char_num.clone()});
        }

        if char == '/' {
            if tokens[tokens.len() - 1].token_type == "DIV" {
                tokens.pop();
                tokens.push(Token {token_type: "DODIV".to_string(), value: "//".to_string(), start: char_num.clone() - 1});
            }
            else {
                tokens.push(Token {token_type: "DIV".to_string(), value: "/".to_string(), start: char_num.clone()});
            }
        }

        if char == '^' {
            tokens.push(Token {token_type: "TTPO".to_string(), value: "^".to_string(), start: char_num.clone()});
        }

        if char == '\\' {
            tokens.push(Token {token_type: "BSLASH".to_string(), value: "\\".to_string(), start: char_num.clone()});
        }

        if char == '(' && !dquote && !squote {
            tokens.push(Token {token_type: "LPAR".to_string(), value: "(".to_string(), start: char_num.clone()});
            lpars.push(Lstore{par: tokens.len() - 1, char: char_num});

        }


        if char == ')' && !dquote && !squote {
            tokens.push(Token {token_type: "RPAR".to_string(), value: ")".to_string(), start: char_num.clone()});


            if lpars.len() == 0 {
                errors.push(PErr{error:1, char: char_num});    //ERROR
                break;
            }


            paren_sets.push(ParPairs{l: lpars[lpars.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lpars.pop();

            
            //P2: Adding a AST object
            let to_be_added: Vec<Token> = tokens[paren_sets[paren_sets.len() - 1].l + 1.. paren_sets[paren_sets.len() - 1].r].to_vec();

                for _i in &to_be_added {
                    tokens.remove(paren_sets[paren_sets.len() - 1].l + 1);
                }

                asts.push(AST {children: to_be_added, ast_type: "AST".to_string()});
                tokens.pop(); tokens.pop();

                
                let mut temp_token = Token{token_type: "NULL".to_string(), value: "NULL".to_string(), start: 0};

                if tokens.len() > 3 {
                    temp_token = tokens[tokens.len() - 2].clone();
                }
                
                let mut possible_keyword = "NULL".to_string();

                if tokens.len() >= 2 {
                    possible_keyword = tokens[tokens.len() - 2].token_type.clone();
                }

                if tokens[tokens.len() - 1].token_type == "CHARSTR" && !(keywords.contains(&temp_token.value)) && !(possible_keyword == "KEYWORD"){

                    let mut temp_token = tokens.pop().unwrap();
                    temp_token.token_type = "FUNC_CALL".to_string();

                    tokens.push(temp_token);
   
                    if asts[asts.len() - 1].children.len() > 0 {
                        tokens.push(Token {token_type: "ARGS".to_string(), value: (asts.len() - 1).to_string(), start: asts[asts.len() - 1].children[0].start});

                    }
                    else {
                        tokens.push(Token {token_type: "ARGS".to_string(), value: (asts.len() - 1).to_string(), start: tokens[tokens.len() - 1].start + (tokens[tokens.len() - 1].value).len() as i64});
                    }
                }

                else {
                    tokens.push(Token {token_type: "AST".to_string(), value: (asts.len() - 1).to_string(), start: asts[asts.len() - 1].children[0].start});
                }


                
                
                
            }

        if char == ':' {
            tokens.push(Token {token_type: "COLON".to_string(), value: ":".to_string(), start: char_num.clone()});
        }

        if char == ';' {
            tokens.push(Token {token_type: "SEMICOLON".to_string(), value: ";".to_string(), start: char_num.clone()});
        }

        if char == '&' {
            if tokens[tokens.len() - 1].token_type == "APERSAND" {
                tokens.pop();
                tokens.push(Token {token_type: "AND".to_string(), value: "&&".to_string(), start: char_num.clone() - 1});
            }
            else {
                tokens.push(Token {token_type: "APERSAND".to_string(), value: "&".to_string(), start: char_num.clone()});
            }
        }

        if char == '|' {
            if tokens[tokens.len() - 1].token_type == "LINE" {
                tokens.pop();
                tokens.push(Token {token_type: "OR".to_string(), value: "||".to_string(), start: char_num.clone() - 1});
            }
            else {
                tokens.push(Token {token_type: "LINE".to_string(), value: "|".to_string(), start: char_num.clone() - 1});
            }
        }

        if char == '!' {
            tokens.push(Token {token_type: "NOT".to_string(), value: "!".to_string(), start: char_num.clone()});
        }

        if char == '>' {
            tokens.push(Token {token_type: "GREATER".to_string(), value: ">".to_string(), start: char_num.clone()});
        }

        if char == '<' {
            tokens.push(Token {token_type: "LESS".to_string(), value: "<".to_string(), start: char_num.clone()});
        }

        if char == ',' {
            tokens.push(Token {token_type: "COMMA".to_string(), value: ",".to_string(), start: char_num.clone()});
        }

        if char == '=' {
            if tokens[tokens.len() - 1].token_type == "EQUAL" {
                tokens.pop();
                tokens.push(Token {token_type: "DEQUAL".to_string(), value: "==".to_string(), start: char_num.clone() - 1});
            }
            if tokens[tokens.len() - 1].token_type == "NOT" {
                tokens.pop();
                tokens.push(Token {token_type: "NEQUAL".to_string(), value: "!=".to_string(), start: char_num.clone() - 1});
            }
            else {
                tokens.push(Token {token_type: "EQUAL".to_string(), value: "=".to_string(), start: char_num.clone()});
            }
        }

        if char == '{' && !dquote && !squote {
            tokens.push(Token {token_type: "LBRACE".to_string(), value: "{".to_string(), start: char_num.clone()});
            lbraces.push(Lstore{par: tokens.len() - 1, char: char_num});
        }


        if char == '}' && !dquote && !squote {
            tokens.push(Token {token_type: "RBRACE".to_string(), value: "}".to_string(), start: char_num.clone()});


            if lbraces.len() == 0 {     //Checks to see if there are any left braces to match the right brace
                errors.push(PErr{error:2, char: char_num});    //ERROR
                break;
            }

            brace_sets.push(ParPairs{l: lbraces[lbraces.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lbraces.pop();



            //P2: Adding a AST object
            let temp: Vec<Token> = tokens[brace_sets[brace_sets.len() - 1].l + 1.. brace_sets[brace_sets.len() - 1].r].to_vec();
            for _i in &temp {
                tokens.remove(brace_sets[brace_sets.len() - 1].l + 1);
            }

            if temp.len() == 0 {
                tokens.pop();
                tokens.pop();
                tokens.push(Token {token_type: "EBRACES".to_string(), value: "{}".to_string(), start: char_num.clone() - 1});
            }
            
            else {
                let temp_len: usize = temp.len();
                tokens.pop(); tokens.pop();
                asts.push(AST {children: temp, ast_type: "SCOPE".to_string()});
                tokens.push(Token {token_type: "SCOPE".to_string(), value: (asts.len() - 1).to_string(), start: char_num.clone() - temp_len as i64});
            }
    
        }


        if char == '[' && !dquote && !squote {
            tokens.push(Token {token_type: "LBRACKET".to_string(), value: "[".to_string(), start: char_num.clone()});
            lbrackets.push(Lstore {par: tokens.len() - 1, char: char_num});
        }


        if char == ']' && !dquote && !squote {
            tokens.push(Token {token_type: "RBRACKET".to_string(), value: "]".to_string(), start: char_num.clone() - 1});

            if lbrackets.len() == 0 {
                errors.push(PErr{error:3, char: char_num});    //ERROR
                break;
            }

            bracket_sets.push(ParPairs{l: lbrackets[lbrackets.len() - 1].par, r: (tokens.len() - 1).try_into().unwrap()});
            lbrackets.pop();


             //P2: Adding a AST object
             let temp: Vec<Token> = tokens[bracket_sets[bracket_sets.len() - 1].l + 1.. bracket_sets[bracket_sets.len() - 1].r].to_vec();
             for _i in &temp {
                 tokens.remove(bracket_sets[bracket_sets.len() - 1].l + 1);
             }

             if temp.len() == 0 {
                    tokens.pop();
                    tokens.pop();
                    tokens.push(Token {token_type: "EBRACKETS".to_string(), value: "[]".to_string(), start: char_num.clone() - 1});
             }
             
             
             else {
                let temp_len: usize = temp.len();
                tokens.pop(); tokens.pop();
                asts.push(AST {children: temp, ast_type: "LIST".to_string()});
                tokens.push(Token {token_type: "LIST".to_string(), value: (asts.len() - 1).to_string(), start: char_num.clone() - temp_len as i64});
            }

        }


        if char == '"' && !squote {     //Makes sure that the character is not a part of a string

            if tokens[tokens.len() - 1].token_type == "BSLASH" {     //Makes sure that if an escaoed character is used, it is added to the current string instead of being treated as a quote
                tokens.pop();
                tokens.push(Token {token_type: "CHAR".to_string(), value: '"'.to_string(), start: char_num.clone() - 1});
            }

            else {
                tokens.push(Token {token_type: "DQUOTE".to_string(), value: '"'.to_string(), start: char_num.clone()});

                if dquote {     //Preps the string to be added to the tokens
                    tokens.pop(); tokens.pop();     //Removes the quotes from the tokens list bc they are not needed

                    string.push(char);

                    if (string.clone().get(1..(string.len() - 1)).unwrap().to_string()).len() == 1 && !newline_in_string{    //Check to see if the string is a char

                        errors.push(PErr{error:8, char: char_num - 1});    //ERROR
                    }

                    let string_len: usize = string.len();


                    tokens.push(Token {token_type: "STRING".to_string(), value: string[1..string_len - 1].to_string(), start: char_num.clone() - string_len as i64});
                    string = String::from("");
                    dquote = false;
                    newline_in_string = false;
                }

                else {
                    dquote = true;       //Starts the adding process
                }
            }
        }


        if char == '\'' && !dquote{     //Makes sure that the character is not a part of a string

            if tokens[tokens.len() - 1].token_type == "BSLASH" {    //Makes sure that if an escape character is used, it is added to the current char instead of being treated as a quote
                tokens.pop();
                tokens.push(Token {token_type: "CHAR".to_string(), value: '\''.to_string(), start: char_num.clone() - 1});
            }

            else {
                tokens.push(Token {token_type: "SQUOTE".to_string(), value: '\''.to_string(), start: char_num.clone()});

                if squote {     //Preps the string to be added to the tokens

                    if string.len() > 2 {       //Check to see if the char is a string
                       errors.push(PErr{error:7, char: char_num - 1});    //ERROR
                        break;
                    }

                    tokens.pop(); tokens.pop();
                    
                    string.push(char);
                    tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone().chars().nth(1).unwrap().to_string(), start: char_num.clone() - 1});
                    string = String::from("");
                    squote = false;
                }

                else {
                    squote = true;      //Starts the adding process
                }
            }
        }


        if dquote {     //"" Work to add it to the current string

        if esc_char_last {

            if char == 'n' {            
                string = string[0..string.len() - 1].to_string();

                string = string + "\n";

                newline_in_string = true;

            }
        }

            if tokens[tokens.len() - 1].token_type != "DQUOTE" && !("()[]{}".contains(&tokens[tokens.len() - 1].token_type)) {
                tokens.pop();    //WAY easier than then stoping anything from adding to the tokens list
            }

            if !esc_char_last {
                string.push(char);
            }

            if char == '\\' {
                esc_char_last = true;
            }
    
            else {
                esc_char_last = false;
            }
        }

        if squote {     //Same but with ''

            if tokens[tokens.len() - 1].token_type != "SQUOTE" && !("()[]{}".contains(&tokens[tokens.len() - 1].token_type)) {
                tokens.pop();   //WAY easier than then stoping anything from adding to the tokens list
            }

            string.push(char);
        }


        char_num += 1;
    }




    if !(num == "") {   //Last checks in case the last character was a number (It't wont add otherwise because it would need another cycle)
        if num_point {
            tokens.push(Token {token_type: "FLOAT".to_string(), value: num.clone(), start: char_num - num.len() as i64});
        }
        else {
            tokens.push(Token {token_type: "INT".to_string(), value: num.clone(), start: char_num - num.len() as i64});
        }
    }

    if !(string == "") {    //Same but with letters

        if keywords.contains(&string) {
            tokens.push(Token {token_type: "KEYWORD".to_string(), value: string.clone(), start: char_num - string.len() as i64});
        }

        else {
            if string.len() == 1 {
                tokens.push(Token {token_type: "CHAR".to_string(), value: string.clone(), start: char_num - 1 as i64});
            }
            else {
                tokens.push(Token {token_type: "CHARSTR".to_string(), value: string.clone(), start: char_num - string.len() as i64});
            }
        }
        string = String::from("");
    }


    if dquote {

        let mut reversed_string = String::new(); // Create a new string to store the reversed string

        for c in text.chars().rev() {
            reversed_string.push(c); // Append each character to the reversed string
        }

        let index = reversed_string.char_indices().position(|(_, c)| c == '"');

        if let Some(i) = index {
            errors.push(PErr{error:10, char: (text.len() - 1 - i) as i64});    //ERROR
        } 
        
        else {
            errors.push(PErr{error:10, char: 1});    //ERROR
        }
    }


    if squote {

        let mut reversed_string = String::new(); // Create a new string to store the reversed string

        for c in text.chars().rev() {
            reversed_string.push(c); // Append each character to the reversed string
        }

        let index = reversed_string.char_indices().position(|(_, c)| c == '\'');

        if let Some(i) = index {
            errors.push(PErr{error:9, char: (text.len() - 1 - i) as i64});    //ERROR
        } 
        
        else {
            errors.push(PErr{error:9, char: 1});    //ERROR
        }
    }


    /* let mut count: i8 = 0;

    for i in &tokens {
        println!("Token: {} | Value: {} ({})", i.token_type, i.value, count);
        count += 1;
    }
    println!("______________");
    for i in paren_sets {
        println!("L: {} | R: {}", i.l, i.r);
    }

    for i in &asts {
        println!("______________");
        for j in &i.children {
            println!("Token: {} | Value: {}", j.token_type, j.value);
        }
    } */


    if lpars.len() != 0 {
        errors.push(PErr{error:4, char: lpars[0].char as i64});    //ERROR
    }

    if lbraces.len() != 0 {
        errors.push(PErr{error:5, char: lbraces[0].char as i64});    //ERROR
    }
    
    if lbrackets.len() != 0 {
        errors.push(PErr{error:6, char: lbrackets[0].char as i64});    //ERROR
    }

    //println!("{}", asts.len());
    let mut temp_line: Vec<Token> = Vec::new();
    let mut cur_line_asts: Vec<i64> = Vec::new();
    let mut line_asts: Vec<Vec<i64>> = Vec::new();
    let mut semicolon: bool = false;
    let mut scopes: Vec<Vec<Token> > = Vec::new();
    let mut scope_asts: Vec<Vec<i64>> = Vec::new();



    for i in tokens.clone() {

        if i.token_type == "SEMICOLON" {

            semicolon = true;

            lines.push(temp_line);

            line_asts.push(cur_line_asts);
            temp_line = Vec::new();
            cur_line_asts = Vec::new();
        }

        else {

            if i.token_type == String::from("AST") || i.token_type == String::from("ARGS") {
                cur_line_asts.push((i.value.clone()).parse::<i64>().unwrap());
            }

            semicolon = false;
            temp_line.push(i.clone());

        }

    }

    if !semicolon{
        errors.push(PErr{error:11, char: text.len() as i64});    //ERROR
    }


    temp_line = Vec::new();
    cur_line_asts = Vec::new();


    for i in &asts{

        if i.ast_type == "SCOPE" {

            for k in i.children.clone() {

                if k.token_type == "SEMICOLON" {

                    semicolon = true;

                    scopes.push(temp_line);

                    scope_asts.push(cur_line_asts);
                    temp_line = Vec::new();
                    cur_line_asts = Vec::new();
                }

                else {

                    if k.token_type == String::from("AST") || k.token_type == String::from("ARGS") {
                        cur_line_asts.push((k.value.clone()).parse::<i64>().unwrap());
                    }

                    semicolon = false;
                    temp_line.push(k.clone());

                }

            }
        }
    }

    if !semicolon{
        errors.push(PErr{error:12, char: text.len() as i64});    //ERROR
    }

    return (lines, asts, errors, line_asts, scopes, scope_asts, newlines);

}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
    pub start: i64,
}

#[derive(Clone, Debug)]
pub struct AST {
    pub children: Vec<Token>,
    pub ast_type: String,
}

pub struct PErr {
    pub error: i8,
    pub char: i64,
}

struct Lstore {
    par: usize,
    char: i64,
}

struct ParPairs {
    l: usize,
    r: usize,
}