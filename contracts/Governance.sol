pragma solidity ^0.8.4;

contract Governance {
    struct Proposal {
        uint id;
        string description;
        uint voteCount;
    }

    mapping(uint => Proposal) public proposals;
    uint public proposalCount;

    function createProposal(string memory description) public {
        proposalCount++;
        proposals[proposalCount] = Proposal(proposalCount, description, 0);
    }

    function vote(uint proposalId) public {
        Proposal storage proposal = proposals[proposalId];
        proposal.voteCount += 1; 
    }
}
