using Xunit;

namespace WhyLang.Compiler.Tokens.Tests
{
    public partial class TokenizerTests
    {
        public class Identifiers
        {
            // TODO: More complex identifiers!

            [Theory]
            [InlineData("test")]
            [InlineData("_42")]
            [InlineData("_foo_42_bar")]
            public void IdentifiersCanBeTokenized(string input)
            {
                SingleTokenTest(input, TokenType.Identifier, TokenValue.Identifier(input));
            }
        }
    }
}