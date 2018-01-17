namespace WhyLang.Compiler.Tokens
{
    public class NullTokenValue : TokenValue
    {
        public static readonly NullTokenValue Instance = new NullTokenValue();

        private NullTokenValue()
        {
        }

        public override bool Equals(object obj) =>
            obj is TokenValue v && Equals(v);

        public override bool Equals(TokenValue other) => 
            ReferenceEquals(other, Instance);

        // We're required to implement this when we implement Equals
        public override int GetHashCode() => 0;

        public override string ToString() => string.Empty;
    }
}