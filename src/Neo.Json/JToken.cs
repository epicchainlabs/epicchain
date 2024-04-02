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
// JToken.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Text.Json;
using static Neo.Json.Utility;

namespace Neo.Json;

/// <summary>
/// Represents an abstract JSON token.
/// </summary>
public abstract class JToken
{
    /// <summary>
    /// Represents a <see langword="null"/> token.
    /// </summary>
    public const JToken? Null = null;

    /// <summary>
    /// Gets or sets the child token at the specified index.
    /// </summary>
    /// <param name="index">The zero-based index of the child token to get or set.</param>
    /// <returns>The child token at the specified index.</returns>
    public virtual JToken? this[int index]
    {
        get => throw new NotSupportedException();
        set => throw new NotSupportedException();
    }

    /// <summary>
    /// Gets or sets the properties of the JSON object.
    /// </summary>
    /// <param name="key">The key of the property to get or set.</param>
    /// <returns>The property with the specified name.</returns>
    public virtual JToken? this[string key]
    {
        get => throw new NotSupportedException();
        set => throw new NotSupportedException();
    }

    /// <summary>
    /// Converts the current JSON token to a boolean value.
    /// </summary>
    /// <returns>The converted value.</returns>
    public virtual bool AsBoolean()
    {
        return true;
    }

    /// <summary>
    /// Converts the current JSON token to an <see cref="Enum"/>.
    /// </summary>
    /// <typeparam name="T">The type of the <see cref="Enum"/>.</typeparam>
    /// <param name="defaultValue">If the current JSON token cannot be converted to type <typeparamref name="T"/>, then the default value is returned.</param>
    /// <param name="ignoreCase">Indicates whether case should be ignored during conversion.</param>
    /// <returns>The converted value.</returns>
    public virtual T AsEnum<T>(T defaultValue = default, bool ignoreCase = false) where T : unmanaged, Enum
    {
        return defaultValue;
    }

    /// <summary>
    /// Converts the current JSON token to a floating point number.
    /// </summary>
    /// <returns>The converted value.</returns>
    public virtual double AsNumber()
    {
        return double.NaN;
    }

    /// <summary>
    /// Converts the current JSON token to a <see cref="string"/>.
    /// </summary>
    /// <returns>The converted value.</returns>
    public virtual string AsString()
    {
        return ToString();
    }

    /// <summary>
    /// Converts the current JSON token to a boolean value.
    /// </summary>
    /// <returns>The converted value.</returns>
    /// <exception cref="InvalidCastException">The JSON token is not a <see cref="JBoolean"/>.</exception>
    public virtual bool GetBoolean() => throw new InvalidCastException();

    public virtual T GetEnum<T>(bool ignoreCase = false) where T : unmanaged, Enum => throw new InvalidCastException();

    /// <summary>
    /// Converts the current JSON token to a 32-bit signed integer.
    /// </summary>
    /// <returns>The converted value.</returns>
    /// <exception cref="InvalidCastException">The JSON token is not a <see cref="JNumber"/>.</exception>
    /// <exception cref="InvalidCastException">The JSON token cannot be converted to an integer.</exception>
    /// <exception cref="OverflowException">The JSON token cannot be converted to a 32-bit signed integer.</exception>
    public int GetInt32()
    {
        double d = GetNumber();
        if (d % 1 != 0) throw new InvalidCastException();
        return checked((int)d);
    }

    /// <summary>
    /// Converts the current JSON token to a floating point number.
    /// </summary>
    /// <returns>The converted value.</returns>
    /// <exception cref="InvalidCastException">The JSON token is not a <see cref="JNumber"/>.</exception>
    public virtual double GetNumber() => throw new InvalidCastException();

    /// <summary>
    /// Converts the current JSON token to a <see cref="string"/>.
    /// </summary>
    /// <returns>The converted value.</returns>
    /// <exception cref="InvalidCastException">The JSON token is not a <see cref="JString"/>.</exception>
    public virtual string GetString() => throw new InvalidCastException();

    /// <summary>
    /// Parses a JSON token from a byte array.
    /// </summary>
    /// <param name="value">The byte array that contains the JSON token.</param>
    /// <param name="max_nest">The maximum nesting depth when parsing the JSON token.</param>
    /// <returns>The parsed JSON token.</returns>
    public static JToken? Parse(ReadOnlySpan<byte> value, int max_nest = 64)
    {
        Utf8JsonReader reader = new(value, new JsonReaderOptions
        {
            AllowTrailingCommas = false,
            CommentHandling = JsonCommentHandling.Skip,
            MaxDepth = max_nest
        });
        try
        {
            JToken? json = Read(ref reader);
            if (reader.Read()) throw new FormatException();
            return json;
        }
        catch (JsonException ex)
        {
            throw new FormatException(ex.Message, ex);
        }
    }

