use text::TextSpan;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    Unknown,
    Number,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenValue {
    None,
    Integer(i64),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Token {
    span: TextSpan,
    typ: TokenType,
    value: TokenValue,
}

impl Token {
    pub fn new(span: TextSpan, typ: TokenType, value: TokenValue) -> Token {
        Token {
            span,
            typ,
            value
        }
    }

    pub fn span(&self) -> TextSpan { self.span }
    pub fn typ(&self) -> TokenType { self.typ }
    pub fn value(&self) -> &TokenValue { &self.value }
    pub fn text<'a>(&self, document: &'a str) -> &'a str {
        &document[self.span.start() as usize..self.span.end() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use text::TextSpan;

    #[test]
    pub fn text_gets_the_text_span_from_the_provided_document() {
        let doc = "this is a test";
        let tok = Token::new(TextSpan::new(5, 9), TokenType::Unknown, TokenValue::None);
        assert_eq!("is a", tok.text(doc));
    }
}