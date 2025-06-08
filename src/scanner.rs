use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            self.line,
        ));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    // Comment goes to end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::And);
                }
                // Single & is not valid in Lox, could report error here
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                }
                // Single | is not valid in Lox, could report error here
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    // Unexpected character - in a real implementation, we'd report an error
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap_or('\0') != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Unterminated string - in a real implementation, we'd report an error
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literal(TokenType::String, Some(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_literal(TokenType::Number, Some(value));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let mut scanner = Scanner::new("".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_single_character_tokens() {
        let mut scanner = Scanner::new("(){},.+-;*".to_string());
        let tokens = scanner.scan_tokens();
        
        let expected_types = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Semicolon,
            TokenType::Star,
            TokenType::Eof,
        ];
        
        assert_eq!(tokens.len(), expected_types.len());
        for (i, expected_type) in expected_types.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_two_character_tokens() {
        let mut scanner = Scanner::new("!= == <= >=".to_string());
        let tokens = scanner.scan_tokens();
        
        let expected_types = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Eof,
        ];
        
        assert_eq!(tokens.len(), expected_types.len());
        for (i, expected_type) in expected_types.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_string_literal() {
        let mut scanner = Scanner::new("\"hello world\"".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 2); // String + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].literal, Some("hello world".to_string()));
    }

    #[test]
    fn test_number_literal() {
        let mut scanner = Scanner::new("123 45.67".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 3); // Two numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some("123".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some("45.67".to_string()));
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("variable _private".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 3); // Two identifiers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "variable");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "_private");
    }

    #[test]
    fn test_keywords() {
        let mut scanner = Scanner::new("var if else while for".to_string());
        let tokens = scanner.scan_tokens();
        
        let expected_types = vec![
            TokenType::Var,
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::For,
            TokenType::Eof,
        ];
        
        assert_eq!(tokens.len(), expected_types.len());
        for (i, expected_type) in expected_types.iter().enumerate() {
            assert_eq!(tokens[i].token_type, *expected_type);
        }
    }

    #[test]
    fn test_comment() {
        let mut scanner = Scanner::new("// This is a comment\nvar x;".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 4); // var, x, ;, EOF
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut scanner = Scanner::new("   var   x   ;   ".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 4); // var, x, ;, EOF
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_line_counting() {
        let mut scanner = Scanner::new("var\nx\n;".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
    }
}
