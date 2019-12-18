using System;
using System.Diagnostics;

namespace WhyLang.Compiler.Tokens
{
    public abstract class TokenValue : IEquatable<TokenValue>
    {
        public static readonly TokenValue Null = NullTokenValue.Instance;

        public abstract bool Equals(TokenValue other);

        public static TokenValue String(string value) => new StringTokenValue(value);

        public static TokenValue Integer(long value) => new IntegerTokenValue(value);

        public static TokenValue Identifier(string value) => new IdentifierTokenValue(value);
    }
}