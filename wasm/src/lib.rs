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
        getNumBoardMembers
        getNumProposers
        getQuorum
        performAction
        proposeAddBoardMember
        proposeAddProposer
        proposeAsyncCall
        proposeChangeQuorum
        proposeClaimRewards
        proposeDelegate
        proposeReDelegateRewards
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
