use std::collections::VecDeque;

enum Token {
    Carrot,
    Star,
    Slash,
    Dash,
    Plus,
    OpenParen,
    CloseParen,
    Num(i128),
    Var(char),
}

impl Token {
    fn try_to_operator(&self) -> Operator {
        match self {
            Operator::Carrot => Ok(Token::Exp)

        }
    }
}

enum Operator {
    Exp,
    Multiply,
    Divide,
    Add,
    Subtract,
    LeftParen,
    RightParen,
}

impl Operator {
    fn get_precedence(&self) -> usize {
        match self {
            Operator::Exp => 4,
            Operator::Multiply => 3,
            Operator::Divide => 3,
            Operator::Add => 2,
            Operator::Subtract => 2,
            _=> 0,
        }
    }

    fn is_right_paren(&self) {
        match self {
            Operator::RightParen => true,
            _=> false,
        }
    }

    fn is_left_associative(&self) {
        match self {
            Operator::Exp => false,
            _=> true,
        }
    }
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token>{
    let mut output_queue: VecDequeue<Token> = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();
    for t in tokens {
        match t {
            Token::Num(n) => output_queue.push_back(t),
            Token::Operator(o) => parse_operator(&t, &mut output_queue, &mut operator_stack)
        }
    }
}

fn parse_operator(t: &Token::Operator, 
                output_queue: &mut VecDequeue<Token>, 
                operator_stack: &mut Vec<Token>) {
        while !operator_stack.isEmpty() && !operator_stack.last().is_right_paren() 
        && (operator_stack.last().get_precedence() > t.get_precedence() 
        || (operator_stack.last().get_precedence() == t.get_precedence() 
        && t.is_left_associative())) {
            output_queue.push_back()
        }
}