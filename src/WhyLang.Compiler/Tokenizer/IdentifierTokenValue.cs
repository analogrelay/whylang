namespace WhyLang.Compiler.Tokenizer
{
    public class IdentifierTokenValue : TokenValue
    {
        public string Value { get; }

        public IdentifierTokenValue(string value)
        {
            Value = value;
        }

        public override bool Equals(object obj) =>
            obj is TokenValue v && Equals(v);

        public override bool Equals(TokenValue other) =>
            other is IdentifierTokenValue iv && 
            string.Equals(iv.Value, Value, Constants.IdentifierComparison);

        // We're required to implement this when we implement Equals
        public override int GetHashCode() => Value.GetHashCode();

        public override string ToString() => Value;
    }
}