    /// <summary>
    /// Parses a JSON token from a <see cref="string"/>.
    /// </summary>
    /// <param name="value">The <see cref="string"/> that contains the JSON token.</param>
    /// <param name="max_nest">The maximum nesting depth when parsing the JSON token.</param>
    /// <returns>The parsed JSON token.</returns>
    public static JToken? Parse(string value, int max_nest = 64)
    {
        return Parse(StrictUTF8.GetBytes(value), max_nest);
    }

    private static JToken? Read(ref Utf8JsonReader reader, bool skipReading = false)
    {
        if (!skipReading && !reader.Read()) throw new FormatException();
        return reader.TokenType switch
        {
            JsonTokenType.False => false,
            JsonTokenType.Null => Null,
            JsonTokenType.Number => reader.GetDouble(),
            JsonTokenType.StartArray => ReadArray(ref reader),
            JsonTokenType.StartObject => ReadObject(ref reader),
            JsonTokenType.String => ReadString(ref reader),
            JsonTokenType.True => true,
            _ => throw new FormatException(),
        };
    }

    private static JArray ReadArray(ref Utf8JsonReader reader)
    {
        JArray array = new();
        while (reader.Read())
        {
            switch (reader.TokenType)
            {
                case JsonTokenType.EndArray:
                    return array;
                default:
                    array.Add(Read(ref reader, skipReading: true));
                    break;
            }
        }
        throw new FormatException();
    }

    private static JObject ReadObject(ref Utf8JsonReader reader)
    {
        JObject obj = new();
        while (reader.Read())
        {
            switch (reader.TokenType)
            {
                case JsonTokenType.EndObject:
                    return obj;
                case JsonTokenType.PropertyName:
                    string name = ReadString(ref reader);
                    if (obj.Properties.ContainsKey(name)) throw new FormatException();
                    JToken? value = Read(ref reader);
                    obj.Properties.Add(name, value);
                    break;
                default:
                    throw new FormatException();
            }
        }
        throw new FormatException();
    }

    private static string ReadString(ref Utf8JsonReader reader)
    {
        try
        {
            return reader.GetString()!;
        }
        catch (InvalidOperationException ex)
        {
            throw new FormatException(ex.Message, ex);
        }
    }

    /// <summary>
    /// Encode the current JSON token into a byte array.
    /// </summary>
    /// <param name="indented">Indicates whether indentation is required.</param>
    /// <returns>The encoded JSON token.</returns>
    public byte[] ToByteArray(bool indented)
    {
        using MemoryStream ms = new();
        using Utf8JsonWriter writer = new(ms, new JsonWriterOptions
        {
            Indented = indented,
            SkipValidation = true
        });
        Write(writer);
        writer.Flush();
        return ms.ToArray();
    }

    /// <summary>
    /// Encode the current JSON token into a <see cref="string"/>.
    /// </summary>
    /// <returns>The encoded JSON token.</returns>
    public override string ToString()
    {
        return ToString(false);
    }

    /// <summary>
    /// Encode the current JSON token into a <see cref="string"/>.
    /// </summary>
    /// <param name="indented">Indicates whether indentation is required.</param>
    /// <returns>The encoded JSON token.</returns>
    public string ToString(bool indented)
    {
        return StrictUTF8.GetString(ToByteArray(indented));
    }

    internal abstract void Write(Utf8JsonWriter writer);

    public abstract JToken Clone();

    public JArray JsonPath(string expr)
    {
        JToken?[] objects = { this };
        if (expr.Length == 0) return objects;
        Queue<JPathToken> tokens = new(JPathToken.Parse(expr));
        JPathToken first = tokens.Dequeue();
        if (first.Type != JPathTokenType.Root) throw new FormatException();
        JPathToken.ProcessJsonPath(ref objects, tokens);
        return objects;
    }

    public static implicit operator JToken(Enum value)
    {
        return (JString)value;
    }

    public static implicit operator JToken(JToken?[] value)
    {
        return (JArray)value;
    }

    public static implicit operator JToken(bool value)
    {
        return (JBoolean)value;
    }

    public static implicit operator JToken(double value)
    {
        return (JNumber)value;
    }

    public static implicit operator JToken?(string? value)
    {
        return (JString?)value;
    }
}
