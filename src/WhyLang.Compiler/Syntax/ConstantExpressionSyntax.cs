namespace WhyLang.Compiler.Syntax
{
    public class ConstantExpressionSyntax : ExpressionSyntax
    {
        public object Value { get; }

        public ConstantExpressionSyntax(object value)
        {
            Value = value;
        }

        public override bool Equals(SyntaxNode other)
        {
            return other is ConstantExpressionSyntax s &&
                Equals(s.Value, Value);
        }

        public override int GetHashCode() => Value.GetHashCode();

        public override string ToString()
        {
            return Value.ToString();
        }
    }
}