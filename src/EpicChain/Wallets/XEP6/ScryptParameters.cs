// Copyright (C) 2021-2024 EpicChain Labs.

//
// ScryptParameters.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using EpicChain.Json;

namespace EpicChain.Wallets.XEP6
{
    /// <summary>
    /// Represents the parameters of the SCrypt algorithm.
    /// </summary>
    public class ScryptParameters
    {
        /// <summary>
        /// The default parameters used by <see cref="XEP6Wallet"/>.
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
