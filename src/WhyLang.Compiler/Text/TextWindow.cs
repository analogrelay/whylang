using System;
using System.Diagnostics;

namespace WhyLang.Compiler
{
    [DebuggerDisplay("{DebuggerDisplay,nq}")]
    public class TextWindow
    {
        private string _buffer;
        private int _start;
        private int _length;

        private int End => _start + _length;

        public ReadOnlySpan<char> Content => _buffer.AsSpan().Slice(_start, _length);
        public TextSpan Span => new TextSpan(_start, _length);
        public char Last => Content.Length > 0 ? Content[Content.Length - 1] : '\0';

        private string DebuggerDisplay
        {
            get
            {
                var prefixLen = Math.Min(3, _start);
                var prefix = prefixLen > 0 ? _buffer.Substring(_start - prefixLen, prefixLen) : string.Empty;
                if(_start > 3)
                {
                    prefix = $"…{prefix}";
                }
                var content = GetString();
                var suffixLen = Math.Min(3, _buffer.Length - _start - _length);
                var suffix = suffixLen > 0 ? _buffer.Substring(_start + _length, suffixLen) : string.Empty;
                if(_buffer.Length - _start - _length > 3)
                {
                    suffix = $"{suffix}…";
                }
                return $"«{prefix}¦{content}¦{suffix}» ({Span})";
            }
        }

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
        public string GetString() => new string(Content);

        /// <summary>
        /// Accepts a new character into the window, returning a boolean indicating if there
        /// was another character to accept
        /// <summary>
        public bool Take() => TakeIf(_ => true);

        /// <summary>
        /// Accepts a new character into the window, but only if it matches the provided character
        /// </summary>
        public bool TakeIf(char expected) => TakeIf(c => c == expected);

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
        /// Accepts characters until the end of file, or when the provided characeter is reached.
        /// </summary>
        public void TakeUntil(char chr) => TakeUntil(c => c == chr);

        /// <summary>
        /// Accepts characters until one matches the provided predicate
        /// </summary>
        public void TakeUntil(Func<char, bool> predicate) => TakeWhile(c => !predicate(c));

        /// <summary>
        /// Accepts a sequence of the provided character
        /// </summary>
        public void TakeWhile(char chr) => TakeWhile(c => c == chr);

        /// <summary>
        /// Accepts characters as long as they match the provided predicate.
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

        private void FillBuffer(Span<char> destination)
        {
            Debug.Assert(destination.Length == _length);
            _buffer.AsSpan().Slice(_start, _length).CopyTo(destination);
        }
    }
}