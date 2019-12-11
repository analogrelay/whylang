using WhyLang.Compiler.Tokens;
using Xunit;

namespace WhyLang.Compiler.Syntax
{
    public partial class SyntaxParserTests
    {
        public class Expressions
        {
            [Theory]
            [InlineData("42", 42L)]
            [InlineData("\"Hello, World!\"", "Hello, World!")]
            public void Constants(string text, object value)
            {
                var tokens = new Tokenizer(text);
                var parser = new SyntaxParser(tokens);
                var expr = parser.ParseExpression();
                Assert.Equal(new ConstantExpressionSyntax(value), expr);
            }

            [Theory]
            [InlineData("print(42)", "print", 42L)]
            public void Calls(string text, string functionName, object parameterValue)
            {
                var tokens = new Tokenizer(text);
                var parser = new SyntaxParser(tokens);
                var actual = parser.ParseExpression();
                var expected = new CallExpressionSyntax(
                    functionName,
                    new[] { new ConstantExpressionSyntax(parameterValue) });
                Assert.Equal(expected, actual);
            }
        }
    }
}