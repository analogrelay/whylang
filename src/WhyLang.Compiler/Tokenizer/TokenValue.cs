using System;

namespace WhyLang.Compiler.Tokenizer
{
    public abstract class TokenValue : IEquatable<TokenValue>
    {
        public static readonly TokenValue Null = NullTokenValue.Instance;

        public abstract bool Equals(TokenValue other);

        public static TokenValue Integer(long value) => new IntegerTokenValue(value);

        public static TokenValue Identifier(string value) => new IdentifierTokenValue(value);
    }
}