#![cfg_attr(not(creusot), feature(stmt_expr_attributes, proc_macro_hygiene))]

use creusot_contracts::*;

#[requires(x < i32::MAX)]
#[ensures(result@ == x@ + 1)]
pub fn add_one(x: i32) -> i32 {
    x + 1
}
