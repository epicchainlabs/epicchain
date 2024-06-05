// Copyright (C) 2021-2024 The EpicChain Lab's.
//
// WitnessRuleAction.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// Indicates the action to be taken if the current context meets with the rule.
    /// </summary>
    public enum WitnessRuleAction : byte
    {
        /// <summary>
        /// Deny the witness according to the rule.
        /// </summary>
        Deny = 0,

        /// <summary>
        /// Allow the witness according to the rule.
        /// </summary>
        Allow = 1
    }
}
