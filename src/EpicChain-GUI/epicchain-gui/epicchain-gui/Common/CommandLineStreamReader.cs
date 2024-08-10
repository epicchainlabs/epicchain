using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common
{
    public class CommandLineStreamReader
    {
        private readonly StreamReader _streamReader;
        private readonly StringBuilder _linesBuffer;

        public CommandLineStreamReader(StreamReader sr)
        {
            _streamReader = sr;
            _linesBuffer=new StringBuilder();
            Task.Factory.StartNew(Run);
        }

        public async Task Run()
        {
            char[] buf = new char[8192];
            while (true)
            {
                int dataLength = await this._streamReader.ReadAsync(buf, 0, buf.Length);
                if (dataLength != 0)
                {
                    //this.OnChunk(new ArraySegment<char>(buf, 0, num1));
                    int endOfLine = Array.IndexOf<char>(buf, '\n', 0, dataLength);
                    if (endOfLine < 0)
                    {
                        _linesBuffer.Append(buf, 0, dataLength);
                    }
                    else
                    {
                        var length = endOfLine + 1;
                        _linesBuffer.Append(buf, 0, length);
                        //this.OnCompleteLine(this._linesBuffer.ToString());
                        _linesBuffer.Clear();
                        _linesBuffer.Append(buf, length, dataLength - length);
                    }
                }
                else
                    break;
            }
            //this.OnClosed();
        }


        private void OnCompleteLine(string line)
        {
            OnReceivedLine?.Invoke(line);
        }
        public event MyEventHandler<string> OnReceivedLine;



        public delegate void MyEventHandler<in TArgs>(TArgs args);

    }
}
