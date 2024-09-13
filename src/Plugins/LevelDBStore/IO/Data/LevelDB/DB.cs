// Copyright (C) 2021-2024 EpicChain Labs.

//
// DB.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.IO;

namespace EpicChain.IO.Data.LevelDB
{
    public class DB : IDisposable
    {
        private IntPtr handle;

        /// <summary>
        /// Return true if haven't got valid handle
        /// </summary>
        public bool IsDisposed => handle == IntPtr.Zero;

        private DB(IntPtr handle)
        {
            this.handle = handle;
        }

        public void Dispose()
        {
            if (handle != IntPtr.Zero)
            {
                Native.leveldb_close(handle);
                handle = IntPtr.Zero;
            }
        }

        public void Delete(WriteOptions options, byte[] key)
        {
            Native.leveldb_delete(handle, options.handle, key, (UIntPtr)key.Length, out IntPtr error);
            NativeHelper.CheckError(error);
        }

        public byte[] Get(ReadOptions options, byte[] key)
        {
            IntPtr value = Native.leveldb_get(handle, options.handle, key, (UIntPtr)key.Length, out UIntPtr length, out IntPtr error);
            try
            {
                NativeHelper.CheckError(error);
                return value.ToByteArray(length);
            }
            finally
            {
                if (value != IntPtr.Zero) Native.leveldb_free(value);
            }
        }

        public bool Contains(ReadOptions options, byte[] key)
        {
            IntPtr value = Native.leveldb_get(handle, options.handle, key, (UIntPtr)key.Length, out _, out IntPtr error);
            NativeHelper.CheckError(error);

            if (value != IntPtr.Zero)
            {
                Native.leveldb_free(value);
                return true;
            }

            return false;
        }

        public Snapshot GetSnapshot()
        {
            return new Snapshot(handle);
        }

        public Iterator NewIterator(ReadOptions options)
        {
            return new Iterator(Native.leveldb_create_iterator(handle, options.handle));
        }

        public static DB Open(string name)
        {
            return Open(name, Options.Default);
        }

        public static DB Open(string name, Options options)
        {
            IntPtr handle = Native.leveldb_open(options.handle, Path.GetFullPath(name), out IntPtr error);
            NativeHelper.CheckError(error);
            return new DB(handle);
        }

        public void Put(WriteOptions options, byte[] key, byte[] value)
        {
            Native.leveldb_put(handle, options.handle, key, (UIntPtr)key.Length, value, (UIntPtr)value.Length, out IntPtr error);
            NativeHelper.CheckError(error);
        }

        public static void Repair(string name, Options options)
        {
            Native.leveldb_repair_db(options.handle, Path.GetFullPath(name), out IntPtr error);
            NativeHelper.CheckError(error);
        }

        public void Write(WriteOptions options, WriteBatch write_batch)
        {
            Native.leveldb_write(handle, options.handle, write_batch.handle, out IntPtr error);
            NativeHelper.CheckError(error);
        }
    }
}
