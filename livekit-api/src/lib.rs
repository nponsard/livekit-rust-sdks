// Copyright 2023 LiveKit, Inc.
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

#[cfg_attr(
    feature = "access-token",
    deprecated(note = "access-token feature is no longer optional, and so is deprecated")
)]
pub mod access_token;

#[cfg(any(
    feature = "services-tokio",
    feature = "services-async",
    feature = "services-dispatcher"
))]
pub mod services;

#[cfg(any(
    feature = "signal-client-tokio",
    feature = "signal-client-async",
    feature = "signal-client-dispatcher"
))]
pub mod signal_client;

#[cfg(any(
    feature = "signal-client-tokio",
    feature = "signal-client-async",
    feature = "signal-client-dispatcher",
    feature = "services-tokio",
    feature = "services-async",
    feature = "services-dispatcher",
))]
mod http_client;

#[cfg(any(feature = "signal-client-dispatcher", feature = "services-dispatcher",))]
pub use http_client::{set_http_client, HttpClient, Response};

#[cfg(feature = "webhooks")]
pub mod webhooks;

#[allow(dead_code)]
pub(crate) fn get_env_keys() -> Result<(String, String), std::env::VarError> {
    let api_key = std::env::var("LIVEKIT_API_KEY")?;
    let api_secret = std::env::var("LIVEKIT_API_SECRET")?;
    Ok((api_key, api_secret))
}
