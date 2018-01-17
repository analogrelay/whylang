using System;

namespace WhyLang.Compiler.Syntax
{
    public abstract class SyntaxNode : IEquatable<SyntaxNode>
    {
        public abstract bool Equals(SyntaxNode other);
        public override bool Equals(object obj) => obj is SyntaxNode n && Equals(n);
        public override int GetHashCode() => base.GetHashCode();
    }
}