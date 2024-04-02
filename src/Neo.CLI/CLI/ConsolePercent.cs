// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// ConsolePercent.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;

namespace Neo.CLI
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
