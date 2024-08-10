using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Neo.Cryptography.ECC;
using Neo.Models;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;

namespace Neo.Services.ApiServices
{
    public class GovernanceService : ApiService
    {

        public async Task<List<string>> GetCommittees()
        {
            var points = NativeContract.NEO.GetCommittee(Helpers.GetDefaultSnapshot());
            return points.Select(p => p.ToVerificationContract().ScriptHash.ToAddress()).ToList();
        }


        public async Task<bool> IsCommittee()
        {
            if (CurrentWallet == null)
            {
                return false;
            }
            var points = NativeContract.NEO.GetCommittee(Helpers.GetDefaultSnapshot());
            var committees = points.Select(p => p.ToVerificationContract().ScriptHash.ToAddress()).ToList();
            return CurrentWallet.GetAccounts().Any(a => committees.Contains(a.Address));
        }


        #region DesignRole

        /// <summary>
        /// vote for consensus node
        /// </summary>
        /// <returns></returns>
        public async Task<object> DesignRole(Role role, string[] pubkeys, List<UInt160> signers = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            ECPoint[] publicKeys = null;
            try
            {
                publicKeys = pubkeys.Select(p => ECPoint.Parse(p, ECCurve.Secp256r1)).ToArray();
            }
            catch (Exception e)
            {
                return Error(ErrorCode.InvalidPara);
            }

            if (publicKeys.IsEmpty())
            {
                return Error(ErrorCode.InvalidPara);
            }


            var paras = new List<ContractParameter>();
            paras.Add(new ContractParameter(ContractParameterType.Integer) { Value = (int)role });
            paras.Add(new ContractParameter(ContractParameterType.Array)
            {
                Value = publicKeys.Select(p => new ContractParameter(ContractParameterType.PublicKey) { Value = p }).ToList()
            });
            using var sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.RoleManagement.Hash, "designateAsRole", paras.ToArray());

            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);

            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }


        /// <summary>
        /// query Designated Nodes by Role
        /// </summary>
        /// <param name="role"></param>
        /// <param name="height"></param>
        /// <returns></returns>
        public async Task<object> GetNodesByRole(Role role, uint? height = null)
        {
            var snapshot = Helpers.GetDefaultSnapshot();
            var points = NativeContract.RoleManagement.GetDesignatedByRole(snapshot, role, height ?? snapshot.GetHeight());
            return points?.Select(p => p.ToString()).ToList();
        }

        #endregion


        #region Policy


        public async Task<long> GetFeePerByte()
        {
            return NativeContract.Policy.GetFeePerByte(Helpers.GetDefaultSnapshot());
        }


        public async Task<object> SetFeePerByte(long fee, List<UInt160> signers = null)
        {
            if (fee < 0 || fee > 1_00000000)
            {
                return Error(ErrorCode.InvalidPara, $"input value should between 0 and  100,000,000");
            }
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            using ScriptBuilder sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.Policy.Hash, "setFeePerByte", new ContractParameter
            {
                Type = ContractParameterType.Integer,
                Value = fee
            });
            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);
            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }

        public async Task<object> GetExecFeeFactor()
        {
            return NativeContract.Policy.GetExecFeeFactor(Helpers.GetDefaultSnapshot());
        }

        public async Task<object> SetExecFeeFactor(uint factor, List<UInt160> signers = null)
        {
            if (factor == 0 || factor > PolicyContract.MaxExecFeeFactor)
            {
                return Error(ErrorCode.InvalidPara, $"input value should between 0 and  1000");
            }
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            using ScriptBuilder sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.Policy.Hash, "setExecFeeFactor", new ContractParameter
            {
                Type = ContractParameterType.Integer,
                Value = factor
            });
            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);
            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }


        public async Task<object> GetStoragePrice()
        {
            return NativeContract.Policy.GetStoragePrice(Helpers.GetDefaultSnapshot());
        }


        public async Task<object> SetStoragePrice(uint factor, List<UInt160> signers = null)
        {
            if (factor == 0 || factor > PolicyContract.MaxStoragePrice)
            {
                return Error(ErrorCode.InvalidPara, $"input value should between 0 and  10000000");
            }
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            using ScriptBuilder sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.Policy.Hash, "setStoragePrice", new ContractParameter
            {
                Type = ContractParameterType.Integer,
                Value = factor
            });
            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);
            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }

        public async Task<bool> IsBlocked(UInt160 account)
        {
            return NativeContract.Policy.IsBlocked(Helpers.GetDefaultSnapshot(), account);
        }


        public async Task<object> BlockAccount(UInt160 account, List<UInt160> signers = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            using ScriptBuilder sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.Policy.Hash, "blockAccount", new ContractParameter
            {
                Type = ContractParameterType.Hash160,
                Value = account
            });
            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);
            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }



        public async Task<object> UnblockAccount(UInt160 account, List<UInt160> signers = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            using ScriptBuilder sb = new ScriptBuilder();
            sb.EmitDynamicCall(NativeContract.Policy.Hash, "unblockAccount", new ContractParameter
            {
                Type = ContractParameterType.Hash160,
                Value = account
            });
            if (signers == null)
            {
                signers = new List<UInt160>();
            }
            var committee = NativeContract.NEO.GetCommitteeAddress(Helpers.GetDefaultSnapshot());
            signers.Add(committee);
            return await SignAndBroadcastTx(sb.ToArray(), signers.ToArray());
        }

        #endregion
    }
}
