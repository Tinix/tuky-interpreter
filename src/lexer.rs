//
// lexer.rs
// Copyright (C) 2023 Daniel Tinivella <tinix@debian>
// Distributed under terms of the MIT license.
//
//
use crate::token::Token;

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
   pub fn new(input: &str) -> Lexer {
      let mut lexer: Lexer = Lexer {
         input: input.chars().collect(),
         position: 0,
         read_position: 0,
         ch: Default::default(),
        };

        lexer.read_char();
        lexer
   }

   fn read_char(&mut self) {
      if self.read_position >= self.input.len() {
         self.ch = '\0';
      } else {
         self.ch = self.input[self.read_position];
      }
      self.position = self.read_position;
      self.read_position += 1;
   }

   fn next_token(&self) -> Token {
      let token = match self.ch {
         '=' =>
            Lexer::new_token(TokenKind::Assign, self.ch),
         ';' =>
            Lexer::new_token(TokenKind::Semicolon, self.ch),
         '(' =>
            Lexer::new_token(TokenKind::LParen, self.ch),
         ')' =>
            Lexer::new_token(TokenKind::RParen, self.ch),
         ',' =>
            Lexer::new_token(TokenKind::Comma, self.ch),
         '+' =>
            Lexer::new_token(TokenKind::Plus, self.ch),
         '{' =>
            Lexer::new_token(TokenKind::LBrace, self.ch),
         '}' =>
            Lexer::new_token(TokenKind::RBrace, self.ch),
         '\0' =>
            Token { kind: TokenKind::Eof, literal: "".to_string() },
         _ => Lexer::new_token(TokenKind::Illegal, self.ch),
      };
   }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token { kind, literal: ch.to_string() }
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
            , exp_token.kind, recv_token.kind
            );

         assert_eq!(
            exp_token.literal, recv_token.literal,
            "tests[{idx}] - token literal wrong. expected={}, got={}" 
            , exp_token.literal, recv_token.literal
            );
      }
   }
}
