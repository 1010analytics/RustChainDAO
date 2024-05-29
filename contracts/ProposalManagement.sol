pragma solidity ^0.8.4;

contract ProposalManagement {
    struct Proposal {
        uint id;
        string description;
        bool active;
    }

    Proposal[] public proposals;

    function createProposal(string memory description) public {
        proposals.push(Proposal(proposals.length, description, true));
    }

    function deactivateProposal(uint proposalId) public {
        proposals[proposalId].active = false;
    }
}
