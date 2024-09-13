// Copyright (C) 2021-2024 EpicChain Labs.

//
// Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Linq;
using System.Net;

namespace EpicChain.Plugins.OracleService
{
    static class Helper
    {
        public static bool IsInternal(this IPHostEntry entry)
        {
            return entry.AddressList.Any(p => IsInternal(p));
        }

        /// <summary>
        ///       ::1          -   IPv6  loopback
        ///       10.0.0.0     -   10.255.255.255  (10/8 prefix)
        ///       127.0.0.0    -   127.255.255.255  (127/8 prefix)
        ///       172.16.0.0   -   172.31.255.255  (172.16/12 prefix)
        ///       192.168.0.0  -   192.168.255.255 (192.168/16 prefix)
        /// </summary>
        /// <param name="ipAddress">Address</param>
        /// <returns>True if it was an internal address</returns>
        public static bool IsInternal(this IPAddress ipAddress)
        {
            if (IPAddress.IsLoopback(ipAddress)) return true;
            if (IPAddress.Broadcast.Equals(ipAddress)) return true;
            if (IPAddress.Any.Equals(ipAddress)) return true;
            if (IPAddress.IPv6Any.Equals(ipAddress)) return true;
            if (IPAddress.IPv6Loopback.Equals(ipAddress)) return true;

            var ip = ipAddress.GetAddressBytes();
            switch (ip[0])
            {
                case 10:
                case 127: return true;
                case 172: return ip[1] >= 16 && ip[1] < 32;
                case 192: return ip[1] == 168;
                default: return false;
            }
        }
    }
}
