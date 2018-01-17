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
                    return ConstantExpression();
                default:
                    _tokens.Next();
                    throw new Exception($"Unexpected {_tokens.Current.Type}! TODO: Error recovery!");
            }
        }

        private ConstantExpressionSyntax ConstantExpression()
        {
            if (_tokens.Current.Value is IntegerTokenValue i)
            {
                return new ConstantExpressionSyntax(i.Value);
            }
            else
            {
                throw new Exception("Unexpected non-numeric constant! TODO: Error recovery!");
            }
        }
    }
}