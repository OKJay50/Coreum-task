# Calculate Balance Changes

This function takes in a multi_send_tx object, original_balances, burn_rate, and commission_rate as input and outputs the balance changes that must be applied to different accounts (negative means deduction, positive means addition), or an error. The error indicates that the transaction must be rejected. 

## Function Signature
```
function calculate_balance_changes(multi_send_tx: object, 
original_balances: object,
burn_rate: number,
commission_rate: number): object|string
```
Here is a brief description of the function parameters:
- `multi_send_tx`: A multi_send_tx object containing inputs and outputs of the transaction. 
- `original_balances`: An object with current balances for each denomination.
- `burn_rate`: A number representing the percentage of burn rate.
- `commission_rate`: A number representing the percentage of commission rate.


## Function Logic

Here are the key steps taken by the function:
1. Calculate the total input and total output of the transaction.
2. Check if the sum of inputs and outputs match. If not, return "Transaction rejected, sum of inputs and outputs does not match".
3. Calculate the burn rate and commission rate by following the steps given in the problem statement.
4. Calculate balance changes for every denomination.
5. Return balance changes if the sender has enough balances to cover the input amount. Otherwise, return "Transaction rejected, sender does not have enough balances to cover the input amount".

## Output
The output of the `calculate_balance_changes` function is an object containing the balance changes that should be applied to different accounts. The key for each denomination represents the denomination and the value represents the balance change. If there is an error in the transaction, the function returns a string with an error message.


## Example Usage

### Input
```
const multi_send_tx = {
  issuer: "issuer_address",
  inputs: [
    { address: "address_1", amount: 100, denom: "USD" },
    { address: "issuer_address", amount: 100, denom: "USD" }
  ],
  outputs: [
    { address: "address_2", amount: 150, denom: "USD" },
    { address: "address_3", amount: 50, denom: "USD" }
  ]
};
const original_balances = { USD: 200 }
const burn_rate = 0.01;
const commission_rate = 0.005;

const balance_changes = calculate_balance_changes(multi_send_tx, original_balances, burn_rate, commission_rate);
console.log(balance_changes);
```

### Output
```
{
  USD: -46.5
}
```
In this example, the output indicates that the balance of USD denomination for address_1 should be reduced by 46.5. 

## License
This code is released under the [MIT License](https://opensource.org/licenses/MIT).
