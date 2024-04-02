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
// Benchmarks.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Diagnostics;

namespace Neo.VM
{
    public static class Benchmarks
    {
        public static void NeoIssue2528()
        {
            // https://github.com/neo-project/neo/issues/2528
            // L01: INITSLOT 1, 0
            // L02: NEWARRAY0
            // L03: DUP
            // L04: DUP
            // L05: PUSHINT16 2043
            // L06: STLOC 0
            // L07: PUSH1
            // L08: PACK
            // L09: LDLOC 0
            // L10: DEC
            // L11: STLOC 0
            // L12: LDLOC 0
            // L13: JMPIF_L L07
            // L14: PUSH1
            // L15: PACK
            // L16: APPEND
            // L17: PUSHINT32 38000
            // L18: STLOC 0
            // L19: PUSH0
            // L20: PICKITEM
            // L21: LDLOC 0
            // L22: DEC
            // L23: STLOC 0
            // L24: LDLOC 0
            // L25: JMPIF_L L19
            // L26: DROP
            Run(nameof(NeoIssue2528), "VwEAwkpKAfsHdwARwG8AnXcAbwAl9////xHAzwJwlAAAdwAQzm8AnXcAbwAl9////0U=");
        }

        public static void NeoVMIssue418()
        {
            // https://github.com/neo-project/neo-vm/issues/418
            // L00: NEWARRAY0
            // L01: PUSH0
            // L02: PICK
            // L03: PUSH1
            // L04: PACK
            // L05: PUSH1
            // L06: PICK
            // L07: PUSH1
            // L08: PACK
            // L09: INITSSLOT 1
            // L10: PUSHINT16 510
            // L11: DEC
            // L12: STSFLD0
            // L13: PUSH1
            // L14: PICK
            // L15: PUSH1
            // L16: PICK
            // L17: PUSH2
            // L18: PACK
            // L19: REVERSE3
            // L20: PUSH2
            // L21: PACK
            // L22: LDSFLD0
            // L23: DUP
            // L24: JMPIF L11
            // L25: DROP
            // L26: ROT
            // L27: DROP
            Run(nameof(NeoVMIssue418), "whBNEcARTRHAVgEB/gGdYBFNEU0SwFMSwFhKJPNFUUU=");
        }

        public static void NeoIssue2723()
        {
            // L00: INITSSLOT 1
            // L01: PUSHINT32 130000
            // L02: STSFLD 0
            // L03: PUSHINT32 1048576
            // L04: NEWBUFFER
            // L05: DROP
            // L06: LDSFLD 0
            // L07: DEC
            // L08: DUP
            // L09: STSFLD 0
            // L10: JMPIF L03
            Run(nameof(NeoIssue2723), "VgEC0PsBAGcAAgAAEACIRV8AnUpnACTz");
        }

        private static void Run(string name, string poc)
        {
            byte[] script = Convert.FromBase64String(poc);
            using ExecutionEngine engine = new();
            engine.LoadScript(script);
            Stopwatch stopwatch = Stopwatch.StartNew();
            engine.Execute();
            stopwatch.Stop();
            Debug.Assert(engine.State == VMState.HALT);
            Console.WriteLine($"Benchmark: {name},\tTime: {stopwatch.Elapsed}");
        }
    }
}
