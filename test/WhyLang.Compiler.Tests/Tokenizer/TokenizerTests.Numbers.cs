using Xunit;

namespace WhyLang.Compiler.Tokenizer.Tests
{
    public partial class TokenizerTests
    {
        public class Numbers
        {
            [Theory]
            [InlineData("0", 0)]
            [InlineData("42", 42)]
            [InlineData("00001", 1)]
            [InlineData("-0", 0)]
            [InlineData("-42", -42)]
            [InlineData("-00001", -1)]
            public void IntegersCanBeTokenized(string input, int value)
            {
                SingleTokenTest(input, TokenType.Integer, TokenValue.Integer(value));
            }
        }
    }
}