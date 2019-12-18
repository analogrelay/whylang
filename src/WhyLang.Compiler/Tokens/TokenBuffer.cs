using System;

namespace WhyLang.Compiler.Tokens
{
    public class TokenBuffer
    {
        private readonly ITokenizer _tokenizer;

        private Token _peek = default;

        public Token Current { get; private set; }

        public TokenBuffer(Tokenizer tokenizer) : this((ITokenizer)tokenizer) { }

        internal TokenBuffer(ITokenizer tokenizer)
        {
            _tokenizer = tokenizer;
            _peek = _tokenizer.Next();
        }

        public Token Next()
        {
            Current = _peek;
            _peek = _tokenizer.Next();
            return Current;
        }

        public Token Peek()
        {
            return _peek;
        }

        public void Expect(TokenType expectedType)
        {
            if (Next().Type != expectedType)
            {
                throw new SyntaxException(Current.Location, $"Expected {expectedType}!");
            }
        }
    }
}