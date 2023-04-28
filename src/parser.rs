pub mod parser {
    use std::collections::VecDeque;
    pub enum Token {
        Operator(Operator),
        Num(f32),
    }

    pub enum Operator {
        Exp,
        Multiply,
        Divide,
        Add,
        Subtract,
        LeftParen,
        RightParen,
    }

    impl Operator {
        pub fn get_precedence_and_is_left(&self) -> (usize, bool) {
            match self {
                Operator::Exp => (4, false),
                Operator::Multiply => (3, true),
                Operator::Divide => (3, true),
                Operator::Add => (2, true),
                Operator::Subtract => (2, true),
                Operator::LeftParen => (9, true),
                Operator::RightParen => (0, true),
            }
        }
    }

    pub fn infix_to_postfix(infix: Vec<Token>) -> Vec<Token>{
        let mut output_queue: VecDeque<Token> = VecDeque::new();
        let mut stack: Vec<Operator> = Vec::new();
        for t in infix {
            match t {
                Token::Num(n) => output_queue.push_back(t),
                Token::Operator(o) => {
                    while !stack.is_empty() { 
                        let cur_operator = &o;
                        let top_of_stack = stack.last().unwrap();
                        let cur_prec_and_left = cur_operator.get_precedence_and_is_left();
                        let tos_prec_and_left = top_of_stack.get_precedence_and_is_left();
                        if (cur_prec_and_left.1 && cur_prec_and_left.0 <= tos_prec_and_left.0)
                        || (!cur_prec_and_left.1 && cur_prec_and_left.0 < tos_prec_and_left.0) {
                            match cur_operator {
                                Operator::RightParen => {
                                    match top_of_stack {
                                        Operator::LeftParen => {
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
                                        Operator::LeftParen => break,
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
                        Operator::RightParen => {},
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