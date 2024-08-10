using Neo.Network.P2P.Payloads;

namespace Neo.Models.Transactions
{
    public class TranAttributeModel
    {
        public string Type => Usage.ToString();
        public TransactionAttributeType Usage { get; set; }
        public string Data { get; set; }
    }
}