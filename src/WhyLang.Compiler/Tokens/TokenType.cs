namespace WhyLang.Compiler.Tokens
{
    public enum TokenType
    {
        Empty = 0,
        Unknown = 1,
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