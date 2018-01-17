using WhyLang.Compiler.Tokens;
using Xunit;

namespace WhyLang.Compiler.Syntax
{
    public partial class SyntaxParserTests
    {
        public class Expressions
        {
            [Theory]
            [InlineData("42", 42)]
            public void Constants(string text, int value)
            {
                var tokens = new Tokenizer(text);
                var parser = new SyntaxParser(tokens);
                var expr = parser.ParseExpression();
                Assert.Equal(new ConstantExpressionSyntax(value), expr);
            }
        }
    }
}