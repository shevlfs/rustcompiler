extern crate prev_iter;
use std::iter::{Peekable, Iterator};



pub fn tokenize(content: String) -> Vec<Vec<Token>> {
    let mut code: Vec<Vec<Token>> = vec![vec![]];
    let mut curword = Vec::new();
    let linesplit: Vec<String> = content.split(";").map(|s| s.to_string()).collect();
    for line in linesplit.iter(){
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
    Stdout(Box<Expression>), 
    Func(),
    Math(Box<Expression> ,char, Box<Expression>),
    Assignment(Box<Expression> ,String),
    Eqls(Box<Expression>, Box<Expression>),
    Literal(String),
    ParseError(),
    Num(f64),
    VarName(String),
    Vartype(String),
} 

fn parsebinop<'a, iter: Iterator<Item=&'a Token>>(left: &Expression, right: &mut prev_iter::PrevPeekable<iter>, cur: &Token, opscount: &mut i32, multdiv: bool) -> Expression {
    let prevel = right.prev_peek().unwrap().clone();
    let nxtel = right.peek().unwrap().clone();
    //dbg!(prevel.clone());
    //dbg!(nxtel.clone());
    if opscount.clone() == 1 {
        if let Token::Number(num) = prevel{
            if let Token::Number(num2) = nxtel {
                if let Token::Operator(op) = cur{
                    return Expression::Eqls(Box::new(left.clone()), Box::new(Expression::Math(Box::new(Expression::Num(num.clone())), op.clone(), Box::new(Expression::Num(num2.clone())))))
                }
            } else if let Token::VarName(vrname) = nxtel {
                if let Token::Operator(op) = cur{
                    return Expression::Eqls(Box::new(left.clone()), Box::new(Expression::Math(Box::new(Expression::Num(num.clone())), op.clone(), Box::new(Expression::VarName(vrname.to_string())))))
                }
            }
        } else if let Token::VarName(vrname) = prevel{
            if let Token::Number(num2) = nxtel {
                if let Token::Operator(op) = cur{
                    return Expression::Eqls(Box::new(left.clone()), Box::new(Expression::Math(Box::new(Expression::VarName(vrname.to_string())), op.clone(), Box::new(Expression::Num(num2.clone())))))
                }
            } else if let Token::VarName(vrname2) = nxtel{
                if let Token::Operator(op) = cur{
                    return Expression::Eqls(Box::new(left.clone()), Box::new(Expression::Math(Box::new(Expression::VarName(vrname.to_string())), op.clone(), Box::new(Expression::VarName(vrname2.to_string())))))
                }
            }   
        }
    } else {
        if let Token::Number(num) = prevel{
            if let Token::Number(num2) = nxtel {
                if let Token::Operator(op) = cur{
                    *opscount -= 1;
                    right.next();
                    let newiter = right;
                    if  newiter.peek().is_some(){
                        let newel = newiter.next().unwrap().clone();
                        if !multdiv{
                            return parsebinop(&Expression::Eqls(Box::new(left.clone()), Box::new(Expression::Math(Box::new(Expression::Num(num.clone())), op.clone(), Box::new(Expression::Num(num2.clone()))))), newiter, &newel, opscount, multdiv);
                        } else {
                            if op.clone() == '*' {
                                //return parsebinop(&Expression)
                            }
                        }

                    } else {
                        return Expression::Math(Box::new(left.clone()), op.clone(),Box::new(Expression::Num(*num2)));
                    }
                    }
            }
        }
    }
    return Expression::Func();
}

pub fn parser(comd: Vec<Token>)->Expression{
    // let node = ASTNode
    //dbg!(&cmd);
    let mut numscount = 7;
    let mut mathexprs: Vec<Expression> = vec![];
    let mut opscount = 0;
    let mut multdiv = false;
    for command in comd.clone(){
        if let Token::Operator(op) = command {
            if op != '='{
                opscount = opscount + 1;
            }
            if op == '*' {
                multdiv = true;
            }
        }
    }
    println!("opscount - {}",opscount.clone());
    let mut mathassign = false;
    let mut iterator = prev_iter::PrevPeekable::new(comd.iter().peekable());
    while let Some(token) = iterator.next(){
        //dbg
        if let Token::Keyword(cmd, ) = token{
            //dbg!(&cmd);
            if cmd.clone() == "print".to_string(){
                if let Token::Word(content) = iterator.peek().unwrap().clone(){
                    return Expression::Stdout(Box::new(Expression::Literal(content.to_string())));
            }
            } else if cmd.clone() == "let".to_string() || cmd.clone() == "var".to_string(){
                mathassign = true;
                dbg!(token);
                let kword = token.clone();
                let varname = iterator.next().unwrap().clone();
                if let Token::VarName(vrnm) = varname{
                    mathexprs.push(Expression::Assignment(Box::new(Expression::Vartype(cmd.clone())), vrnm.clone()));
                    numscount = 1;
                }
            }
        } else if let Token::Operator(cmdf) = token {
            if cmdf.clone() == '+' || cmdf.clone() == '-' || cmdf.clone() == '/' || cmdf.clone() == '*' {
                //mathexprs.push(Expression::Math(iterator.prev_peek().unwrap().clone().clone(), token.clone(), iterator.peek().unwrap().clone().clone()))
                if opscount == 1 {
                    if let Token::Number(numprev) = iterator.prev_peek().unwrap() {
                        if let Token::Number(numnext ) = iterator.peek().unwrap() {
                            return Expression::Eqls(Box::new(mathexprs.last().unwrap().clone()), Box::new(Expression::Math(Box::new(Expression::Num(numprev.clone())), cmdf.clone(), Box::new(Expression::Num(numnext.clone())) )));
                        }
                    }
                } else {
                    opscount = opscount + 1;
                    let xpr = parsebinop(&mathexprs.last().unwrap().clone(), &mut iterator, token, &mut opscount, multdiv);
                    return xpr;
                }
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