use crate::lexer::error::LexError::{
    self, IntegerOverflow, InvalidEscape, InvalidIdentifier, UnterminatedString,
};
use logos::{Lexer, Logos};

mod error;

const VALID_ESCAPES: [char; 7] = ['n', 'r', 't', '0', '\\', '"', '\''];

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexError)]
pub enum Token<'a> {
    #[token("func")]
    Function,
    #[token("return")]
    Return,

    #[token("->")]
    RightArrow,
    #[token("(")]
    OpenParenthesis,
    #[token(")")]
    CloseParenthesis,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("=")]
    Equals,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,

    #[regex("\"", lex_string)]
    String(&'a str),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    Identifier(&'a str),

    #[regex(r"[0-9][a-zA-Z0-9_]*", lex_integer)]
    Integer(u64),
}

fn lex_string<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<&'a str, LexError> {
    let remainder = lexer.remainder();
    let mut escaping = false;

    for (pos, c) in lexer.remainder().char_indices() {
        if escaping {
            if !VALID_ESCAPES.contains(&c) {
                lexer.bump(pos);

                return Err(InvalidEscape);
            }

            escaping = false;

            continue;
        }

        if c == '"' {
            lexer.bump(pos + 1);

            return Ok(&remainder[..pos]);
        }

        if c == '\\' {
            escaping = true;
        }

        if c == '\n' {
            lexer.bump(pos);

            return Err(UnterminatedString);
        }
    }

    lexer.bump(remainder.len());

    return Err(UnterminatedString);
}

fn lex_integer<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<u64, LexError> {
    let slice = lexer.slice();
    let result = slice.parse::<u64>();

    match result {
        Err(e) => match e.kind() {
            std::num::IntErrorKind::InvalidDigit => Err(InvalidIdentifier),
            std::num::IntErrorKind::PosOverflow => Err(IntegerOverflow),

            _ => unreachable!(),
        },

        Ok(x) => Ok(x),
    }
}
