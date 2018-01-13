using System;

namespace WhyLang.Compiler.Tokenizer
{
    public class Tokenizer
    {
        private readonly TextWindow _window;

        public Tokenizer(TextWindow window)
        {
            _window = window;
        }

        public Token Next()
        {
            if (!_window.Take())
            {
                // End of file
                return Emit(TokenType.EndOfFile);
            }

            switch(_window.Last) 
            {
                // Hrm, wish we didn't need the type pattern here :(
                case '-':
                case char c when char.IsDigit(c):
                    return Number();
                default:
                    return Emit(TokenType.Unknown);
            }
        }

        private Token Number()
        {
            throw new NotImplementedException();
        }

        private Token Emit(TokenType type) => Emit(type, TokenValue.Null);

        private Token Emit(TokenType type, TokenValue value)
        {
            var token = new Token(_window.Span, type, value);
            _window.Advance();
            return token;
        }
    }
}