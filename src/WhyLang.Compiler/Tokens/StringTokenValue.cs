namespace WhyLang.Compiler.Tokens
{
    public class StringTokenValue : TokenValue
    {
        public string Value { get; }

        public StringTokenValue(string value)
        {
            Value = value;
        }

        public override bool Equals(object obj) =>
            obj is TokenValue v && Equals(v);

        public override bool Equals(TokenValue other) =>
            other is StringTokenValue iv &&
            string.Equals(iv.Value, Value, Constants.IdentifierComparison);

        // We're required to implement this when we implement Equals
        public override int GetHashCode() => Value.GetHashCode();

        public override string ToString() => $"\"{Value}\"";
    }
}