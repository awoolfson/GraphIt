pub mod parser {
    use std::collections::VecDeque;
    pub enum Token {
        Operator(Operator),
        Num(f32),
        Var,
    }

    #[derive(Debug)]
    pub enum Operator {
        Exp(char),
        Multiply(char),
        Divide(char),
        Add(char),
        Subtract(char),
        LeftParen(char),
        RightParen(char),
    }

    impl Operator {
        pub fn get_precedence_and_is_left(&self) -> (usize, bool) {
            match self {
                Operator::Exp(_) => (4, false),
                Operator::Multiply(_) => (3, true),
                Operator::Divide(_) => (3, true),
                Operator::Add(_) => (2, true),
                Operator::Subtract(_) => (2, true),
                Operator::LeftParen(_) => (9, true),
                Operator::RightParen(_) => (0, true),
            }
        }
    }

    pub fn tokenize(input: String) -> Result<Vec<Token>, &'static str> {
        let functions = ["sin", "cos", "tan", "ln", "log", "sqrt", "abs"];
        functions.map(|s| String::from(s));
        let mut output: Vec<Token> = Vec::new();
        let mut cur_function: String = String::new();
        let mut cur_num: String = String::new();
        let mut on_function = false;
        let chars = input.chars();
        for (idx, c) in chars.enumerate() {
            // while on_function {
            //     match cur_function {
            //         functions[..] => {
            //             on_function = false;
            //         },
            //     }
            // }
            match c {
                '0'..='9' => {
                    cur_num.push(c);
                },
                '.' => cur_num.push(c),
                '+' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::Add('+')));
                }
                '-' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    } else {
                        let last = output.last();
                        if output.is_empty() {
                            output.push(Token::Num(0.0));   
                        } else if let Token::Operator(Operator::LeftParen(_)) = last.unwrap() {
                            output.push(Token::Num(0.0));
                        }
                    }
                    output.push(Token::Operator(Operator::Subtract('-')));
                }
                '*' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::Multiply('*')));
                }
                '/' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::Divide('/')));
                }
                '^' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::Exp('^')));
                }
                '(' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::LeftParen('(')));
                }
                ')' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Operator(Operator::RightParen(')')));
                }
                'x' => {
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap();
                        output.push(Token::Num(num));
                        cur_num = String::new();
                    }
                    output.push(Token::Var);
                }
                ' ' => {}
                _ => {
                    // if c.is_alphabetic() {
                    //     on_function = true;
                    //     cur_function.push(c);
                    // } else {
                        println!("Invalid character found: {}", c);
                        return Err("Invalid character");
                    //}
                }
            }
        }
        if !cur_num.is_empty() {
            output.push(Token::Num(cur_num.parse::<f32>().unwrap()));
        }
        // for t in &output {
        //     match t {
        //         Token::Num(n) => println!("{}", n),
        //         Token::Operator(o) => println!("{:?}", o),
        //         Token::Var => println!("x"),
        //     }
        // }
        Ok(output)
    }

    pub fn infix_to_postfix(infix: Vec<Token>) -> Vec<Token>{
        let mut output_queue: VecDeque<Token> = VecDeque::new();
        let mut stack: Vec<Operator> = Vec::new();
        for t in infix {
            match t {
                Token::Num(_) => output_queue.push_back(t),
                Token::Var => output_queue.push_back(t),
                Token::Operator(o) => {
                    while !stack.is_empty() { 
                        let cur_operator = &o;
                        let top_of_stack = stack.last().unwrap();
                        let cur_prec_and_left = cur_operator.get_precedence_and_is_left();
                        let tos_prec_and_left = top_of_stack.get_precedence_and_is_left();
                        if (cur_prec_and_left.1 && cur_prec_and_left.0 <= tos_prec_and_left.0)
                        || (!cur_prec_and_left.1 && cur_prec_and_left.0 < tos_prec_and_left.0) {
                            match cur_operator {
                                Operator::RightParen(_) => {
                                    match top_of_stack {
                                        Operator::LeftParen(_) => {
                                            stack.pop();
                                            break;
                                        },
                                        _=> {
                                            let to_push = Token::Operator(stack.pop().unwrap());
                                            output_queue.push_back(to_push);
                                        }
                                    }
                                },
                                _=> {
                                    match top_of_stack {
                                        Operator::LeftParen(_) => break,
                                        _=> {
                                            let to_push = Token::Operator(stack.pop().unwrap());
                                            output_queue.push_back(to_push);
                                        }
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    } 
                    let cur_operator = o;
                    match cur_operator {
                        Operator::RightParen(_) => {},
                        _=> {
                            stack.push(cur_operator);
                        }
                    }
                }
            }
        } while !stack.is_empty() {
            let to_push =Token::Operator(stack.pop().unwrap());
            output_queue.push_back(to_push);
        }
        let mut output: Vec<Token> = Vec::new();
        for t in output_queue {
            output.push(t);
        }
        output
    }
}