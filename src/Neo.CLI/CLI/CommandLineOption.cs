// 
// Copyright (C) 2021-2024 EpicChain Lab's
// All rights reserved.
// 
// This file is part of the EpicChain project, developed by xmoohad.
// 
// This file is subject to the terms and conditions defined in the LICENSE file found in the top-level 
// directory of this distribution. Unauthorized copying, modification, or distribution of this file,
// via any medium, is strictly prohibited. Any use of this file without explicit permission from EpicChain Lab's
// is a violation of copyright law and will be prosecuted to the fullest extent possible.
// 
// This file is licensed under the MIT License; you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://opensource.org/licenses/MIT
// 
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed 
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for 
// the specific language governing permissions and limitations under the License.
// 
// For more information about EpicChain Lab's projects and innovations, visit our website at https://epic-chain.org
// or contact us at xmoohad@epic-chain.org.
// 
//

namespace Neo.CLI
{
    public class CommandLineOptions
    {
        public string? Config { get; init; }
        public string? Wallet { get; init; }
        public string? Password { get; init; }
        public string[]? Plugins { get; set; }
        public string? DBEngine { get; init; }
        public string? DBPath { get; init; }
        public bool? NoVerify { get; init; }

        /// <summary>
        /// Check if CommandLineOptions was configured
        /// </summary>
        public bool IsValid =>
                !string.IsNullOrEmpty(Config) ||
                !string.IsNullOrEmpty(Wallet) ||
                !string.IsNullOrEmpty(Password) ||
                !string.IsNullOrEmpty(DBEngine) ||
                !string.IsNullOrEmpty(DBPath) ||
                (Plugins?.Length > 0) ||
                NoVerify is not null;
    }
}
