// Copyright (C) 2021-2024 EpicChain Labs.

//
// PriorityMailbox.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Actor;
using Akka.Configuration;
using Akka.Dispatch;
using Akka.Dispatch.MessageQueues;
using System.Collections;

namespace EpicChain.IO.Actors
{
    internal abstract class PriorityMailbox
        (Settings settings, Config config) : MailboxType(settings, config), IProducesMessageQueue<PriorityMessageQueue>
    {
        public override IMessageQueue Create(IActorRef owner, ActorSystem system) =>
            new PriorityMessageQueue(ShallDrop, IsHighPriority);

        internal protected virtual bool IsHighPriority(object message) => false;
        internal protected virtual bool ShallDrop(object message, IEnumerable queue) => false;
    }
}
