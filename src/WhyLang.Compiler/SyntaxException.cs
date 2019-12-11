namespace WhyLang.Compiler
{
    // TODO: Replace with proper diagnostics.
    public class SyntaxException : System.Exception
    {
        public SyntaxException(TextSpan location)
        {
            Location = location;
        }

        public SyntaxException(TextSpan location, string message) : base(message)
        {
            Location = location;
        }

        public SyntaxException(TextSpan location, string message, System.Exception inner) : base(message, inner)
        {
            Location = location;
        }

        public TextSpan Location { get; }
    }
}