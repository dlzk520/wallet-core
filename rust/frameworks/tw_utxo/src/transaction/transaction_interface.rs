// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::encode::Encodable;
use crate::script::{Script, Witness};
use crate::transaction::transaction_parts::OutPoint;

/// An interface of the Bitcoin transaction.
pub trait TransactionInterface: Clone + Encodable {
    type Input: TxInputInterface;
    type Output: TxOutputInterface;

    fn version(&self) -> i32;

    fn inputs(&self) -> &[Self::Input];

    fn inputs_mut(&mut self) -> &mut [Self::Input];

    fn set_inputs(&mut self, inputs: Vec<Self::Input>);

    fn outputs(&self) -> &[Self::Output];

    fn set_outputs(&mut self, outputs: Vec<Self::Output>);

    fn has_witness(&self) -> bool;

    fn locktime(&self) -> u32;
}

pub trait TxInputInterface: Clone {
    fn previous_output(&self) -> &OutPoint;

    fn sequence(&self) -> u32;

    fn set_sequence(&mut self, sequence: u32);

    fn set_script_sig(&mut self, script_sig: Script);

    fn set_witness(&mut self, witness: Witness);

    fn witness_items(&self) -> &[Script];

    fn has_witness(&self) -> bool;

    fn clear_witness(&mut self);
}

pub trait TxOutputInterface: Clone + Default + Encodable {}
