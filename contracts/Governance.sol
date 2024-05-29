// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Governance {
    struct Proposal {
        uint id;
        string description;
        uint voteCount;
    }

    mapping(uint => Proposal) public proposals;
    uint public proposalCount;
    address public chairperson;

    event ProposalCreated(uint proposalId, string description);
    event Voted(uint proposalId, uint voteCount);

    modifier onlyChairperson() {
        require(msg.sender == chairperson, "Only the chairperson can perform this action.");
        _;
    }

    constructor() {
        chairperson = msg.sender;
    }

    function createProposal(string memory description) public onlyChairperson {
        proposalCount++;
        proposals[proposalCount] = Proposal(proposalCount, description, 0);
        emit ProposalCreated(proposalCount, description);
    }

    function vote(uint proposalId) public {
        Proposal storage proposal = proposals[proposalId];
        proposal.voteCount += 1;
        emit Voted(proposalId, proposal.voteCount);
    }
}
