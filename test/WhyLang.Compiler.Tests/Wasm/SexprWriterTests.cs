using System;
using System.IO;
using WhyLang.Compiler.Wasm;
using Xunit;

namespace WhyLang.Compiler.Tests.Wasm
{
    public class SexprWriterTests
    {
        [Theory]
        [InlineData("foo", "foo")]
        [InlineData("42", "42")]
        [InlineData("\"hello\"", "\"hello\"")]
        public void SimpleAtom(string atom, string expected)
        {
            var actual = WriteToString(w =>
            {
                w.WriteAtom(atom);
            });
            Assert.Equal(expected, actual);
        }

        [Theory]
        [InlineData("foo", "(foo)")]
        [InlineData("42", "(42)")]
        [InlineData("\"hello\"", "(\"hello\")")]
        public void SexprWithAtom(string atom, string expected)
        {
            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.WriteAtom(atom);
                w.EndExpression();
            });
            Assert.Equal(expected, actual);
        }

        [Fact]
        public void SexprWithMultipleAtoms()
        {
            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.WriteAtom("a");
                w.WriteAtom("b");
                w.WriteAtom("c");
                w.EndExpression();
            });
            Assert.Equal("(a b c)", actual);
        }

        [Fact]
        public void NestedSexpr()
        {
            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.StartExpression();
                w.WriteAtom("a");
                w.WriteAtom("b");
                w.WriteAtom("c");
                w.EndExpression();
                w.EndExpression();
            });
            Assert.Equal("((a b c))", actual);
        }

        [Fact]
        public void AtomAfterNestedSexpr()
        {
            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.StartExpression();
                w.WriteAtom("a");
                w.WriteAtom("b");
                w.WriteAtom("c");
                w.EndExpression();
                w.WriteAtom("d");
                w.EndExpression();
            });
            Assert.Equal("((a b c) d)", actual);
        }

        [Fact]
        public void AtomBeforeNestedSexpr()
        {
            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.WriteAtom("a");
                w.StartExpression();
                w.WriteAtom("b");
                w.WriteAtom("c");
                w.WriteAtom("d");
                w.EndExpression();
                w.EndExpression();
            });
            Assert.Equal("(a (b c d))", actual);
        }

        [Fact]
        public void MultipleRootExprs()
        {
            var actual = WriteToString(w =>
            {
                w.WriteAtom("a");
                w.StartExpression();
                w.WriteAtom("b");
                w.EndExpression();
                w.WriteAtom("c");
            });
            Assert.Equal("a (b) c", actual);
        }

        [Fact]
        public void Indentation()
        {
            var expected = @"
(a b c
  (d e
    (f g) h)
  (i j))".TrimStart();

            var actual = WriteToString(w =>
            {
                w.StartExpression();
                w.WriteAtom("a");
                w.WriteAtom("b");
                w.WriteAtom("c");
                w.StartExpression();
                w.WriteAtom("d");
                w.WriteAtom("e");
                w.StartExpression();
                w.WriteAtom("f");
                w.WriteAtom("g");
                w.EndExpression();
                w.WriteAtom("h");
                w.EndExpression();
                w.StartExpression();
                w.WriteAtom("i");
                w.WriteAtom("j");
                w.EndExpression();
                w.EndExpression();
            }, indented: true);
            Assert.Equal(expected, actual);
        }

        private string WriteToString(Action<SexprWriter> action, bool indented = false)
        {
            var writer = new StringWriter();
            var sexprWriter = new SexprWriter(writer, indented);
            action(sexprWriter);
            return writer.ToString();
        }
    }
}
