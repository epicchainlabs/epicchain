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
// JObject.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Text.Json;

namespace Neo.Json
{
    /// <summary>
    /// Represents a JSON object.
    /// </summary>
    public class JObject : JContainer
    {
        private readonly OrderedDictionary<string, JToken?> properties = new();

        /// <summary>
        /// Gets or sets the properties of the JSON object.
        /// </summary>
        public IDictionary<string, JToken?> Properties => properties;

        /// <summary>
        /// Gets or sets the properties of the JSON object.
        /// </summary>
        /// <param name="name">The name of the property to get or set.</param>
        /// <returns>The property with the specified name.</returns>
        public override JToken? this[string name]
        {
            get
            {
                if (Properties.TryGetValue(name, out JToken? value))
                    return value;
                return null;
            }
            set
            {
                Properties[name] = value;
            }
        }

        public override IReadOnlyList<JToken?> Children => properties.Values;

        /// <summary>
        /// Determines whether the JSON object contains a property with the specified name.
        /// </summary>
        /// <param name="key">The property name to locate in the JSON object.</param>
        /// <returns><see langword="true"/> if the JSON object contains a property with the name; otherwise, <see langword="false"/>.</returns>
        public bool ContainsProperty(string key)
        {
            return Properties.ContainsKey(key);
        }

        public override void Clear() => properties.Clear();

        internal override void Write(Utf8JsonWriter writer)
        {
            writer.WriteStartObject();
            foreach (var (key, value) in Properties)
            {
                writer.WritePropertyName(key);
                if (value is null)
                    writer.WriteNullValue();
                else
                    value.Write(writer);
            }
            writer.WriteEndObject();
        }

        /// <summary>
        /// Creates a copy of the current JSON object.
        /// </summary>
        /// <returns>A copy of the current JSON object.</returns>
        public override JToken Clone()
        {
            var cloned = new JObject();

            foreach (var (key, value) in Properties)
            {
                cloned[key] = value != null ? value.Clone() : Null;
            }

            return cloned;
        }
    }
}
