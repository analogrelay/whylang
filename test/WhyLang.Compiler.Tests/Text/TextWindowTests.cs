using Xunit;

namespace WhyLang.Compiler.Tests
{
    public class TextWindowTests
    {
        [Fact]
        public void WhenInitializedTheWindowIsEmpty()
        {
            var window = new TextWindow("Test Document");

            Assert.Equal(new TextSpan(0, 0), window.Span);
            Assert.Equal(0, window.Content.Length);
        }

        [Fact]
        public void TakeAddsNextCharacterToWindow()
        {
            var window = new TextWindow("Test Document");

            Assert.True(window.Take());

            Assert.Equal(new TextSpan(0, 1), window.Span);
            Assert.Equal(new[] { 'T' }, window.Content.ToArray());
        }

        [Theory]
        [InlineData("")]
        [InlineData("Foo")]
        public void TakeReturnsFalseWhenNoMoreCharactersToAdd(string str)
        {
            var window = new TextWindow(str);
            for (var i = 0; i < str.Length; i += 1)
            {
                Assert.True(window.Take());
            }
            Assert.False(window.Take());
        }

        [Fact]
        public void AdvanceMovesWindowUpToEnd()
        {
            var window = new TextWindow("This is a test");
            Assert.True(window.Take());
            Assert.True(window.Take());
            Assert.True(window.Take());
            Assert.True(window.Take());
            Assert.True(window.Take());

            Assert.Equal("This ", window.GetString());

            window.Advance();

            Assert.Equal(new TextSpan(5, 0), window.Span);
            Assert.Equal(0, window.Content.Length);
        }
    }
}