using System;
using System.Collections.Generic;

namespace WhyLang.Compiler.Tokenizer
{
    public class Tokenizer
    {
        private readonly TextWindow _window;

        private static readonly Dictionary<string, TokenType> _keywords = new Dictionary<string, TokenType>()
        {
            {"def", TokenType.Def},
            {"extern", TokenType.Extern}
        };

        public Tokenizer(string input) : this(new TextWindow(input))
        {
        }

        public Tokenizer(TextWindow window)
        {
            _window = window;
        }

        public static IEnumerable<Token> Tokenize(string input)
        {
            var tokenizer = new Tokenizer(input);
            Token token;
            while ((token = tokenizer.Next()).Type != TokenType.EndOfFile)
            {
                yield return token;
            }
        }

        public Token Next()
        {
            _window.SkipWhile(c => char.IsWhiteSpace(c));

            if (!_window.Take())
            {
                // End of file
                return Emit(TokenType.EndOfFile);
            }

            switch (_window.Last)
            {
                // Hrm, wish we didn't need the type pattern here :(
                case '-' when _window.Peek(x => char.IsDigit(x)):
                case char c when char.IsDigit(c):
                    return Number();
                case '_':
                case char c when char.IsLetter(c):
                    return Identifier();
                case '(': return Emit(TokenType.LParen);
                case ')': return Emit(TokenType.RParen);
                case ',': return Emit(TokenType.Comma);
                case '+': return Emit(TokenType.Plus);
                case '*': return Emit(TokenType.Star);
                case '/': return Emit(TokenType.Slash);
                case '=': return Emit(TokenType.Assign);
                default:
                    return Emit(TokenType.Unknown);
            }
        }

        private Token Identifier()
        {
            _window.TakeWhile(c => char.IsLetterOrDigit(c) || c == '_');
            var ident = _window.GetString();

            // Check if it's a keyword
            if (_keywords.TryGetValue(ident, out var type))
            {
                return Emit(type, TokenValue.Null);
            }
            else
            {
                return Emit(
                    type: TokenType.Identifier,
                    value: TokenValue.Identifier(ident));
            }
        }

        private Token Number()
        {
            _window.TakeWhile(c => char.IsDigit(c));
            return Emit(
                type: TokenType.Integer,
                // Can't use Span<char>-native Int32.Parse in netstandard2.0 :(
                value: TokenValue.Integer(Int32.Parse(_window.GetString())));
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