// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Treasury {
    address public admin;
    mapping(address => uint) public balances;

    event DepositReceived(address depositor, uint amount);
    event ExpenseApproved(uint amount, address recipient);

    constructor() {
        admin = msg.sender;
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "Only the admin can perform this action.");
        _;
    }

    function deposit() public payable {
        require(msg.value > 0, "Deposit must be greater than 0");
        balances[msg.sender] += msg.value;
        emit DepositReceived(msg.sender, msg.value);
    }

    function approveExpense(uint amount, address recipient) public onlyAdmin {
        require(address(this).balance >= amount, "Insufficient funds");
        payable(recipient).transfer(amount);
        emit ExpenseApproved(amount, recipient);
    }
}
