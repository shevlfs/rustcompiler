// #[macro_use] extern crate text_io;
// use std::io::Read;
// use std::io::{self, BufRead};
extern crate prev_iter;
// fn main() {
//     let input: String = read!("{}\n");
//     println!("{}", input);
//     let linesplit = tokenize(input);
//     for cmd in linesplit.iter(){
//         for token in cmd.iter(){
//             println!("{:?}", token);
//         }
//     }
// }



pub fn tokenize(content: String) -> Vec<Vec<Token>> {
    let mut code: Vec<Vec<Token>> = vec![vec![]];
    let mut curword = Vec::new();
    let linesplit: Vec<String> = content.split(";").map(|s| s.to_string()).collect();
    for line in linesplit.iter(){
        // if line.contains("print") && line.chars().nth(0) == 'p'{
        //     code.push(vec![Token::keyword("print".to_string())]);
        //     code.last().clone().unwrap().push(line.trim_start_matches("print").trim_matches('"').to_string());
        // }
        let mut varletfound = false; 
        if line.trim().starts_with("var"){
            varletfound = true;
        } else if line.trim().starts_with("let"){
            varletfound = true;
        }
        if !varletfound{
            for char in line.trim().chars(){
                if char != ' ' && char != '('{
                    curword.push(char.clone());
                } else {
                    break
                }
            }
            let word: String = curword.clone().into_iter().collect();
        // println!("word - {}", word);
        match word {
            // print tokenization
            word if word == "print" => {
                let mut arg = String::new();
                curword = Vec::new();
                code.push(vec![Token::Keyword("print".to_string())]);
                let mut argfound = false;
                //code.last().clone().unwrap().push(line.trim_start_matches("print").trim_matches('"').to_string());
                let mut cc = 5; 
                for char in line.trim().chars(){
                    if cc != 0{
                        cc = cc - 1;
                    } else if char == ' ' && !argfound{
                        continue
                    } else if char == '(' && !argfound{
                        cc = cc + 1;
                        argfound = true
                    }else if char != '"' && argfound{
                        curword.push(char.clone());
                    } else if char == '"'{
                        break
                    }
                }
                arg = curword.clone().into_iter().collect();
                code.last_mut().unwrap().push(Token::Word(arg));
                curword = Vec::new();
            },
            _ => break,
        }
        } else {
            let newcmd = remove_whitespace(line.as_str());
            let mut fnmfound = false;
            let mut fnm = String::new(); 
            let mut cc = 3;
            let mut opscount = newcmd.matches("+").count() + newcmd.matches("-").count() + newcmd.matches("*").count() + newcmd.matches("/").count();
            let mut lastvar = false;
            if newcmd.starts_with("let"){
                code.push(vec![Token::Keyword("let".to_string())]);
            } else if newcmd.starts_with("var"){
                code.push(vec![Token::Keyword("var".to_string())]);
            } else {
                panic!();
            }
            for char in newcmd.chars(){
                    if lastvar {
                        curword.push(char);
                        continue
                    }
                    if cc != 0 && !fnmfound{
                        cc = cc - 1;
                    } else if char != '=' && char != '*' && char != '+' && char != '-' && char != '/' && !fnmfound{
                        curword.push(dbg!(char));

                    } else if char == '=' && !fnmfound{ 
                        fnmfound = true;
                        fnm = curword.clone().into_iter().collect();
                        code.last_mut().unwrap().push(Token::VarName(fnm));
                        code.last_mut().unwrap().push(Token::Operator('='));
                        curword.clear();
                        let mut fnm = String::new();
                        fnmfound = false;
                    } else if char == '+' && !fnmfound{
                        fnmfound = true;
                        fnm = curword.clone().into_iter().collect();
                        if is_string_numeric(&fnm){
                            code.last_mut().unwrap().push(Token::Number(fnm.parse::<f64>().unwrap()));
                        } else if fnm.contains('(') && is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_start_matches('(').parse::<f64>().unwrap()));
                        } else if fnm.contains(')') && is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_end_matches(')').parse::<f64>().unwrap()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else if fnm.contains('(') && !is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                        } else if fnm.contains(')') && !is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else {
                            code.last_mut().unwrap().push(Token::VarName(fnm));
                        }
                        code.last_mut().unwrap().push(Token::Operator('+'));
                        curword.clear();
                        let mut fnm = String::new();
                        opscount = opscount - 1;
                        fnmfound = false;
                        if opscount == 0 {
                            lastvar = true;
                        }
                    } else if char == '-' && !fnmfound{
                        fnmfound = true;
                        fnm = curword.clone().into_iter().collect();
                        if is_string_numeric(&fnm){
                            code.last_mut().unwrap().push(Token::Number(fnm.parse::<f64>().unwrap()));
                        } else if fnm.contains('(') && is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_start_matches('(').parse::<f64>().unwrap()));
                        } else if fnm.contains(')') && is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_end_matches(')').parse::<f64>().unwrap()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else if fnm.contains('(') && !is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                        } else if fnm.contains(')') && !is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else {
                            code.last_mut().unwrap().push(Token::VarName(fnm));
                        }
                        code.last_mut().unwrap().push(Token::Operator('-'));
                        curword.clear();
                        let mut fnm = String::new();
                        opscount = opscount - 1;
                        fnmfound = false;
                        if opscount == 0 {
                            lastvar = true;
                        }
                    } else if char == '*' && !fnmfound{
                        fnmfound = true;
                        fnm = curword.clone().into_iter().collect();
                        if is_string_numeric(&fnm){
                            code.last_mut().unwrap().push(Token::Number(fnm.parse::<f64>().unwrap()));
                        } else if fnm.contains('(') && is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_start_matches('(').parse::<f64>().unwrap()));
                        } else if fnm.contains(')') && is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_end_matches(')').parse::<f64>().unwrap()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else if fnm.contains('(') && !is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                        } else if fnm.contains(')') && !is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else {
                            code.last_mut().unwrap().push(Token::VarName(fnm));
                        }
                        code.last_mut().unwrap().push(Token::Operator('*'));
                        curword.clear();
                        let mut fnm = String::new();
                        opscount = opscount - 1;
                        fnmfound = false;
                        if opscount == 0 {
                            lastvar = true;
                        }
                    } else if char == '/' && !fnmfound{
                        fnmfound = true;
                        fnm = curword.clone().into_iter().collect();
                        if is_string_numeric(&fnm){
                            code.last_mut().unwrap().push(Token::Number(fnm.parse::<f64>().unwrap()));
                        } else if fnm.contains('(') && is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_start_matches('(').parse::<f64>().unwrap()));
                        } else if fnm.contains(')') && is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::Number(fnm.trim_end_matches(')').parse::<f64>().unwrap()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else if fnm.contains('(') && !is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                            code.last_mut().unwrap().push(Token::Operator('('));
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                        } else if fnm.contains(')') && !is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                            code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                            code.last_mut().unwrap().push(Token::Operator(')'));
                        } else {
                            code.last_mut().unwrap().push(Token::VarName(fnm));
                        }
                        code.last_mut().unwrap().push(Token::Operator('/'));
                        curword.clear();
                        let mut fnm = String::new();
                        opscount = opscount - 1;
                        fnmfound = false;
                        if opscount == 0 {
                            lastvar = true;
                        }
                    } 
            }
            fnm = curword.clone().into_iter().collect();
            if is_string_numeric(&fnm){
                code.last_mut().unwrap().push(Token::Number(fnm.parse::<f64>().unwrap()));
            } else if fnm.contains('(') && is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                code.last_mut().unwrap().push(Token::Operator('('));
                code.last_mut().unwrap().push(Token::Number(fnm.trim_start_matches('(').parse::<f64>().unwrap()));
            } else if fnm.contains(')') && is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                code.last_mut().unwrap().push(Token::Number(fnm.trim_end_matches(')').parse::<f64>().unwrap()));
                code.last_mut().unwrap().push(Token::Operator(')'));
            } else if fnm.contains('(') && !is_string_numeric(&fnm.trim_start_matches('(').to_string()){
                code.last_mut().unwrap().push(Token::Operator('('));
                code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
            } else if fnm.contains(')') && !is_string_numeric(&fnm.trim_end_matches(')').to_string()){
                code.last_mut().unwrap().push(Token::VarName(fnm.trim_end_matches(')').to_string()));
                code.last_mut().unwrap().push(Token::Operator(')'));
            } else {
                code.last_mut().unwrap().push(Token::VarName(fnm));
            }
            curword.clear();
        }
    } 
    code
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}


