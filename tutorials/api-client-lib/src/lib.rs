/*
    Copyright 2019 Supercomputing Systems AG
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use codec::{Decode, Encode};
use keyring::AccountKeyring;

use primitives::crypto::Pair;
use primitives::sr25519;

use substrate_api_client::{
    Api,
    compose_extrinsic,
    extrinsic::xt_primitives::UncheckedExtrinsicV3,
    utils::{hexstr_to_u64, hexstr_to_vec}
};

#[derive(Encode, Decode, Debug)]
struct Kitty {
    id: [u8; 32],
    price: u128,
}