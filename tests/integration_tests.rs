use rlox::{interpreter::Interpreter, scanner::Scanner, token::TokenType};

#[test]
fn test_scanner_integration() {
    let source = "var x = 42 + 3.14;".to_string();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    
    let expected_types = vec![
        TokenType::Var,
        TokenType::Identifier,
        TokenType::Equal,
        TokenType::Number,
        TokenType::Plus,
        TokenType::Number,
        TokenType::Semicolon,
        TokenType::Eof,
    ];
    
    assert_eq!(tokens.len(), expected_types.len());
    for (i, expected_type) in expected_types.iter().enumerate() {
        assert_eq!(tokens[i].token_type, *expected_type);
    }
}

#[test]
fn test_interpreter_scanner_integration() {
    let mut interpreter = Interpreter::new();
    let source = "print \"Hello\"; var x = 123;".to_string();
    let had_error = interpreter.run_source(source);
    assert!(!had_error);
}

#[test]
fn test_full_program_tokenization() {
    let source = r#"
        fun fibonacci(n) {
            if (n <= 1) return n;
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        var result = fibonacci(10);
        print result;
    "#.to_string();
    
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    
    // Should have tokens for: fun, identifier, (, identifier, ), {, if, (, identifier, <=, number, ), return, identifier, ;, return, etc.
    assert!(tokens.len() > 20); // Should have many tokens
    assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    
    // Check some key tokens
    assert_eq!(tokens[0].token_type, TokenType::Fun);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].lexeme, "fibonacci");
}

#[test]
fn test_error_handling_integration() {
    let mut interpreter = Interpreter::new();
    
    // Test with normal source
    let had_error1 = interpreter.run_source("var x;".to_string());
    assert!(!had_error1);
    
    // Force an error state and check it resets
    interpreter.report(1, "test".to_string(), "test error".to_string());
    assert!(interpreter.has_error());
    
    // Running new source should reset error state
    let had_error2 = interpreter.run_source("var y;".to_string());
    assert!(!had_error2);
}

#[test]
fn test_complex_expression_parsing() {
    let source = "var result = (a + b) * c - d / e >= f && g || h != i;".to_string();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    
    // Verify we have the expected operator tokens
    let operator_tokens: Vec<&TokenType> = tokens.iter()
        .map(|t| &t.token_type)
        .filter(|t| matches!(t, 
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash |
            TokenType::GreaterEqual | TokenType::And | TokenType::Or | TokenType::BangEqual
        ))
        .collect();
        
    assert!(operator_tokens.len() >= 7); // Should have multiple operators
}

#[test]
fn test_string_and_number_literals() {
    let source = r#"var message = "Hello, World!"; var pi = 3.14159; var count = 42;"#.to_string();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    
    // Find the literal tokens
    let string_token = tokens.iter().find(|t| t.token_type == TokenType::String).unwrap();
    let number_tokens: Vec<_> = tokens.iter().filter(|t| t.token_type == TokenType::Number).collect();
    
    assert_eq!(string_token.literal, Some("Hello, World!".to_string()));
    assert_eq!(number_tokens.len(), 2);
    assert_eq!(number_tokens[0].literal, Some("3.14159".to_string()));
    assert_eq!(number_tokens[1].literal, Some("42".to_string()));
}

#[test]
fn test_multiline_program() {
    let source = r#"
// This is a multiline program
var x = 10;
var y = 20;

if (x < y) {
    print "x is smaller";
} else {
    print "y is smaller or equal";
}

for (var i = 0; i < 3; i = i + 1) {
    print i;
}
"#.to_string();

    let mut interpreter = Interpreter::new();
    let had_error = interpreter.run_source(source);
    assert!(!had_error);
}