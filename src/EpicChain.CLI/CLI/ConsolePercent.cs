// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: ConsolePercent.cs
//  Project: EpicChain Labs - Core Blockchain Infrastructure
//  Author: Xmoohad (Muhammad Ibrahim Muhammad)
//
// ---------------------------------------------------------------------------------------------
//  Description:
//  This file is an integral part of the EpicChain Labs ecosystem, a forward-looking, open-source
//  blockchain initiative founded by Xmoohad. The EpicChain project aims to create a robust,
//  decentralized, and developer-friendly blockchain infrastructure that empowers innovation,
//  transparency, and digital sovereignty.
//
// ---------------------------------------------------------------------------------------------
//  Licensing:
//  This file is distributed under the permissive MIT License, which grants anyone the freedom
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this
//  software. These rights are granted with the understanding that the original license notice
//  and copyright attribution remain intact.
//
//  For the full license text, please refer to the LICENSE file included in the root directory of
//  this repository or visit the official MIT License page at:
//  ➤ https://opensource.org/licenses/MIT
//
// ---------------------------------------------------------------------------------------------
//  Community and Contribution:
//  EpicChain Labs is deeply rooted in the principles of open-source development. We believe that
//  collaboration, transparency, and inclusiveness are the cornerstones of sustainable technology.
//
//  This file, like all components of the EpicChain ecosystem, is offered to the global development
//  community to explore, extend, and improve. Whether you're fixing bugs, optimizing performance,
//  or building new features, your contributions are welcome and appreciated.
//
//  By contributing to this project, you become part of a community dedicated to shaping the future
//  of blockchain technology. Join us in our mission to create more secure, scalable, and accessible
//  digital infrastructure for all.
//
// ---------------------------------------------------------------------------------------------
//  Terms of Use:
//  Redistribution and usage of this file in both source and compiled (binary) forms—with or without
//  modification—are fully permitted under the MIT License. Users of this software are expected to
//  adhere to the simple and clear guidelines established in the LICENSE file.
//
//  By using this file and other components of the EpicChain Labs project, you acknowledge and agree
//  to the terms of the MIT License. This ensures that the ethos of free and open software development
//  continues to flourish and remain protected.
//
// ---------------------------------------------------------------------------------------------
//  Final Note:
//  EpicChain Labs remains committed to pushing the boundaries of blockchain innovation. Whether
//  you're an experienced developer, a researcher, a student, or simply a curious enthusiast, we
//  invite you to explore the possibilities of EpicChain—and contribute toward a decentralized future.
//
//  Learn more about the project, get involved, or access full documentation at:
//  ➤ https://epic-chain.org
//
// =============================================================================================



using System;

namespace EpicChain.CLI
{
    public class ConsolePercent : IDisposable
    {
        #region Variables

        private readonly long _maxValue;
        private long _value;
        private decimal _lastFactor;
        private string? _lastPercent;

        private readonly int _x, _y;

        private readonly bool _inputRedirected;

        #endregion

        #region Properties

        /// <summary>
        /// Value
        /// </summary>
        public long Value
        {
            get => _value;
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
            get => _maxValue;
            init
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
            var factor = Math.Round(Percent / 100M, 1);
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
