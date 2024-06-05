// Copyright (C) 2021-2024 The EpicChain Labs.
//
// PriorityMessageQueue.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Akka.Actor;
using Akka.Dispatch;
using Akka.Dispatch.MessageQueues;
using System;
using System.Collections;
using System.Collections.Concurrent;
using System.Linq;
using System.Threading;

namespace Neo.IO.Actors
{
    internal class PriorityMessageQueue : IMessageQueue, IUnboundedMessageQueueSemantics
    {
        private readonly ConcurrentQueue<Envelope> high = new();
        private readonly ConcurrentQueue<Envelope> low = new();
        private readonly Func<object, IEnumerable, bool> dropper;
        private readonly Func<object, bool> priority_generator;
        private int idle = 1;

        public bool HasMessages => !high.IsEmpty || !low.IsEmpty;
        public int Count => high.Count + low.Count;

        public PriorityMessageQueue(Func<object, IEnumerable, bool> dropper, Func<object, bool> priority_generator)
        {
            this.dropper = dropper;
            this.priority_generator = priority_generator;
        }

        public void CleanUp(IActorRef owner, IMessageQueue deadletters)
        {
        }

        public void Enqueue(IActorRef receiver, Envelope envelope)
        {
            Interlocked.Increment(ref idle);
            if (envelope.Message is Idle) return;
            if (dropper(envelope.Message, high.Concat(low).Select(p => p.Message)))
                return;
            ConcurrentQueue<Envelope> queue = priority_generator(envelope.Message) ? high : low;
            queue.Enqueue(envelope);
        }

        public bool TryDequeue(out Envelope envelope)
        {
            if (high.TryDequeue(out envelope)) return true;
            if (low.TryDequeue(out envelope)) return true;
            if (Interlocked.Exchange(ref idle, 0) > 0)
            {
                envelope = new Envelope(Idle.Instance, ActorRefs.NoSender);
                return true;
            }
            return false;
        }
    }
}
