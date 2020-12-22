use std::iter::Peekable;

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/18");

    let result = INPUT
        .lines()
        .map(|l| parse(l, false).evaluate())
        .sum::<usize>();

    println!("[PART 1] Sum {}", result);

    let result = INPUT
        .lines()
        .map(|l| parse(l, true).evaluate())
        .sum::<usize>();

    println!("[PART 2] Sum {}", result);
}

#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(usize),
    Paren
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
    pub fn evaluate(&self) -> usize {
        //println!("{:?}", self);
        match self.entry {
            GrammarItem::Product => self.children[0].evaluate() * self.children[1].evaluate(),
            GrammarItem::Sum => self.children[0].evaluate() + self.children[1].evaluate(),
            GrammarItem::Number(n) => n,
            GrammarItem::Paren => self.children[0].evaluate(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(usize),
}

fn lex(input: &str) -> Vec<LexItem> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                unreachable!()
            }
        }
    }
    result
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> usize {
    let mut number = c.to_string().parse::<usize>().unwrap();
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

pub fn parse(input: &str, weird: bool) -> ParseNode {
    let mut tokens = lex(input);
    tokens.reverse();
    parse_expr(&tokens, 0, weird).0
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize, weird: bool) -> (ParseNode, usize) {
    let (node_summand, next_pos) = parse_summand(tokens, pos, weird);
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            if weird {
                (node_summand, next_pos)
            }else {
                // recurse on the expr
                let mut sum = ParseNode::new();
                sum.entry = GrammarItem::Sum;
                sum.children.push(node_summand);
                let (rhs, i) = parse_expr(tokens, next_pos + 1, weird);
                sum.children.push(rhs);
                (sum, i)
            }
        }
        Some(&LexItem::Op('*')) => {
            // recurse on the expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Product;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1, weird);
            sum.children.push(rhs);
            (sum, i)
        }
        _ => {
            // we have just the summand production, nothing more.
            (node_summand, next_pos)
        }
    }
}

fn parse_summand(tokens: &Vec<LexItem>, pos: usize, weird: bool) -> (ParseNode, usize) {
    let (node_term, next_pos) = parse_term(tokens, pos, weird);
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            if weird {
                (node_term, next_pos)
            }else {
                // recurse on the summand
                let mut product = ParseNode::new();
                product.entry = GrammarItem::Product;
                product.children.push(node_term);
                let (rhs, i) = parse_summand(tokens, next_pos + 1, weird);
                product.children.push(rhs);
                (product, i)
            }
        }
        Some(&LexItem::Op('+')) => {
            // recurse on the summand
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Sum;
            product.children.push(node_term);
            let (rhs, i) = parse_summand(tokens, next_pos + 1, weird);
            product.children.push(rhs);
            (product, i)
        }
        _ => {
            // we have just the term production, nothing more.
            (node_term, next_pos)
        }
    }
}

fn parse_term(tokens: &Vec<LexItem>, pos: usize, weird: bool) -> (ParseNode, usize) {
    let c: &LexItem = tokens.get(pos).unwrap();
    match c {
        &LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(n);
            (node, pos + 1)
        }
        &LexItem::Paren(c) => {
            assert_eq!(')', c);
            let (node, next_pos) = parse_expr(tokens, pos + 1, weird);
            assert_eq!(Some(&LexItem::Paren('(')), tokens.get(next_pos));
            let mut paren = ParseNode::new();
            paren.children.push(node);
            (paren, next_pos + 1)
        }
        _ => {
            unreachable!("Unexpected token {:?}, expected ')' or number", c)
        }
    }
}

