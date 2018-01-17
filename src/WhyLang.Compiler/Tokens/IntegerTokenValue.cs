namespace WhyLang.Compiler.Tokens
{
    public class IntegerTokenValue : TokenValue
    {
        public long Value { get; }

        public IntegerTokenValue(long value)
        {
            Value = value;
        }


        public override bool Equals(object obj) =>
            obj is TokenValue v && Equals(v);

        public override bool Equals(TokenValue other) =>
            other is IntegerTokenValue iv && iv.Value == Value;

        // We're required to implement this when we implement Equals
        public override int GetHashCode() => Value.GetHashCode();

        public override string ToString() => Value.ToString();
    }
}