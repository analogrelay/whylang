using Xunit;

namespace WhyLang.Compiler.Tokens.Tests
{
    public partial class TokenizerTests
    {
        public class Operators
        {
            [Theory]
            [InlineData("(", TokenType.LParen)]
            [InlineData(")", TokenType.RParen)]
            [InlineData(",", TokenType.Comma)]
            [InlineData("+", TokenType.Plus)]
            [InlineData("*", TokenType.Star)]
            [InlineData("/", TokenType.Slash)]
            [InlineData("=", TokenType.Assign)]
            public void OperatorsCanBeTokenized(string input, TokenType type)
            {
                SingleTokenTest(input, type);
            }
        }
    }
}