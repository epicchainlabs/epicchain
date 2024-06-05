// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Program.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.CLI;

namespace Neo
{
    static class Program
    {
        static void Main(string[] args)
        {
            var mainService = new MainService();
            mainService.Run(args);
        }
    }
}
