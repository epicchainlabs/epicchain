// Copyright (C) 2021-2024 The EpicChain Labs.
//
// StoreFactory.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Collections.Generic;

namespace Neo.Persistence;

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
