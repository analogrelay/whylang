namespace WhyLang.Compiler.Tokenizer
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
        Slash,
        Assign,
    }
}