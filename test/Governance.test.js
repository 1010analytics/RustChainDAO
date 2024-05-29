const Governance = artifacts.require("Governance");

module.exports = function (deployer) {
  deployer.deploy(Governance);
};

contract("Governance", (accounts) => {
  let governance;

  before(async () => {
    governance = await Governance.deployed();
  });

  describe("Proposal creation", () => {
    it("should create a proposal correctly", async () => {
      await governance.createProposal("Test proposal");
      const proposal = await governance.proposals(1);
      assert.equal(
        proposal.description,
        "Test proposal",
        "Description should match"
      );
      assert.equal(proposal.voteCount, 0, "Initial vote count should be zero");
    });
  });

  describe("Voting mechanism", () => {
    it("should allow voting on a proposal", async () => {
      await governance.vote(1);
      const proposal = await governance.proposals(1);
      assert.equal(proposal.voteCount, 1, "Vote count should increment");
    });

    it("should not allow voting on a non-existent proposal", async () => {
      try {
        await governance.vote(2);
        assert.fail("Should have thrown an error");
      } catch (error) {
        assert.include(error.message, "revert", "Expected revert error");
      }
    });
  });
});
