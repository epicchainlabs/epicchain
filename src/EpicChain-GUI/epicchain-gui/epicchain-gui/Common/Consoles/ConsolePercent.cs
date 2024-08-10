using System;

namespace Neo.Common.Consoles
{
    public class ConsolePercent : IDisposable
    {
        #region Variables

        private long _maxValue, _value;
        private decimal _lastFactor;
        private string _lastPercent;

        private readonly int _x, _y;

        private bool _inputRedirected;

        #endregion

        #region Properties

        /// <summary>
        /// Value
        /// </summary>
        public long Value
        {
            get { return _value; }
            set
            {
                if (value == _value) return;

                _value = Math.Min(value, _maxValue);
                Invalidate();
            }
        }

        /// <summary>
        /// Maximum value
        /// </summary>
        public long MaxValue
        {
            get { return _maxValue; }
            set
            {
                if (value == _maxValue) return;

                _maxValue = value;

                if (_value > _maxValue)
                    _value = _maxValue;

                Invalidate();
            }
        }

        /// <summary>
        /// Percent
        /// </summary>
        public decimal Percent
        {
            get
            {
                if (_maxValue == 0) return 0;
                return (_value * 100M) / _maxValue;
            }
        }

        #endregion

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="value">Value</param>
        /// <param name="maxValue">Maximum value</param>
        public ConsolePercent(long value = 0, long maxValue = 100)
        {
            _inputRedirected = Console.IsInputRedirected;
            _lastFactor = -1;
            _x = _inputRedirected ? 0 : Console.CursorLeft;
            _y = _inputRedirected ? 0 : Console.CursorTop;

            MaxValue = maxValue;
            Value = value;
            Invalidate();
        }

        /// <summary>
        /// Invalidate
        /// </summary>
        public void Invalidate()
        {
            var factor = Math.Round((Percent / 100M), 1);
            var percent = Percent.ToString("0.0").PadLeft(5, ' ');

            if (_lastFactor == factor && _lastPercent == percent)
            {
                return;
            }

            _lastFactor = factor;
            _lastPercent = percent;

            var fill = string.Empty.PadLeft((int)(10 * factor), '■');
            var clean = string.Empty.PadLeft(10 - fill.Length, _inputRedirected ? '□' : '■');

            if (_inputRedirected)
            {
                Console.WriteLine("[" + fill + clean + "] (" + percent + "%)");
            }
            else
            {
                Console.SetCursorPosition(_x, _y);

                var prevColor = Console.ForegroundColor;

                Console.ForegroundColor = ConsoleColor.White;
                Console.Write("[");
                Console.ForegroundColor = Percent > 50 ? ConsoleColor.Green : ConsoleColor.DarkGreen;
                Console.Write(fill);
                Console.ForegroundColor = ConsoleColor.White;
                Console.Write(clean + "] (" + percent + "%)");

                Console.ForegroundColor = prevColor;
            }
        }

        /// <summary>
        /// Free console
        /// </summary>
        public void Dispose()
        {
            Console.WriteLine("");
        }
    }
}
