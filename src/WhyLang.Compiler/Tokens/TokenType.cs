namespace WhyLang.Compiler.Tokens
{
    public enum TokenType
    {
        Unknown,
        EndOfFile,
        Integer,
        Identifier,
        Def,
        Extern,
        LParen,
        RParen,
        Comma,
        Plus,
        Minus,
        Star,
        String,
        Slash,
        Assign,
    }
}