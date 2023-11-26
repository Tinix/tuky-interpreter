//
// lexer.rs
// Copyright (C) 2023 Daniel Tinivella <tinix@debian>
// Distributed under terms of the MIT license.
//
//
use crate::token::Token;

struct Lexer {
   input: Vec<char>,
}

impl Lexer {
   pub fn new(input: &str) -> Lexer {
      todo!();
   }
   fn next_token(&self) -> Token {
      todo!();
   }
}



#[cfg(test)]
mod tests {
   use crate::token::{Token, TokenKind};
   use crate::lexer::Lexer;


   #[test]
   fn test_next_token() {
      let input: &str = "=+(){},;";

      let expected: Vec<Token> = vec![
         Token {
            kind: TokenKind::Assign,
            literal: "=".to_string(),
         },
         Token {
            kind: TokenKind::Plus,
            literal: "+".to_string(),
         },
         Token {
            kind: TokenKind::LParen,
            literal: "(".to_string(),
         },
         Token {
            kind: TokenKind::RParen,
            literal: ")".to_string(),
         },
         Token {
            kind: TokenKind::LBrace,
            literal: "{".to_string(),
         },
         Token {
            kind: TokenKind::RBrace,
            literal: "}".to_string(),
         },
         Token {
            kind: TokenKind::Comma,
            literal: ",".to_string(),
         },
         Token {
            kind: TokenKind::Semicolon,
            literal: ";".to_string(),
         },
         Token {
            kind: TokenKind::Eof,
            literal: "".to_string(),
         },
         ];
      let lexer: Lexer = Lexer::new(input);

      for (idx, exp_token) in expected.into_iter().enumerate() {
         let recv_token = lexer.next_token();
         assert_eq!(
            exp_token.kind, recv_token.kind,
            "tests[{idx}] - token type wrong. expected={}, got={}" 
            , exp_token.kind, recv_token.kind);

         assert_eq!(
            exp_token.literal, recv_token.literal,
            "tests[{idx}] - token literal wrong. expected={}, got={}" 
            , exp_token.literal, recv_token.literal
            );
      }
   }
}
