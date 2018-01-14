using System;
using System.Diagnostics;

namespace WhyLang.Compiler
{
    public class TextWindow
    {
        private string _buffer;
        private int _start;
        private int _length;

        private int End => _start + _length;

        public ReadOnlySpan<char> Content => _buffer.AsSpan().Slice(_start, _length);
        public TextSpan Span => new TextSpan(_start, _length);
        public char Last => Content.Length > 0 ? Content[Content.Length - 1] : '\0';

        public TextWindow(string buffer)
        {
            _buffer = buffer;
            _start = 0;
            _length = 0;
        }

        /// <summary>
        /// Advance the window so that the end point becomes the start point.
        /// </summary>
        public void Advance()
        {
            // Nothing to check here. If there aren't any more characters, Take will return false.
            _start += _length;
            _length = 0;
        }

        /// <summary>
        /// Returns the next character that would be read by <see cref="Take" /> or a null character ('\0')
        /// if there are no further characters.
        /// </summary>
        public char Peek() => End < _buffer.Length ? _buffer[End] : '\0';

        /// <summary>
        /// Returns true if the next character matches the provided predicate
        /// </summary>
        public bool Peek(Func<char, bool> predicate) => predicate(Peek());

        /// <summary>
        /// Creates a new string containing the characters currently in the window
        /// </summary>
        /// <remarks>
        /// This allocates a new string EACH time it is called
        /// </remarks>
        public string GetString()
        {
            return new string(Content.ToArray());
        }

        /// <summary>
        /// Accepts a new character into the window, returning a boolean indicating if there
        /// was another character to accept
        /// <summary>
        public bool Take() => TakeIf(_ => true);

        /// <summary>
        /// Accepts a new character into the window, but only if it matches the provided predicate
        /// </summary>
        public bool TakeIf(Func<char, bool> predicate)
        {
            if (Peek(c => c != '\0' && predicate(c)))
            {
                _length += 1;
                return true;
            }
            return false;
        }

        /// <summary>
        /// Accepts characters as long as they are matching the provided predicate.
        /// </summary>
        public void TakeWhile(Func<char, bool> predicate)
        {
            while (TakeIf(predicate))
            {
                // Nothing to do in the body :)
            }
        }

        /// <summary>
        /// Skips characters as long as they match the provided predicate
        /// </summary>
        /// <remarks>
        /// This can only be called when the window is empty
        /// </remarks>
        public void SkipWhile(Func<char, bool> predicate)
        {
            if (_length != 0)
            {
                throw new InvalidOperationException("Cannot use SkipWhile when the window has text in it");
            }

            TakeWhile(predicate);
            Advance();
        }
    }
}