// Copyright (C) 2021-2024 EpicChain Labs.

//
// WitnessRule.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads.Conditions;
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// The rule used to describe the scope of the witness.
    /// </summary>
    public class WitnessRule : IInteroperable, ISerializable
    {
        /// <summary>
        /// Indicates the action to be taken if the current context meets with the rule.
        /// </summary>
        public WitnessRuleAction Action;

        /// <summary>
        /// The condition of the rule.
        /// </summary>
        public WitnessCondition Condition;

        int ISerializable.Size => sizeof(WitnessRuleAction) + Condition.Size;

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Action = (WitnessRuleAction)reader.ReadByte();
            if (Action != WitnessRuleAction.Allow && Action != WitnessRuleAction.Deny)
                throw new FormatException();
            Condition = WitnessCondition.DeserializeFrom(ref reader, WitnessCondition.MaxNestingDepth);
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Action);
            writer.Write(Condition);
        }

        /// <summary>
        /// Converts the <see cref="WitnessRule"/> from a JSON object.
        /// </summary>
        /// <param name="json">The <see cref="WitnessRule"/> represented by a JSON object.</param>
        /// <returns>The converted <see cref="WitnessRule"/>.</returns>
        public static WitnessRule FromJson(JObject json)
        {
            WitnessRuleAction action = Enum.Parse<WitnessRuleAction>(json["action"].GetString());

            if (action != WitnessRuleAction.Allow && action != WitnessRuleAction.Deny)
                throw new FormatException();

            return new()
            {
                Action = action,
                Condition = WitnessCondition.FromJson((JObject)json["condition"], WitnessCondition.MaxNestingDepth)
            };
        }

        /// <summary>
        /// Converts the rule to a JSON object.
        /// </summary>
        /// <returns>The rule represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["action"] = Action,
                ["condition"] = Condition.ToJson()
            };
        }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            throw new NotSupportedException();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new VM.Types.Array(referenceCounter, new StackItem[]
            {
                (byte)Action,
                Condition.ToStackItem(referenceCounter)
            });
        }
    }
}
