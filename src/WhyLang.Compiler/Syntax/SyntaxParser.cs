using System;
using System.Collections.Generic;
using System.Diagnostics;
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
            _tokens.Next();
            switch (_tokens.Current.Type)
            {
                case TokenType.Integer:
                case TokenType.String:
                    return ConstantExpression();
                case TokenType.Identifier:
                    return CallExpression();
                default:
                    throw new SyntaxException(_tokens.Current.Location, $"Unexpected {_tokens.Current.Type}.");
            }
        }

        private ExpressionSyntax CallExpression()
        {
            Debug.Assert(_tokens.Current.Type == TokenType.Identifier);
            Debug.Assert(_tokens.Current.Value is IdentifierTokenValue);
            var function = ((IdentifierTokenValue)_tokens.Current.Value).Value;

            // Consume Argument list
            _tokens.Expect(TokenType.LParen);
            var arguments = ArgumentList();
            _tokens.Expect(TokenType.RParen);

            return new CallExpressionSyntax(function, arguments);
        }

        private IReadOnlyList<ExpressionSyntax> ArgumentList()
        {
            var expressions = new List<ExpressionSyntax>();
            do
            {
                expressions.Add(ParseExpression());
            } while (_tokens.Peek().Type == TokenType.Comma);
            return expressions;
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