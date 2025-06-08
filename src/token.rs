#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(
            TokenType::Identifier,
            "variable".to_string(),
            None,
            1,
        );
        
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.lexeme, "variable");
        assert_eq!(token.literal, None);
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_token_with_literal() {
        let token = Token::new(
            TokenType::String,
            "\"hello\"".to_string(),
            Some("hello".to_string()),
            2,
        );
        
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(token.lexeme, "\"hello\"");
        assert_eq!(token.literal, Some("hello".to_string()));
        assert_eq!(token.line, 2);
    }

    #[test]
    fn test_token_clone() {
        let original = Token::new(
            TokenType::Number,
            "42".to_string(),
            Some("42".to_string()),
            3,
        );
        
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Plus, TokenType::Plus);
        assert_ne!(TokenType::Plus, TokenType::Minus);
    }
}
