// Copyright (C) 2021-2024 EpicChain Labs.

//
// StoreFactory.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;

namespace EpicChain.Persistence;

public static class StoreFactory
{
    private static readonly Dictionary<string, IStoreProvider> providers = new();

    static StoreFactory()
    {
        var memProvider = new MemoryStoreProvider();
        RegisterProvider(memProvider);

        // Default cases
        providers.Add("", memProvider);
    }

    public static void RegisterProvider(IStoreProvider provider)
    {
        providers.Add(provider.Name, provider);
    }

    /// <summary>
    /// Get store provider by name
    /// </summary>
    /// <param name="name">Name</param>
    /// <returns>Store provider</returns>
    public static IStoreProvider GetStoreProvider(string name)
    {
        if (providers.TryGetValue(name, out var provider))
        {
            return provider;
        }

        return null;
    }

    /// <summary>
    /// Get store from name
    /// </summary>
    /// <param name="storageProvider">The storage engine used to create the <see cref="IStore"/> objects. If this parameter is <see langword="null"/>, a default in-memory storage engine will be used.</param>
    /// <param name="path">The path of the storage. If <paramref name="storageProvider"/> is the default in-memory storage engine, this parameter is ignored.</param>
    /// <returns>The storage engine.</returns>
    public static IStore GetStore(string storageProvider, string path)
    {
        return providers[storageProvider].GetStore(path);
    }
}
