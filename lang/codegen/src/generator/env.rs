// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::GenerateCode;
use derive_more::From;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code for the ink! environment of the contract.
#[derive(From)]
pub struct Env<'a> {
    contract: &'a ir::Contract,
}

impl GenerateCode for Env<'_> {
    fn generate_code(&self) -> TokenStream2 {
        let env_types = self.contract.config().env_types();
        quote! {
            type EnvTypes = #env_types;

            type AccountId = <#env_types as ::ink_core::env::EnvTypes>::AccountId;
            type Balance = <#env_types as ::ink_core::env::EnvTypes>::Balance;
            type Hash = <#env_types as ::ink_core::env::EnvTypes>::Hash;
            type Timestamp = <#env_types as ::ink_core::env::EnvTypes>::Timestamp;
            type BlockNumber = <#env_types as ::ink_core::env::EnvTypes>::BlockNumber;
        }
    }
}