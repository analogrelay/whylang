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
        public void TakeAddsNextCharacterToWindow() {
            var window = new TextWindow("Test Document");

            Assert.True(window.Take());

            Assert.Equal(new TextSpan(0, 1), window.Span);
            Assert.Equal(new [] {'T'}, window.Content.ToArray());
        }
    }
}