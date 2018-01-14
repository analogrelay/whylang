using Xunit;

namespace WhyLang.Compiler.Text.Tests
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

            Assert.True(window.Take());
            Assert.Equal(new TextSpan(5, 1), window.Span);
            Assert.Equal(1, window.Content.Length);
        }

        [Fact]
        public void TakeIfDoesNothingAndReturnsFalseIfPredicateNotMatched()
        {
            var window = new TextWindow("Test");
            Assert.False(window.TakeIf(c => c == 'A'));
            Assert.Equal(new TextSpan(0, 0), window.Span);
            Assert.Empty(window.Content.ToArray());
        }

        [Fact]
        public void TakeIfAcceptsCharacterAndReturnsTrueIfPredicateMatched()
        {
            var window = new TextWindow("Test");
            Assert.True(window.TakeIf(c => c == 'T'));
            Assert.Equal(new TextSpan(0, 1), window.Span);
            Assert.Equal(new[] { 'T' }, window.Content.ToArray());
        }

        [Fact]
        public void TakeWhileAcceptsCharactersUntilPredicateIsNoLongerMatched()
        {
            var window = new TextWindow("0123456789abcedefg");
            window.Take();
            window.Advance();
            window.TakeWhile(c => char.IsDigit(c));
            Assert.Equal("123456789", window.GetString());
            Assert.Equal(new TextSpan(1, 9), window.Span);
        }

        [Fact]
        public void PeekReturnsNextCharacter()
        {
            var window = new TextWindow("0123456789abcedefg");
            window.Take();
            Assert.Equal('1', window.Peek());
        }

        [Fact]
        public void PeekReturnsNullAtEndOfWindow()
        {
            var window = new TextWindow("0");
            window.Take();
            Assert.Equal('\0', window.Peek());
        }

        [Fact]
        public void PeekReturnsTrueIfPredicateMatches()
        {
            var window = new TextWindow("01");
            window.Take();
            Assert.True(window.Peek(c => c == '1'));
        }

        [Fact]
        public void PeekReturnsFalseIfPredicateDoesNotMatch()
        {
            var window = new TextWindow("01");
            window.Take();
            Assert.False(window.Peek(c => c != '1'));
        }
    }
}