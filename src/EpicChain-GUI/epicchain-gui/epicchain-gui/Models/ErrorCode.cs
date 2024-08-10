using System.ComponentModel;

namespace Neo.Models
{
    public enum ErrorCode
    {
        [Description("Invalid parameter.")]
        InvalidPara = 20000,
        [Description("Wallet file is not exist.")]
        WalletFileNotFound = 20001,
        [Description("Wrong password")]
        WrongPassword = 20002,
        [Description("Failed to open the wallet.")]
        FailToOpenWallet = 20003,
        [Description("Wallet should be open first.")]
        WalletNotOpen = 20004,
        [Description("Method not found.")]
        MethodNotFound = 20005,
        [Description("Parameter cannot be empty.")]
        ParameterIsNull = 20006,
        [Description("Invalid private key.")]
        InvalidPrivateKey = 20007,
        [Description("Transaction is not exist.")]
        TxIdNotFound = 20008,
        [Description("Address is not exist.")]
        AddressNotFound = 20009,
        [Description("Address's private key is not exist here.")]
        AddressNotFoundPrivateKey = 20010,
        [Description("The block height is invalid.")]
        BlockHeightInvalid = 20011,
        [Description("The block hash is invalid.")]
        BlockHashInvalid = 20012,
        [Description("Balance is not enough.")]
        BalanceNotEnough = 20013,
        [Description("Sign fail.")]
        SignFail = 20014,
        [Description("Create multi address fail.")]
        CreateMultiContractFail = 20015,
        [Description("Create contract address fail.")]
        CreateContractAddressFail = 20016,
        [Description("Cliam gas fail.")]
        ClaimGasFail = 20017,
        [Description("Insufficient GAS.")]
        GasNotEnough = 20018,
        [Description("Transfer Error.")]
        TransferError = 20019,
        [Description("File does not exist.")]
        FileNotExist = 20020,
        [Description("Transaction exceed MaxTransactionSize.")]
        ExceedMaxTransactionSize = 20021,
        [Description("Invalid opcode.")]
        InvalidOpCode = 20022,
        [Description("Invalid ContractScript.")]
        InvalidContractScript = 20023,
        [Description("Manifest file is invalid Json file.")]
        InvalidManifestFile = 20024,
        [Description("Nef file is invalid file.")]
        InvalidNefFile = 20025,
        [Description("Engine faulted.")]
        EngineFault = 20026,
        [Description("Failed execution contract.")]
        ExecuteContractFail = 20027,
        [Description("Unknown contract. ")]
        UnknownContract = 20028,
        [Description("Contract already onchain.")]
        ContractAlreadyExist = 20029,
        [Description("Validator already exists.")]
        ValidatorAlreadyExist = 20030,
        [Description("No gas for claim.")]
        NoNeedClaimGas = 20031,
    }
}