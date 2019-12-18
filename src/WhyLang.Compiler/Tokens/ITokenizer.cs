namespace WhyLang.Compiler.Tokens
{
    // Isn't this just IEnumerable<Token> you may ask? Well yes and no. It's really more IEnumerator<Token>
    // Plus there may be additional things we want to hang off of this. Also it returns an EOF token at the end.
    internal interface ITokenizer
    {
        Token Next();
    }
}