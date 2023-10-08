// Copyright (C) 2019-2023 Howard Wu
// This file is part of the credits library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use crate::{Authorized, Credits};

use rand::Rng;

/// Samples the authorization for `unbond_public`.
fn sample_unbond_public(rng: &mut TestRng) -> Authorized<CurrentNetwork> {
    // Sample the sender.
    let (sender_private_key, _) = sample_account(rng);
    // Sample the amount in microcredits.
    let amount_in_microcredits = rng.gen_range(0..1000000);
    // Sample the fee in microcredits.
    let fee_in_microcredits = rng.gen_range(1..1000000);

    // Sample the authorization.
    Credits::unbond_public(&sender_private_key.to_string(), amount_in_microcredits, fee_in_microcredits, false, rng)
        .unwrap()
}

#[test]
fn test_unbond_public_remote() {
    let rng = &mut TestRng::default();

    // Sample the authorization.
    let authorization = sample_unbond_public(rng);
    // Execute the authorization.
    let transaction = authorization.execute().unwrap();
    println!("{:?}", transaction);
}

#[test]
fn test_unbond_public_local() {
    let rng = &mut TestRng::default();

    // Sample the authorization.
    let authorization = sample_unbond_public(rng);
    // Execute the authorization.
    let transaction = authorization.execute_local::<CurrentAleo, _>(rng).unwrap();
    println!("{:?}", transaction);
}
