using Neo.SmartContract.Manifest;

namespace Neo.Models.Contracts
{
    public class ManifestModel
    {

        public ManifestModel(ContractManifest manifest)
        {
            if (manifest != null)
            {
                ContractName = manifest.Name;
                SupportedStandards = manifest.SupportedStandards;
                Groups = manifest.Groups;
                Permissions = manifest.Permissions;
                Trusts = manifest.Trusts;
                Abi = new ContractAbiModel(manifest.Abi);
                Extra = manifest.Extra;
            }
        }
        /// <summary>
        /// Contract Name
        /// </summary>
        public string ContractName { get; set; }
        /// <summary>
        /// A group represents a set of mutually trusted contracts. A contract will trust and allow any contract in the same group to invoke it, and the user interface will not give any warnings.
        /// </summary>
        public ContractGroup[] Groups { get; set; }


        /// <summary>
        /// For technical details of ABI, please refer to NEP-3: NeoContract ABI. (https://github.com/neo-project/proposals/blob/master/nep-3.mediawiki)
        /// </summary>
        public ContractAbiModel Abi { get; set; }

        /// <summary>
        /// The permissions field is an array containing a set of Permission objects. It describes which contracts may be invoked and which methods are called.
        /// </summary>
        public ContractPermission[] Permissions { get; set; }

        /// <summary>
        /// The trusts field is an array containing a set of contract hashes or group public keys. It can also be assigned with a wildcard *. If it is a wildcard *, then it means that it trusts any contract.
        /// If a contract is trusted, the user interface will not give any warnings when called by the contract.
        /// </summary>
        public WildcardContainer<ContractPermissionDescriptor> Trusts { get; set; }


        public string[] SupportedStandards
        {
            get;
            set;
        }

        /// <summary>
        /// Custom user data
        /// </summary>
        public object Extra { get; set; }

    }
}