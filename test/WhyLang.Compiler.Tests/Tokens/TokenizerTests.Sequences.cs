using System.Linq;
using Xunit;

namespace WhyLang.Compiler.Tokens.Tests
{
    public partial class TokenizerTests
    {
        public class Sequences
        {
            [Fact]
            public void NumbersOperatorsAndIdentifiers()
            {
                var tokens = Tokenizer.Tokenize("1 + x / def(extern)").ToArray();
                Assert.Equal(new[] {
                    new Token(new TextSpan(0, 1), TokenType.Integer, TokenValue.Integer(1)),
                    new Token(new TextSpan(2, 1), TokenType.Plus, TokenValue.Null),
                    new Token(new TextSpan(4, 1), TokenType.Identifier, TokenValue.Identifier("x")),
                    new Token(new TextSpan(6, 1), TokenType.Slash, TokenValue.Null),
                    new Token(new TextSpan(8, 3), TokenType.Def, TokenValue.Null),
                    new Token(new TextSpan(11, 1), TokenType.LParen, TokenValue.Null),
                    new Token(new TextSpan(12, 6), TokenType.Extern, TokenValue.Null),
                    new Token(new TextSpan(18, 1), TokenType.RParen, TokenValue.Null),
                }, tokens);
            }
        }
    }
}