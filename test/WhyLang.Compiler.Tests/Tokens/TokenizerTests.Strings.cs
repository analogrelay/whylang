using System.Linq;
using Xunit;

namespace WhyLang.Compiler.Tokens.Tests
{
    public partial class TokenizerTests
    {
        public class Strings
        {
            [Theory]
            [InlineData("\"\"", "")]
            [InlineData("\"abc\"", "abc")]
            public void StringsCanBeTokenized(string input, string value)
            {
                SingleTokenTest(input, TokenType.String, TokenValue.String(value));
            }

            [Theory]
            [InlineData("\"foo", 4, "Unexpected end-of-file")]
            [InlineData("\"foo\n", 4, "Unexpected new line")]
            [InlineData("\"foo\r\n", 4, "Unexpected new line")]
            public void InvalidStrings(string input, int length, string message)
            {
                var span = new TextSpan(0, length);
                var ex = Assert.Throws<SyntaxException>(() => Tokenizer.Tokenize(input).ToList());
                Assert.Equal(span, ex.Location);
                Assert.Equal(message,ex.Message);
            }
        }
    }
}