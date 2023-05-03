pub mod parser {
    use std::collections::VecDeque;
    #[derive(Debug)]

    // used to store general tokens of the input string
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
        Func(Function<'static>),
    }

    // used to store all operators and functions
    impl Operator {
        pub fn get_precedence_and_is_left(&self) -> (usize, bool) {
            match self {
                Operator::Func(_) => (5, false),
                Operator::Exp(_) => (4, false),
                Operator::Multiply(_) => (3, true),
                Operator::Divide(_) => (3, true),
                Operator::Add(_) => (2, true),
                Operator::Subtract(_) => (2, true),
                Operator::LeftParen(_) => (9, true),
                Operator::RightParen(_) => (0, true),
            }
        }

        pub fn execute(&self, num2: f32, num1: f32) -> f32 {
            match self {
                Operator::Exp(_) => num2.powf(num1),
                Operator::Multiply(_) => num2 * num1,
                Operator::Divide(_) => num2 / num1,
                Operator::Add(_) => num2 + num1,
                Operator::Subtract(_) => num2 - num1,
                Operator::Func(f) => f.execute(num1),
                _=> 0.0,
            }
        }
    }

    #[derive(Debug)]
    pub enum Function<'a> {
        Sin(&'a str),
        Cos(&'a str),
        Tan(&'a str),
        Ln(&'a str),
        Log(&'a str),
        Sqrt(&'a str),
        Abs(&'a str),
    }

    impl Function<'static> {
        pub fn execute(&self, num: f32) -> f32 {
            match self {
                Function::Sin(_) => num.sin(),
                Function::Cos(_) => num.cos(),
                Function::Tan(_) => num.tan(),
                Function::Ln(_) => num.ln(),
                Function::Log(_) => num.log10(),
                Function::Sqrt(_) => num.sqrt(),
                Function::Abs(_) => num.abs(),
            }
        }
    }

    // converts input string to tokens
    // offloaded as much work as possible fromt the shunting yard algorithm to the tokenizer
    pub fn tokenize(input: String) -> Result<Vec<Token>, &'static str> {
        let functions = ["sin", "cos", "tan", "ln", "log", "sqrt", "abs"];
        let functions = functions.map(|s| String::from(s));
        let mut output: Vec<Token> = Vec::new();
        let mut cur_function: String = String::new();
        let mut cur_num: String = String::new();
        let chars = input.chars();
        for (idx, c) in chars.enumerate() {
            match c {
                '0'..='9' => {
                    if !cur_function.is_empty() {
                        // a number cannot come directly after a function
                        return Err("invalid function");
                    }
                    if cur_num.is_empty() {
                        if idx > 0 {
                            // a number directly following a variable or right paren is a multiplication
                            let last = input.chars().nth(idx - 1).unwrap_or_default();
                            if last == 'x' || last == ')' {
                                output.push(Token::Operator(Operator::Multiply('*')));
                            }
                        }
                    }
                    cur_num.push(c);
                },
                '.' => {
                    if !cur_function.is_empty() {
                        return Err("invalid function");
                    }
                    cur_num.push(c)
                },
                '+' => {
                    if !cur_function.is_empty() {
                        return Err("invalid function");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    output.push(Token::Operator(Operator::Add('+')));
                }
                '*' => {
                    if !cur_function.is_empty() {
                        return Err("invalid function");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    output.push(Token::Operator(Operator::Multiply('*')));
                }
                '/' => {
                    if !cur_function.is_empty() {
                        return Err("invalid function");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    output.push(Token::Operator(Operator::Divide('/')));
                }
                '^' => {
                    if !cur_function.is_empty() {
                        return Err("invalid function");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    output.push(Token::Operator(Operator::Exp('^')));
                }
                '(' => {
                    if !cur_function.is_empty() {
                        // check if the previous group of characters is a valid function
                        if functions.contains(&cur_function) {
                            match cur_function.as_str() {
                                "sin" => {
                                    let op = Operator::Func(Function::Sin("sin"));
                                    output.push(Token::Operator(op));
                                },
                                "cos" => {
                                    let op = Operator::Func(Function::Cos("cos"));
                                    output.push(Token::Operator(op));
                                },
                                "tan" => {
                                    let op = Operator::Func(Function::Tan("tan"));
                                    output.push(Token::Operator(op));
                                },
                                "ln" => {
                                    let op = Operator::Func(Function::Ln("ln"));
                                    output.push(Token::Operator(op));
                                },
                                "log" => {
                                    let op = Operator::Func(Function::Log("log"));
                                    output.push(Token::Operator(op));
                                },
                                "sqrt" => {
                                    let op = Operator::Func(Function::Sqrt("sqrt"));
                                    output.push(Token::Operator(op));
                                },
                                "abs" => {
                                    let op = Operator::Func(Function::Abs("abs"));
                                    output.push(Token::Operator(op));
                                },
                                _=> {},
                            }
                            // reset cur_function after pushing
                            cur_function = String::new();
                        } else {
                            return Err("invalid function");
                        }
                    }
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap_or_default();
                        output.push(Token::Num(num));
                        //a number directly before parenthesis is a multiplication
                        output.push(Token::Operator(Operator::Multiply('*')));
                        cur_num = String::new();
                    } else if let Some(&Token::Var) = output.last() {
                        output.push(Token::Operator(Operator::Multiply('*')));
                    }
                    output.push(Token::Operator(Operator::LeftParen('(')));
                }
                ')' => {
                    if !cur_function.is_empty() {
                        return Err("Syntax error");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    output.push(Token::Operator(Operator::RightParen(')')));
                }
                'x' => {
                    if !cur_function.is_empty() {
                        return Err("Syntax error");
                    }
                    if !cur_num.is_empty() {
                        let num = cur_num.parse::<f32>().unwrap_or_default();
                        output.push(Token::Num(num));
                        output.push(Token::Operator(Operator::Multiply('*')));
                        cur_num = String::new();
                    } else if let Some(&Token::Var) = output.last() {
                        output.push(Token::Operator(Operator::Multiply('*')));
                    }
                    output.push(Token::Var);
                }
                '-' => {
                    if !cur_function.is_empty() {
                        return Err("Syntax error");
                    }
                    cur_num = check_cur_num(&mut output, cur_num);
                    // handle unary -
                    let last = output.last();
                    match last {
                        Some(Token::Operator(o)) => {
                            match o {
                                Operator::Add(_) => {
                                    output.pop();
                                    output.push(Token::Operator(Operator::Subtract('-')));
                                },
                                Operator::Subtract(_) => {
                                    output.pop();
                                    output.push(Token::Operator(Operator::Add('+')));
                                },
                                Operator::Multiply(_) => {
                                    output.push(Token::Num(-1.0));
                                    output.push(Token::Operator(Operator::Multiply('*')));
                                },
                                Operator::Divide(_) => {
                                    output.push(Token::Num(-1.0));
                                    output.push(Token::Operator(Operator::Divide('/')));
                                },
                                Operator::Exp(_) => {
                                    output.push(Token::Num(-1.0));
                                    output.push(Token::Operator(Operator::Exp('^')));
                                },
                                Operator::LeftParen(_) => {
                                    output.push(Token::Num(-1.0));
                                    output.push(Token::Operator(Operator::Multiply('*')));
                                },
                                Operator::RightParen(_) => {
                                    output.push(Token::Operator(Operator::Subtract('-')));
                                },
                                _=> {
                                    return Err("Syntax error");
                                },
                            }
                        },
                        Some(Token::Num(_)) => {
                            output.push(Token::Operator(Operator::Subtract('-')));
                        },
                        Some(Token::Var) => {
                            output.push(Token::Operator(Operator::Subtract('-')));
                        }
                        _=> {
                            output.push(Token::Num(-1.0));
                            output.push(Token::Operator(Operator::Multiply('*')));
                        }
                    }
                }
                ' ' => {}
                _ => {
                    // is alphabetic checks if it could be part of a valid function
                    if c.is_alphabetic() {
                        cur_num = check_cur_num(&mut output, cur_num);
                        if cur_function.is_empty() && idx > 0 {
                            let last = input.chars().nth(idx - 1).unwrap_or_default();
                            if last == 'x' || last == ')' || last.is_numeric() {
                                output.push(Token::Operator(Operator::Multiply('*')));
                            }
                        }
                        cur_function.push(c);
                    } else {
                        println!("Invalid character found: {}", c);
                        return Err("Invalid character");
                    }
                }
            }
        }
        if !cur_num.is_empty() {
            output.push(Token::Num(cur_num.parse::<f32>().unwrap_or_default()));
        }
        // for printing tokens
        // for t in &output {
        //     match t {
        //         Token::Num(n) => println!("{}", n),
        //         Token::Operator(o) => println!("{:?}", o),
        //         Token::Var => println!("x"),
        //     }
        // }
        Ok(output)
    }

    fn check_cur_num(tokens_output: &mut Vec<Token>, cur_num: String) -> String {
        if !cur_num.is_empty() {
            let num = cur_num.parse::<f32>().unwrap_or_default();
            tokens_output.push(Token::Num(num));
            String::new()
        } else {
            cur_num
        }
    }

    pub fn infix_to_postfix(infix: Vec<Token>) -> Vec<Token>{
        // shunting yard algorithm for arithmetic parsing
        //TODO: add functions
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
                        } else { break; }
                    } 
                    let cur_operator = o;
                    match cur_operator {
                        Operator::RightParen(_) => {},
                        _=> { stack.push(cur_operator); }
                    }
                }
            }
        } while !stack.is_empty() {
            let to_push = Token::Operator(stack.pop().unwrap());
            output_queue.push_back(to_push);
        }
        let mut output: Vec<Token> = Vec::new();
        for t in output_queue {
            //println!("{:?}", t);
            output.push(t);
        }
        output
    }
}