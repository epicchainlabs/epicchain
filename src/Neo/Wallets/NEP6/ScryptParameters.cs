// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ScryptParameters.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Json;

namespace Neo.Wallets.NEP6
{
    /// <summary>
    /// Represents the parameters of the SCrypt algorithm.
    /// </summary>
    public class ScryptParameters
    {
        /// <summary>
        /// The default parameters used by <see cref="NEP6Wallet"/>.
        /// </summary>
        public static ScryptParameters Default { get; } = new ScryptParameters(16384, 8, 8);

        /// <summary>
        /// CPU/Memory cost parameter. Must be larger than 1, a power of 2 and less than 2^(128 * r / 8).
        /// </summary>
        public readonly int N;

        /// <summary>
        /// The block size, must be >= 1.
        /// </summary>
        public readonly int R;

        /// <summary>
        /// Parallelization parameter. Must be a positive integer less than or equal to Int32.MaxValue / (128 * r * 8).
        /// </summary>
        public readonly int P;

        /// <summary>
        /// Initializes a new instance of the <see cref="ScryptParameters"/> class.
        /// </summary>
        /// <param name="n">CPU/Memory cost parameter.</param>
        /// <param name="r">The block size.</param>
        /// <param name="p">Parallelization parameter.</param>
        public ScryptParameters(int n, int r, int p)
        {
            N = n;
            R = r;
            P = p;
        }

        /// <summary>
        /// Converts the parameters from a JSON object.
        /// </summary>
        /// <param name="json">The parameters represented by a JSON object.</param>
        /// <returns>The converted parameters.</returns>
        public static ScryptParameters FromJson(JObject json)
        {
            return new ScryptParameters((int)json["n"].AsNumber(), (int)json["r"].AsNumber(), (int)json["p"].AsNumber());
        }

        /// <summary>
        /// Converts the parameters to a JSON object.
        /// </summary>
        /// <returns>The parameters represented by a JSON object.</returns>
        public JObject ToJson()
        {
            JObject json = new();
            json["n"] = N;
            json["r"] = R;
            json["p"] = P;
            return json;
        }
    }
}
