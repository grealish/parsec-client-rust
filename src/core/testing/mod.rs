// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
#![cfg(test)]
use super::basic_client::BasicClient;
use super::ipc_handler::{Connect, ReadWrite};
use crate::auth::AuthenticationData;
use crate::error::Result;
use mockstream::{FailingMockStream, SyncMockStream};
use parsec_interface::requests::ProviderID;
use std::ops::{Deref, DerefMut};

mod core_tests;

const DEFAULT_APP_NAME: &str = "default-test-app-name";

struct MockIpc(SyncMockStream);

impl Connect for MockIpc {
    fn connect(&self) -> Result<Box<dyn ReadWrite>> {
        Ok(Box::from(self.0.clone()))
    }
}

struct FailingMockIpc(FailingMockStream);

impl Connect for FailingMockIpc {
    fn connect(&self) -> Result<Box<dyn ReadWrite>> {
        Ok(Box::from(self.0.clone()))
    }
}

struct TestBasicClient {
    core_client: BasicClient,
    mock_stream: SyncMockStream,
}

impl TestBasicClient {
    pub fn set_mock_read(&mut self, bytes: &[u8]) {
        self.mock_stream.push_bytes_to_read(bytes);
    }

    pub fn get_mock_write(&mut self) -> Vec<u8> {
        self.mock_stream.pop_bytes_written()
    }
}

impl Deref for TestBasicClient {
    type Target = BasicClient;

    fn deref(&self) -> &Self::Target {
        &self.core_client
    }
}

impl DerefMut for TestBasicClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core_client
    }
}

impl Default for TestBasicClient {
    fn default() -> Self {
        let mut client = TestBasicClient {
            core_client: BasicClient::new(
                AuthenticationData::AppIdentity(String::from(DEFAULT_APP_NAME)),
                ProviderID::Pkcs11,
            ),
            mock_stream: SyncMockStream::new(),
        };

        client
            .core_client
            .set_ipc_handler(Box::from(MockIpc(client.mock_stream.clone())));
        client
    }
}
