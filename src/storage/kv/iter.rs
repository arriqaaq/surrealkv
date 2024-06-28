use std::{collections::VecDeque, f64::consts::E, ops::RangeBounds};

use bytes::Bytes;
use vart::{iter::{Iter}, KeyTrait, VariableSizeKey, art::{Node}};
use crate::storage::kv::snapshot::Snapshot;

use crate::{Result, Error, Transaction};
use crate::storage::kv::{
    entry::{Entry, Value, ValueRef},
    store::Core,
    util::{now, sha256},
};

use std::sync::Arc;

pub struct TransactionIterator<'a> {
    transaction: &'a Transaction,
    iter: Iter<'a, VariableSizeKey, Bytes>,
}

impl Transaction {
    pub fn iter(&self) -> Result<TransactionIterator> {
        if self.mode.is_write_only() {
            return Err(Error::TransactionReadOnly);
        }

        let iter = self.snapshot.as_ref().unwrap().write().iter();
        
        Ok(TransactionIterator {
            transaction: self,
            iter,
        })
    }
}
