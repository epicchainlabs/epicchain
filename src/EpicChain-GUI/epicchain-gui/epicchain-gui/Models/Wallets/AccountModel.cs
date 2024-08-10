namespace Neo.Models.Wallets
{
    public class AccountModel
    {
        public UInt160 ScriptHash { get; set; }
        public string Address { get; set; }
        public AccountType AccountType { get; set; } = AccountType.Standard;

        public bool WatchOnly { get; set; }

        public string Neo { get; set; } = "0";
        public string Gas { get; set; } = "0";
    }
}