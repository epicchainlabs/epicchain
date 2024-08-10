namespace Neo.Models.Wallets
{
    public class MultiReceiverRequest
    {
        public UInt160 Address { get; set; }
        public string Amount { get; set; }
    }
}