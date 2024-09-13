// Copyright (C) 2021-2024 EpicChain Labs.

//
// Benchmarks.Types.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using BenchmarkDotNet.Attributes;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.VM.Benchmark;

public class Benchmarks_Types
{
    public IEnumerable<(int Depth, int ElementsPerLevel)> ParamSource()
    {
        int[] depths = [2, 4];
        int[] elementsPerLevel = [2, 4, 6];

        foreach (var depth in depths)
        {
            foreach (var elements in elementsPerLevel)
            {
                if (depth <= 8 || elements <= 2)
                {
                    yield return (depth, elements);
                }
            }
        }
    }

    [ParamsSource(nameof(ParamSource))]
    public (int Depth, int ElementsPerLevel) Params;

    [Benchmark]
    public void BenchNestedArrayDeepCopy()
    {
        var root = new Array(new ReferenceCounter());
        CreateNestedArray(root, Params.Depth, Params.ElementsPerLevel);
        _ = root.DeepCopy();
    }

    [Benchmark]
    public void BenchNestedArrayDeepCopyWithReferenceCounter()
    {
        var referenceCounter = new ReferenceCounter();
        var root = new Array(referenceCounter);
        CreateNestedArray(root, Params.Depth, Params.ElementsPerLevel, referenceCounter);
        _ = root.DeepCopy();
    }

    [Benchmark]
    public void BenchNestedTestArrayDeepCopy()
    {
        var root = new TestArray(new ReferenceCounter());
        CreateNestedTestArray(root, Params.Depth, Params.ElementsPerLevel);
        _ = root.DeepCopy();
    }

    [Benchmark]
    public void BenchNestedTestArrayDeepCopyWithReferenceCounter()
    {
        var referenceCounter = new ReferenceCounter();
        var root = new TestArray(referenceCounter);
        CreateNestedTestArray(root, Params.Depth, Params.ElementsPerLevel, referenceCounter);
        _ = root.DeepCopy();
    }

    private static void CreateNestedArray(Array? rootArray, int depth, int elementsPerLevel = 1, ReferenceCounter? referenceCounter = null)
    {
        if (depth < 0)
        {
            throw new ArgumentException("Depth must be non-negative", nameof(depth));
        }

        if (rootArray == null)
        {
            throw new ArgumentNullException(nameof(rootArray));
        }

        if (depth == 0)
        {
            return;
        }

        for (var i = 0; i < elementsPerLevel; i++)
        {
            var childArray = new Array(referenceCounter);
            rootArray.Add(childArray);
            CreateNestedArray(childArray, depth - 1, elementsPerLevel, referenceCounter);
        }
    }

    private static void CreateNestedTestArray(TestArray rootArray, int depth, int elementsPerLevel = 1, ReferenceCounter referenceCounter = null)
    {
        if (depth < 0)
        {
            throw new ArgumentException("Depth must be non-negative", nameof(depth));
        }

        if (rootArray == null)
        {
            throw new ArgumentNullException(nameof(rootArray));
        }

        if (depth == 0)
        {
            return;
        }

        for (var i = 0; i < elementsPerLevel; i++)
        {
            var childArray = new TestArray(referenceCounter);
            rootArray.Add(childArray);
            CreateNestedTestArray(childArray, depth - 1, elementsPerLevel, referenceCounter);
        }
    }
}
