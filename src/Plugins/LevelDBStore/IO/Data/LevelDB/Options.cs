// Copyright (C) 2021-2024 EpicChain Labs.

//
// Options.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.IO.Data.LevelDB
{
    public class Options
    {
        public static readonly Options Default = new Options();
        internal readonly IntPtr handle = Native.leveldb_options_create();

        public bool CreateIfMissing
        {
            set
            {
                Native.leveldb_options_set_create_if_missing(handle, value);
            }
        }

        public bool ErrorIfExists
        {
            set
            {
                Native.leveldb_options_set_error_if_exists(handle, value);
            }
        }

        public bool ParanoidChecks
        {
            set
            {
                Native.leveldb_options_set_paranoid_checks(handle, value);
            }
        }

        public int WriteBufferSize
        {
            set
            {
                Native.leveldb_options_set_write_buffer_size(handle, (UIntPtr)value);
            }
        }

        public int MaxOpenFiles
        {
            set
            {
                Native.leveldb_options_set_max_open_files(handle, value);
            }
        }

        public int BlockSize
        {
            set
            {
                Native.leveldb_options_set_block_size(handle, (UIntPtr)value);
            }
        }

        public int BlockRestartInterval
        {
            set
            {
                Native.leveldb_options_set_block_restart_interval(handle, value);
            }
        }

        public CompressionType Compression
        {
            set
            {
                Native.leveldb_options_set_compression(handle, value);
            }
        }

        public IntPtr FilterPolicy
        {
            set
            {
                Native.leveldb_options_set_filter_policy(handle, value);
            }
        }

        ~Options()
        {
            Native.leveldb_options_destroy(handle);
        }
    }
}
