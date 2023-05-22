//This Code Is For The Function "calculate_balance_changes"
function calculate_balance_changes(multi_send_tx, original_balances, burn_rate, commission_rate) {
  //Calculate Total Input 
  let total_input = 0;
  multi_send_tx.inputs.forEach(input => total_input += input.amount);

  //Calculate Total Output  
  let total_output = 0;
  multi_send_tx.outputs.forEach(output => total_output += output.amount);

  //Check if sum of inputs and outputs match
  if(total_input !== total_output) 
    return "Transaction rejected, sum of inputs and outputs does not match";

  //Calculate Burn Rate and Commission Rate as described above 
  let non_issuer_input_sum = 0, non_issuer_output_sum = 0;
  let burn_rate_output = 0, commission_rate_output = 0;

  multi_send_tx.inputs.forEach(input => {
    if(input.address !== multi_send_tx.issuer) //If input address is not an issuer  
      non_issuer_input_sum += input.amount;
  });

  multi_send_tx.outputs.forEach(output => {
    if(output.address !== multi_send_tx.issuer) //If output address is not an issuer  
      non_issuer_output_sum += output.amount;
  });

  //Apply burn_rate and commission_rate as per the definition
  burn_rate_output = burn_rate * (non_issuer_input_sum + non_issuer_output_sum);
  commission_rate_output = commission_rate * (non_issuer_input_sum + non_issuer_output_sum);

  //Calculate balance changes for every denom 
  let balance_changes = {};
  for(const denom in original_balances){
    let original_balance = original_balances[denom];
    let input_amount_with_burn_rate = 0;

    multi_send_tx.inputs.forEach(input => {
      if(input.address !== multi_send_tx.issuer && input.denom === denom){ //If input address is non-issuer and of same denom 
        input_amount_with_burn_rate += input.amount - (input.amount * burn_rate / total_input);
      }
    });

    let output_amount_with_commission_rate = 0;
    multi_send_tx.outputs.forEach(output => {
      if(output.address !== multi_send_tx.issuer && output.denom === denom){ //If output address is non-issuer and of same denom 
        output_amount_with_commission_rate += output.amount + (output.amount * commission_rate_output / total_output);
      }
    });
    
    //Calculate balance changes for the denom 
    let balance_change = output_amount_with_commission_rate - input_amount_with_burn_rate;
    if(original_balance < input_amount_with_burn_rate)
      return "Transaction rejected, sender does not have enough balances to cover the input amount";
    balance_changes[denom] = balance_change;
  }

  return balance_changes;
