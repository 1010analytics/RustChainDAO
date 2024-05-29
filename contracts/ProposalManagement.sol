// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract ProposalManagement {
    struct Proposal {
        uint id;
        string description;
        bool active;
    }

    Proposal[] public proposals;
    event ProposalStatusChanged(uint proposalId, bool isActive);

    function createProposal(string memory description) public {
        proposals.push(Proposal(proposals.length, description, true));
        emit ProposalStatusChanged(proposals.length - 1, true);
    }

    function deactivateProposal(uint proposalId) public {
        proposals[proposalId]. active = false;
        emit ProposalStatusChanged(proposalId, false);
    }
}
