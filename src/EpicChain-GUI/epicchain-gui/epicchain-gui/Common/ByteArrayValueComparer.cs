using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common
{
    public class ByteArrayValueComparer : IEqualityComparer<byte[]>
    {

        public static readonly ByteArrayValueComparer Default = new();

        public bool Equals(byte[] x, byte[] y)
        {
            return x.AsSpan().SequenceEqual(y.AsSpan());
        }

        public int GetHashCode(byte[] obj)
        {
            unchecked
            {
                int hash = 17;
                foreach (byte element in obj)
                    hash = hash * 31 + element;
                return hash;
            }
        }

    }
}
