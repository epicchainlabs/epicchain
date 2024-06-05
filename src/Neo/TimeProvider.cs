// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TimeProvider.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;

namespace Neo
{
    /// <summary>
    /// The time provider for the NEO system.
    /// </summary>
    public class TimeProvider
    {
        private static readonly TimeProvider Default = new();

        /// <summary>
        /// The currently used <see cref="TimeProvider"/> instance.
        /// </summary>
        public static TimeProvider Current { get; internal set; } = Default;

        /// <summary>
        /// Gets the current time expressed as the Coordinated Universal Time (UTC).
        /// </summary>
        public virtual DateTime UtcNow => DateTime.UtcNow;

        internal static void ResetToDefault()
        {
            Current = Default;
        }
    }
}
