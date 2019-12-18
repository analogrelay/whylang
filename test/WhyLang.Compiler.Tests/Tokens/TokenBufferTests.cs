using System.Collections.Generic;
using WhyLang.Compiler.Tokens;
using Xunit;

namespace WhyLang.Compiler.Tests.Tokens
{
    public class TokenBufferTests
    {
        [Fact]
        public void AfterInitializationNextReturnsFirstToken()
        {
            var first = new Token(new TextSpan(0, 1), TokenType.LParen, TokenValue.Null);
            var tokenizer = new TestTokenizer(first);
            var buffer = new TokenBuffer(tokenizer);
            Assert.Equal(default(Token), buffer.Current);
            Assert.Equal(first, buffer.Next());
        }

        [Fact]
        public void NextSetsCurrentProperty()
        {
            var first = new Token(new TextSpan(0, 1), TokenType.LParen, TokenValue.Null);
            var tokenizer = new TestTokenizer(first);
            var buffer = new TokenBuffer(tokenizer);
            Assert.Equal(first, buffer.Next());
            Assert.Equal(first, buffer.Current);
        }

        [Fact]
        public void NextReturnsEofTokenAtEnd()
        {
            var first = new Token(new TextSpan(0, 1), TokenType.LParen, TokenValue.Null);
            var tokenizer = new TestTokenizer(first);
            var buffer = new TokenBuffer(tokenizer);
            Assert.Equal(first, buffer.Next());
            Assert.Equal(TokenType.EndOfFile, buffer.Next().Type);
            Assert.Equal(TokenType.EndOfFile, buffer.Next().Type);
            Assert.Equal(TokenType.EndOfFile, buffer.Next().Type);
        }

        [Fact]
        public void NextMovesToNextTokenInBuffer()
        {
            var first = new Token(new TextSpan(0, 1), TokenType.LParen, TokenValue.Null);
            var second = new Token(new TextSpan(1, 1), TokenType.RParen, TokenValue.Null);
            var third = new Token(new TextSpan(2, 1), TokenType.Comma, TokenValue.Null);
            var tokenizer = new TestTokenizer(first, second, third);
            var buffer = new TokenBuffer(tokenizer);
            Assert.Equal(first, buffer.Next());
            Assert.Equal(second, buffer.Next());
            Assert.Equal(third, buffer.Next());
        }

        [Fact]
        public void PeekShowsNextTokenWithoutConsuming()
        {
            var first = new Token(new TextSpan(0, 1), TokenType.LParen, TokenValue.Null);
            var second = new Token(new TextSpan(1, 1), TokenType.RParen, TokenValue.Null);
            var third = new Token(new TextSpan(2, 1), TokenType.Comma, TokenValue.Null);
            var tokenizer = new TestTokenizer(first, second, third);
            var buffer = new TokenBuffer(tokenizer);
            Assert.Equal(first, buffer.Peek());
            Assert.Equal(first, buffer.Next());
            Assert.Equal(second, buffer.Peek());
            Assert.Equal(first, buffer.Current);
        }
    }

    public class TestTokenizer : ITokenizer
    {
        private readonly IEnumerator<Token> _tokens;

        public TestTokenizer(params Token[] tokens): this((IEnumerable<Token>)tokens)
        {
        }

        public TestTokenizer(IEnumerable<Token> tokens)
        {
            _tokens = tokens.GetEnumerator();
        }

        public Token Next()
        {
            if(_tokens.MoveNext())
            {
                return _tokens.Current;
            }
            return new Token(
                new TextSpan(0, 0),
                TokenType.EndOfFile,
                TokenValue.Null);
        }
    }
}