pub fn is_string_numeric(str: &String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

// struct Token {
// }
#[derive(Debug,Clone, PartialEq)]
pub enum Token {
        Keyword(String),
        Word(String),
        Operator(char), 
        Number(f64),
        VarName(String),
}
// pub trait getVal{
//     fn getVal(self)->String{}
// }

// impl getVal for Token {
//     fn getVal(self)-> String{

//     }
// }

// impl Copy for Vec<Token>{
//     T 
// }
  
//   static &str = IdentifierStr; // Filled in if tok_identifier
//   static double NumVal;


#[derive(Debug, Clone)]
pub enum Expression{
    SysFunc(Token, Token), 
    Func(),
    Math(Box<Expression> ,char, Box<Expression>),
    Assignment(Box<Expression> ,String),
    ParseError(),
    MathLast(Token, Token),
    Num(f64),
    Vartype(String),
} 

fn parsebinop<Item, Container: IntoIterator<Item=Item>>(left: &Expression, right: &Container) -> Expression{
    
}

pub fn parser(comd: Vec<Token>)->Expression{
    // let node = ASTNode
    //dbg!(&cmd);
    let mut kword = Token::Keyword("".to_string());
    let mut mathexprs: Vec<Expression> = vec![];
    let mut varname = Token::Keyword("".to_string());
    let mut mathassign = false;
    let mut iterator = comd.iter().peekable();
    while let Some(token) = iterator.next(){
        //dbg!(token);
        if let Token::Keyword(cmd) = token{
            //dbg!(&cmd);
            if cmd.clone() == "print".to_string(){
                //return Expression::SysFunc(cmd , ())
            } else if cmd.clone() == "let".to_string() || cmd.clone() == "var".to_string(){
                mathassign = true;
                dbg!(token);
                kword = token.clone();
                varname = iterator.next().unwrap().clone();
                if let Token::VarName(vrnm) = varname{
                    mathexprs.push(Expression::Assignment(Box::new(Expression::Vartype(cmd.clone())), vrnm.clone()));
                }
            }
        } else if let Token::Operator(cmdf) = token {
            if  cmdf.clone() == '+' || cmdf.clone() == '-' || cmdf.clone() == '*' || cmdf.clone() == '/' {
                //mathexprs.push(Expression::Math(iterator.prev_peek().unwrap().clone().clone(), token.clone(), iterator.peek().unwrap().clone().clone()))
                parsebinop(&mathexprs.last().unwrap().clone(), &iterator);
            }
        }
    }
    if mathassign{
            
    }
    // dbg!(kword);
    // dbg!(varname);
    return Expression::Func();
    //node.unwrap()
    // match token {
        //     tok: if tok as String == "print".to_string() => {
        //         return Expression::SysFunc(token.clone(), iterator.next().unwrap().clone());
        //     },
        //     _ => panic!()
        //     // Token::Keyword()
        // }
    //return ASNode{content: Token::Keyword("print".to_string()), LNode: Some(Box::new(ASTNode{content: iterator.peek().unwrap(), LNode: None, RNode: None})) , RNode: None};
    //ASTNode{content: Token::Keyword("dfadsf".to_string()), LNode: None, RNode: None}
}

//fn calculate()
// #[derive(Debug)]
// pub struct ASTNode{
//     pub content: Expression,
//     pub RNode: Option<Box<ASTNode>>, 
//     pub LNode: Option<Box<ASTNode>>,
// }

// pub struct ASTTree{
//     pub content: Token,
//     pub RNode : Option<Box<ASTNode>>,
//     pub LNode : Option<Box<ASTNode>>,
// }

// impl ASTTree {
//     fn new(content: Token, rnode: ASTNode, lnode: ASTNode)->ASTTree {
//         ASTTree{content: content, RNode: None, LNode: None}
//     }
// }