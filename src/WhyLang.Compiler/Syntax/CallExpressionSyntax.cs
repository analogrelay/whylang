using System;
using System.Collections.Generic;
using System.Linq;

namespace WhyLang.Compiler.Syntax
{
    public class CallExpressionSyntax : ExpressionSyntax
    {
        // TODO: Some kind of symbol table with scoping.
        public string Function { get; }
        public IReadOnlyList<ExpressionSyntax> Arguments { get; }

        public CallExpressionSyntax(string function, IEnumerable<ExpressionSyntax> arguments)
        {
            Function = function;
            Arguments = arguments.ToList();
        }

        public override bool Equals(SyntaxNode other)
        {
            return other is CallExpressionSyntax s &&
                Equals(s.Function, Function) &&
                Enumerable.SequenceEqual(s.Arguments, Arguments);
        }

        public override int GetHashCode() => HashCode.Combine(Function, Arguments);

        public override string ToString() => $"{Function}({string.Join(", ", Arguments.Select(a => a.ToString()))})";
    }
}