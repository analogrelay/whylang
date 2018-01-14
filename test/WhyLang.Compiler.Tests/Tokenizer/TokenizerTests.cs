using System.Linq;
using Xunit;

namespace WhyLang.Compiler.Tokenizer.Tests
{
    public partial class TokenizerTests
    {
        public static void SingleTokenTest(string input, TokenType expectedType, TokenValue expectedValue)
        {
            var tokens = Tokenizer.Tokenize(" \t\r\n" + input + "\n\t ");
            var expectedToken = new Token(new TextSpan(4, input.Length), expectedType, expectedValue);
            Assert.Equal(new [] { expectedToken }, tokens.ToArray());
        }
    }
}