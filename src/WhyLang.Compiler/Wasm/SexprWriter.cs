using System;
using System.IO;

namespace WhyLang.Compiler.Wasm
{
    internal class SexprWriter
    {
        private readonly TextWriter _writer;
        private bool _startExpr = true;

        private int _indent = 0;

        public bool Indented { get; }

        public SexprWriter(TextWriter writer, bool indented)
        {
            _writer = writer;
            Indented = indented;
        }

        public void StartExpression()
        {
            if (Indented)
            {
                WriteIndent();
            }
            else if(!_startExpr)
            {
                _writer.Write(" ");
            }

            _writer.Write("(");
            _startExpr = true;
            _indent += 1;
        }

        private void WriteIndent()
        {
            if (_indent > 0)
            {
                _writer.Write(new string(' ', 2 * _indent));
            }
        }

        public void EndExpression()
        {
            _writer.Write(")");
            _indent -= 1;
        }

        public void WriteAtom(string atom)
        {
            if(!_startExpr)
            {
                _writer.Write(" ");
            }

            _writer.Write(atom);
            _startExpr = false;
        }
    }
}
