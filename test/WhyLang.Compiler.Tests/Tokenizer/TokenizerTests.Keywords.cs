using Xunit;

namespace WhyLang.Compiler.Tokenizer.Tests
{
    public partial class TokenizerTests
    {
        public class Keywords
        {
            [Theory]
            [InlineData("def", TokenType.Def)]
            [InlineData("extern", TokenType.Extern)]
            public void KeywordsCanBeTokenized(string input, TokenType type)
            {
                SingleTokenTest(input, type);
            }
        }
    }
}
