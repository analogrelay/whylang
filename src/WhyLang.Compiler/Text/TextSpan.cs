using System;
using Microsoft.Extensions.Internal;

namespace WhyLang.Compiler
{
    public struct TextSpan : IEquatable<TextSpan>
    {
        public int Start { get; }
        public int Length { get; }
        public int End => Start + Length;

        public TextSpan(int start, int length)
        {
            Start = start;
            Length = length;
        }

        public override bool Equals(object obj) =>
            obj is TextSpan other && Equals(other);

        public bool Equals(TextSpan other)
        {
            return Start == other.Start &&
                Length == other.Length;
        }

        public override int GetHashCode()
        {
            var combiner = HashCodeCombiner.Start();
            combiner.Add(Start);
            combiner.Add(Length);
            return combiner;
        }

        public override string ToString()
        {
            return $"{Start}..{End}";
        }
    }
}