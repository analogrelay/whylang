using System;
using WhyLang.Compiler.Tokens;

namespace WhyLang.Compiler.Syntax
{
    public class SyntaxParser
    {
        private readonly TokenBuffer _tokens;

        public SyntaxParser(Tokenizer tokens) : this(new TokenBuffer(tokens))
        {
        }

        public SyntaxParser(TokenBuffer tokens)
        {
            _tokens = tokens;
        }

        public ExpressionSyntax ParseExpression()
        {
            switch (_tokens.Current.Type)
            {
                case TokenType.Integer:
                case TokenType.String:
                    return ConstantExpression();
                case TokenType.Identifier:
                    return CallExpression();
                default:
                    _tokens.Next();
                    throw new SyntaxException(_tokens.Current.Location, $"Unexpected {_tokens.Current.Type}.");
            }
        }

        private ExpressionSyntax CallExpression()
        {
            throw new NotImplementedException();
        }

        private ConstantExpressionSyntax ConstantExpression()
        {
            return _tokens.Current.Value switch
            {
                IntegerTokenValue i => new ConstantExpressionSyntax(i.Value),
                StringTokenValue s => new ConstantExpressionSyntax(s.Value),
                _ => throw new SyntaxException(_tokens.Current.Location, "Unexpected constant type."),
            };
        }
    }
}