use crate::action::{Action, CallActionData};

elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> SCResult<usize> {
        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).insert(caller_id);
        }

        Ok(action_id)
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    fn prepare_call_data(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        opt_function: OptionalArg<ManagedBuffer>,
        arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> CallActionData<Self::Api> {
        let endpoint_name = match opt_function {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };
        CallActionData {
            to,
            egld_amount,
            endpoint_name,
            arguments: arguments.into_vec_of_buffers(),
        }
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can send EGLD without calling anything.
    /// Can call smart contract endpoints directly.
    /// Doesn't really work with builtin functions.
    #[endpoint(proposeTransferExecute)]
    fn propose_transfer_execute(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let call_data = self.prepare_call_data(to, egld_amount, opt_function, arguments);
        self.propose_action(Action::SendTransferExecute(call_data))
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can call smart contract endpoints directly.
    /// Can use ESDTTransfer/ESDTNFTTransfer/MultiESDTTransfer to send tokens, while also optionally calling endpoints.
    /// Works well with builtin functions.
    /// Cannot simply send EGLD directly without calling anything.
    #[endpoint(proposeAsyncCall)]
    fn propose_async_call(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
        #[var_args] opt_function: OptionalArg<ManagedBuffer>,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        let call_data = self.prepare_call_data(to, egld_amount, opt_function, arguments);
        self.propose_action(Action::SendAsyncCall(call_data))
    }

    #[endpoint(proposeSCDeployFromSource)]
    fn propose_sc_deploy_from_source(
        &self,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCDeployFromSource {
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    #[endpoint(proposeSCUpgradeFromSource)]
    fn propose_sc_upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        amount: BigUint,
        source: ManagedAddress,
        code_metadata: CodeMetadata,
        #[var_args] arguments: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<usize> {
        self.propose_action(Action::SCUpgradeFromSource {
            sc_address,
            amount,
            source,
            code_metadata,
            arguments: arguments.into_vec_of_buffers(),
        })
    }

    /// Initiates delegation process.
    #[endpoint(proposeDelegate)]
    fn propose_delegate(
        &self, 
        provider_address: ManagedAddress, 
        amount: BigUint
    ) -> SCResult<usize> {
        self.propose_action(Action::Delegate {
            provider_address,
            amount,
        })
    }

    /// Initiates un-delegation process.
    #[endpoint(proposeUnDelegate)]
    fn propose_undelegate(
        &self, 
        provider_address: ManagedAddress, 
        amount: BigUint
    ) -> SCResult<usize> {
        self.propose_action(Action::UnDelegate {
            provider_address,
            amount,
        })
    }

    /// Initiates redelegation of rewards
    #[endpoint(proposeReDelegateRewards)]
    fn propose_redelegate_rewards(&self, provider_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::ReDelegateRewards(provider_address))
    }

    /// Initiates claiming rewards
    #[endpoint(proposeClaimRewards)]
    fn propose_claim_rewards(&self, provider_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::ClaimRewards(provider_address))
    }

    /// Initiates claiming rewards
    #[endpoint(proposeWithdraw)]
    fn propose_withdraw(&self, provider_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::Widthdraw(provider_address))
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Will send all contract funds to the addresses on the beneficiary list
    /// Amounts are according to the amount fractions associated with the beneficiary addresses.
    #[endpoint(proposeDistributeFunds)]
    fn propose_distribute_funds(
        &self,
    ) -> SCResult<usize> {
        self.propose_action(Action::DistributeFunds)
    }

    /// Initiates beneficiary addition process.
    #[endpoint(proposeAddBeneficiary)]
    fn propose_add_beneficiary(
        &self, 
        beneficiary_address: ManagedAddress, 
        amount_fraction: u32
    ) -> SCResult<usize> {
        self.propose_action(Action::AddBeneficiary {
            beneficiary_address,
            amount_fraction,
        })
    }

    /// Initiates beneficiary removal process.
    #[endpoint(proposeRemoveBeneficiary)]
    fn propose_remove_beneficiary(&self, beneficiary_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::RemoveBeneficiary(beneficiary_address))
    }
}
