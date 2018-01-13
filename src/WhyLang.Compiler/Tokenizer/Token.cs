using System;
using Microsoft.Extensions.Internal;

namespace WhyLang.Compiler.Tokenizer
{
    public struct Token : IEquatable<Token>
    {
        public TextSpan Location { get; }
        public TokenType Type { get; }
        public TokenValue Value { get; }

        public Token(TextSpan location, TokenType type, TokenValue value)
        {
            Location = location;
            Type = type;
            Value = value;
        }

        public override bool Equals(object obj) => obj is Token t && Equals(t);

        public bool Equals(Token other)
        {
            return Equals(Location, other.Location) &&
                Type == other.Type &&
                Equals(Value, other.Value);
        }

        public override int GetHashCode()
        {
            var hash = new HashCodeCombiner();
            hash.Add(Location);
            hash.Add(Type);
            hash.Add(Value);
            return hash;
        }
    }
}