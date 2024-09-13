// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Plugin.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Ledger;
using EpicChain.Plugins;
using System;
using System.Reflection;

namespace EpicChain.UnitTests.Plugins
{
    [TestClass]
    public class UT_Plugin
    {
        private static readonly object s_locker = new();

        [TestInitialize]
        public void TestInitialize()
        {
            ClearEventHandlers();
        }

        [TestCleanup]
        public void TestCleanup()
        {
            ClearEventHandlers();
        }

        private static void ClearEventHandlers()
        {
            ClearEventHandler("Committing");
            ClearEventHandler("Committed");
        }

        private static void ClearEventHandler(string eventName)
        {
            var eventInfo = typeof(Blockchain).GetEvent(eventName, BindingFlags.Static | BindingFlags.Public | BindingFlags.NonPublic);
            if (eventInfo == null)
            {
                return;
            }

            var fields = typeof(Blockchain).GetFields(BindingFlags.Static | BindingFlags.NonPublic | BindingFlags.Instance | BindingFlags.Public);
            foreach (var field in fields)
            {
                if (field.FieldType == typeof(MulticastDelegate) || field.FieldType.BaseType == typeof(MulticastDelegate))
                {
                    var eventDelegate = (MulticastDelegate)field.GetValue(null);
                    if (eventDelegate != null && field.Name.Contains(eventName))
                    {
                        foreach (var handler in eventDelegate.GetInvocationList())
                        {
                            eventInfo.RemoveEventHandler(null, handler);
                        }
                        break;
                    }
                }
            }
        }

        [TestMethod]
        public void TestGetConfigFile()
        {
            var pp = new TestPlugin();
            var file = pp.ConfigFile;
            file.EndsWith("config.json").Should().BeTrue();
        }

        [TestMethod]
        public void TestGetName()
        {
            var pp = new TestPlugin();
            pp.Name.Should().Be("TestPlugin");
        }

        [TestMethod]
        public void TestGetVersion()
        {
            var pp = new TestPlugin();
            Action action = () => pp.Version.ToString();
            action.Should().NotThrow();
        }

        [TestMethod]
        public void TestSendMessage()
        {
            lock (s_locker)
            {
                Plugin.Plugins.Clear();
                Plugin.SendMessage("hey1").Should().BeFalse();

                var lp = new TestPlugin();
                Plugin.SendMessage("hey2").Should().BeTrue();
            }
        }

        [TestMethod]
        public void TestGetConfiguration()
        {
            var pp = new TestPlugin();
            pp.TestGetConfiguration().Key.Should().Be("PluginConfiguration");
        }

        [TestMethod]
        public void TestOnException()
        {
            _ = new TestPlugin();
            // Ensure no exception is thrown
            try
            {
                Blockchain.InvokeCommitting(null, null, null, null);
                Blockchain.InvokeCommitted(null, null);
            }
            catch (Exception ex)
            {
                Assert.Fail($"InvokeCommitting or InvokeCommitted threw an exception: {ex.Message}");
            }

            // Register TestNonPlugin that throws exceptions
            _ = new TestNonPlugin();

            // Ensure exception is thrown
            Assert.ThrowsException<NotImplementedException>(() =>
           {
               Blockchain.InvokeCommitting(null, null, null, null);
           });

            Assert.ThrowsException<NotImplementedException>(() =>
           {
               Blockchain.InvokeCommitted(null, null);
           });
        }

        [TestMethod]
        public void TestOnPluginStopped()
        {
            var pp = new TestPlugin();
            Assert.AreEqual(false, pp.IsStopped);
            // Ensure no exception is thrown
            try
            {
                Blockchain.InvokeCommitting(null, null, null, null);
                Blockchain.InvokeCommitted(null, null);
            }
            catch (Exception ex)
            {
                Assert.Fail($"InvokeCommitting or InvokeCommitted threw an exception: {ex.Message}");
            }

            Assert.AreEqual(true, pp.IsStopped);
        }

        [TestMethod]
        public void TestOnPluginStopOnException()
        {
            // pp will stop on exception.
            var pp = new TestPlugin();
            Assert.AreEqual(false, pp.IsStopped);
            // Ensure no exception is thrown
            try
            {
                Blockchain.InvokeCommitting(null, null, null, null);
                Blockchain.InvokeCommitted(null, null);
            }
            catch (Exception ex)
            {
                Assert.Fail($"InvokeCommitting or InvokeCommitted threw an exception: {ex.Message}");
            }

            Assert.AreEqual(true, pp.IsStopped);

            // pp2 will not stop on exception.
            var pp2 = new TestPlugin(UnhandledExceptionPolicy.Ignore);
            Assert.AreEqual(false, pp2.IsStopped);
            // Ensure no exception is thrown
            try
            {
                Blockchain.InvokeCommitting(null, null, null, null);
                Blockchain.InvokeCommitted(null, null);
            }
            catch (Exception ex)
            {
                Assert.Fail($"InvokeCommitting or InvokeCommitted threw an exception: {ex.Message}");
            }

            Assert.AreEqual(false, pp2.IsStopped);
        }

        [TestMethod]
        public void TestOnNodeStopOnPluginException()
        {
            // node will stop on pp exception.
            var pp = new TestPlugin(UnhandledExceptionPolicy.StopNode);
            Assert.AreEqual(false, pp.IsStopped);
            Assert.ThrowsException<NotImplementedException>(() =>
            {
                Blockchain.InvokeCommitting(null, null, null, null);
            });

            Assert.ThrowsException<NotImplementedException>(() =>
            {
                Blockchain.InvokeCommitted(null, null);
            });

            Assert.AreEqual(false, pp.IsStopped);
        }
    }
}
