namespace WhyLang.Compiler.Tokens
{
    public class TokenBuffer
    {
        private readonly Tokenizer _tokenizer;

        public Token Current { get; private set; }

        public TokenBuffer(Tokenizer tokenizer)
        {
            _tokenizer = tokenizer;

            Next();
        }

        public bool Next()
        {
            Current = _tokenizer.Next();
            return Current.Type != TokenType.EndOfFile;
        }
    }
}