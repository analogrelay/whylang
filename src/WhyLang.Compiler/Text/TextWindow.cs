using System;

namespace WhyLang.Compiler
{
    public class TextWindow
    {
        private string _buffer;
        private int _start;
        private int _length;

        public ReadOnlySpan<char> Content => _buffer.AsSpan().Slice(_start, _length);
        public TextSpan Span => new TextSpan(_start, _length);

        /// <summary>
        /// Accepts a new character into the window, returning a boolean indicating if there
        /// was another character to accept
        /// <summary>
        public bool Take()
        {
            var newLength = _length + 1;
            if (newLength + _start > _buffer.Length)
            {
                return false;
            }
            else
            {
                _length = newLength;
                return true;
            }
        }

        /// <summary>
        /// Advance the window so that the end point becomes the start point.
        /// </summary>
        public void Advance()
        {
            // Nothing to check here. If there aren't any more characters, Take will return false.
            _start = _start + _length;
            _length = 0;
        }
    }
}