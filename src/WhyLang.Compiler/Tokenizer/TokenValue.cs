using System;

namespace WhyLang.Compiler.Tokenizer
{
    public abstract class TokenValue : IEquatable<TokenValue>
    {
        public static readonly TokenValue Null = NullTokenValue.Instance;

        public abstract bool Equals(TokenValue other);
    }
}