// Copyright (C) 2021-2024 EpicChain Labs.

//
// PriorityMessageQueue.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Akka.Dispatch;
using Akka.Dispatch.MessageQueues;
using System;
using System.Collections;
using System.Collections.Concurrent;
using System.Linq;
using System.Threading;

namespace EpicChain.IO.Actors
{
    internal class PriorityMessageQueue
        (Func<object, IEnumerable, bool> dropper, Func<object, bool> priority_generator) : IMessageQueue, IUnboundedMessageQueueSemantics
    {
        private readonly ConcurrentQueue<Envelope> _high = new();
        private readonly ConcurrentQueue<Envelope> _low = new();
        private readonly Func<object, IEnumerable, bool> _dropper = dropper;
        private readonly Func<object, bool> _priority_generator = priority_generator;
        private int _idle = 1;

        public bool HasMessages => !_high.IsEmpty || !_low.IsEmpty;
        public int Count => _high.Count + _low.Count;

        public void CleanUp(IActorRef owner, IMessageQueue deadletters)
        {
        }

        public void Enqueue(IActorRef receiver, Envelope envelope)
        {
            Interlocked.Increment(ref _idle);
            if (envelope.Message is Idle) return;
            if (_dropper(envelope.Message, _high.Concat(_low).Select(p => p.Message)))
                return;
            var queue = _priority_generator(envelope.Message) ? _high : _low;
            queue.Enqueue(envelope);
        }

        public bool TryDequeue(out Envelope envelope)
        {
            if (_high.TryDequeue(out envelope)) return true;
            if (_low.TryDequeue(out envelope)) return true;
            if (Interlocked.Exchange(ref _idle, 0) > 0)
            {
                envelope = new Envelope(Idle.Instance, ActorRefs.NoSender);
                return true;
            }
            return false;
        }
    }
}
