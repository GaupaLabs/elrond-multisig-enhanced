////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    multisig_enhanced
    (
        deposit
        discardAction
        getActionLastIndex
        getBeneficiaries
        getNumBoardMembers
        getNumProposers
        getQuorum
        getUserFraction
        performAction
        proposeAddBeneficiary
        proposeAddBoardMember
        proposeAddProposer
        proposeAsyncCall
        proposeChangeQuorum
        proposeClaimRewards
        proposeDelegate
        proposeDistributeFunds
        proposeReDelegateRewards
        proposeRemoveBeneficiary
        proposeRemoveUser
        proposeSCDeployFromSource
        proposeSCUpgradeFromSource
        proposeTransferExecute
        proposeUnDelegate
        proposeWithdraw
        quorumReached
        sign
        signed
        unsign
    )
}

elrond_wasm_node::wasm_empty_callback! {}
