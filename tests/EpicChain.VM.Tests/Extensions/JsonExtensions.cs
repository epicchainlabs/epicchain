// Copyright (C) 2021-2024 EpicChain Labs.

//
// JsonExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Serialization;

namespace EpicChain.Test.Extensions
{
    public static class JsonExtensions
    {
        private static readonly JsonSerializerSettings _settings;

        /// <summary>
        /// Static constructor
        /// </summary>
        static JsonExtensions()
        {
            _settings = new JsonSerializerSettings
            {
                ContractResolver = new CamelCasePropertyNamesContractResolver(),
                Formatting = Formatting.Indented,
                NullValueHandling = NullValueHandling.Ignore,
            };

            _settings.Converters.Add(new StringEnumConverter(new CamelCaseNamingStrategy()));
        }

        /// <summary>
        /// Deserialize json to object
        /// </summary>
        /// <typeparam name="T">Type</typeparam>
        /// <param name="input">Json</param>
        /// <returns>Unit test</returns>
        public static T DeserializeJson<T>(this string input)
        {
            return JsonConvert.DeserializeObject<T>(input, _settings);
        }

        /// <summary>
        /// Serialize UT to json
        /// </summary>
        /// <param name="ut">Unit test</param>
        /// <returns>Json</returns>
        public static string ToJson(this object ut)
        {
            return JsonConvert.SerializeObject(ut, _settings);
        }
    }
}
