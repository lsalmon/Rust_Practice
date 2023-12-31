use std::iter::Peekable;
use std::str::Chars;
use anyhow::{bail, Context, Result};
use thiserror::Error;
use std::error::Error;

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

#[derive(Debug)]
struct Tokenizer<'a>(Peekable<Chars<'a>>);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Unexpected character {0}")]
struct TokenizerError(char);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Result<Token, TokenizerError>> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Number(num)))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Identifier(ident)))
            }
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(TokenizerError(c))),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Unexpected token {0}")]
struct ParserError(String);

fn parse(input: &str) -> Result<Expression, Box<dyn Error>> {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Result<Expression, Box<dyn Error>> {
        let tok = tokens.next()
                        .with_context(|| format!("Failed to read {tokens:?}"))?;
        let expr = match tok {
            Ok(Token::Number(num)) => {
                let v = num.parse().context("Invalid 32-bit integer'")?;
                Expression::Number(v)
            }
            Ok(Token::Identifier(ident)) => Expression::Var(ident),
            Ok(Token::Operator(_)) => return Err(Box::new(ParserError("token".to_string()))),
            Err(_) => return Err("Tokenize failed".into())
        };
        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => Ok(expr),
            Some(Ok(Token::Operator(op))) => {
                Ok(Expression::Operation(Box::new(expr), op, Box::new(parse_expr(tokens)?)))
            }
            Some(Ok(tok)) => Err(Box::new(ParserError("token".to_string()))),
            Some(Err(e)) => Err(Box::new(e))
        }
    }

    Ok(parse_expr(&mut tokens)?)
}

fn main() {
    let expr = match parse("10+foo+20-30") {
        Ok(exp) => exp,
        Err(reason) => {
            println!("{reason}");
            return ();
        }
    };
    println!("{expr:?}");
}
