pragma solidity ^0.8.4;

contract Treasury {
    address public admin;
    mapping(address => uint) public balances;

    constructor() {
        admin = msg.sender;
    }

    function deposit() public payable {
        require(msg.value > 0, "Deposit must be greater than 0");
        balances[msg.sender] += msg.value;
    }

    function approveExpense(uint amount, address recipient) public {
        require(msg.sender == admin, "Only admin can approve expenses");
        require(address(this).balance >= amount, "Insufficient funds");
        payable(recipient).transfer(amount);
    }
}
