// Copyright (C) 2021-2024 EpicChain Labs.

//
// Result.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
namespace EpicChain.Plugins.RpcServer
{
    public static class Result
    {
        /// <summary>
        /// Checks the execution result of a function and throws an exception if it is null or throw an exception.
        /// </summary>
        /// <param name="function">The function to execute</param>
        /// <param name="err">The rpc error</param>
        /// <param name="withData">Append extra base exception message</param>
        /// <typeparam name="T">The return type</typeparam>
        /// <returns>The execution result</returns>
        /// <exception cref="RpcException">The Rpc exception</exception>
        public static T Ok_Or<T>(Func<T> function, RpcError err, bool withData = false)
        {
            try
            {
                var result = function();
                if (result == null) throw new RpcException(err);
                return result;
            }
            catch (Exception ex)
            {
                if (withData)
                    throw new RpcException(err.WithData(ex.GetBaseException().Message));
                throw new RpcException(err);
            }
        }

        /// <summary>
        /// Checks the execution result and throws an exception if it is null.
        /// </summary>
        /// <param name="result">The execution result</param>
        /// <param name="err">The rpc error</param>
        /// <typeparam name="T">The return type</typeparam>
        /// <returns>The execution result</returns>
        /// <exception cref="RpcException">The Rpc exception</exception>
        public static T NotNull_Or<T>(this T result, RpcError err)
        {
            if (result == null) throw new RpcException(err);
            return result;
        }

        /// <summary>
        /// The execution result is true or throws an exception or null.
        /// </summary>
        /// <param name="function">The function to execute</param>
        /// <param name="err">the rpc exception code</param>
        /// <returns>the execution result</returns>
        /// <exception cref="RpcException">The rpc exception</exception>
        public static bool True_Or(Func<bool> function, RpcError err)
        {
            try
            {
                var result = function();
                if (!result.Equals(true)) throw new RpcException(err);
                return result;
            }
            catch
            {
                throw new RpcException(err);
            }
        }

        /// <summary>
        /// Checks if the execution result is true or throws an exception.
        /// </summary>
        /// <param name="result">the execution result</param>
        /// <param name="err">the rpc exception code</param>
        /// <returns>the execution result</returns>
        /// <exception cref="RpcException">The rpc exception</exception>
        public static bool True_Or(this bool result, RpcError err)
        {
            if (!result.Equals(true)) throw new RpcException(err);
            return result;
        }

        /// <summary>
        /// Checks if the execution result is false or throws an exception.
        /// </summary>
        /// <param name="result">the execution result</param>
        /// <param name="err">the rpc exception code</param>
        /// <returns>the execution result</returns>
        /// <exception cref="RpcException">The rpc exception</exception>
        public static bool False_Or(this bool result, RpcError err)
        {
            if (!result.Equals(false)) throw new RpcException(err);
            return result;
        }

        /// <summary>
        /// Check if the execution result is null or throws an exception.
        /// </summary>
        /// <param name="result">The execution result</param>
        /// <param name="err">the rpc error</param>
        /// <typeparam name="T">The execution result type</typeparam>
        /// <returns>The execution result</returns>
        /// <exception cref="RpcException">the rpc exception</exception>
        public static void Null_Or<T>(this T result, RpcError err)
        {
            if (result != null) throw new RpcException(err);
        }
    }
}
