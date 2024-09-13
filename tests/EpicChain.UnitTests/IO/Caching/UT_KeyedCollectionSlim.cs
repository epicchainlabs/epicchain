// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_KeyedCollectionSlim.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO.Caching;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace EpicChain.UnitTests.IO.Caching
{
    [TestClass]
    public class UT_KeyedCollectionSlim
    {
        [TestMethod]
        public void Add_ShouldAddItem()
        {
            // Arrange
            var collection = new TestKeyedCollectionSlim();
            var item = new TestItem { Id = 1, Name = "Item1" };

            // Act
            collection.Add(item);

            // Assert
            collection.Count.Should().Be(1);
            collection.Contains(1).Should().BeTrue();
            collection.First.Should().Be(item);
        }

        [TestMethod]
        public void Add_ShouldThrowException_WhenKeyAlreadyExists()
        {
            // Arrange
            var collection = new TestKeyedCollectionSlim();
            var item1 = new TestItem { Id = 1, Name = "Item1" };
            var item2 = new TestItem { Id = 1, Name = "Item2" }; // Same ID as item1

            // Act
            collection.Add(item1);

            // Assert
            var act = (() => collection.Add(item2));
            act.Should().Throw<ArgumentException>();
        }

        [TestMethod]
        public void Remove_ShouldRemoveItem()
        {
            // Arrange
            var collection = new TestKeyedCollectionSlim();
            var item = new TestItem { Id = 1, Name = "Item1" };
            collection.Add(item);

            // Act
            collection.Remove(1);

            // Assert
            collection.Count.Should().Be(0);
            collection.Contains(1).Should().BeFalse();
        }

        [TestMethod]
        public void RemoveFirst_ShouldRemoveFirstItem()
        {
            // Arrange
            var collection = new TestKeyedCollectionSlim();
            var item1 = new TestItem { Id = 1, Name = "Item1" };
            var item2 = new TestItem { Id = 2, Name = "Item2" };
            collection.Add(item1);
            collection.Add(item2);

            // Act
            collection.RemoveFirst();

            // Assert
            collection.Count.Should().Be(1);
            collection.Contains(1).Should().BeFalse();
            collection.Contains(2).Should().BeTrue();
        }

        public class TestItem : IStructuralEquatable, IStructuralComparable, IComparable
        {
            public int Id { get; set; }

            public string Name { get; set; }

            public int CompareTo(object? obj)
            {
                if (obj is not TestItem other) throw new ArgumentException("Object is not a TestItem");
                return Id.CompareTo(other.Id);
            }

            public bool Equals(object? other, IEqualityComparer comparer)
            {
                return other is TestItem item && Id == item.Id && Name == item.Name;
            }

            public int GetHashCode(IEqualityComparer comparer)
            {
                return HashCode.Combine(Id, Name);
            }

            public int CompareTo(TestItem other)
            {
                return Id.CompareTo(other.Id);
            }

            public int CompareTo(object other, IComparer comparer)
            {
                throw new NotImplementedException();
            }
        }

        internal class TestKeyedCollectionSlim : KeyedCollectionSlim<int, TestItem>
        {
            protected override int GetKeyForItem(TestItem? item)
            {
                return item?.Id ?? throw new ArgumentNullException(nameof(item), "Item cannot be null");
            }
        }
    }
}
