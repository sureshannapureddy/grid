/*
 * Copyright 2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use sabre_sdk::protocol::payload::{
    CreateContractActionBuildError, CreateContractRegistryActionBuildError,
    CreateNamespaceRegistryActionBuildError, CreateNamespaceRegistryPermissionActionBuildError,
    SabrePayloadBuildError,
};
use sabre_sdk::protos::ProtoConversionError as SabreProtoConversionError;
use sawtooth_sdk::signing::Error as SigningError;
use splinter::{events, service::scabbard::client::Error as ScabbardError};
use std::error::Error;
use std::fmt;

use crate::event::EventIoError;
use crate::splinter::{app_auth_handler::node::GetNodeError, event::ScabbardEventConnectionError};

#[derive(Debug)]
pub enum AppAuthHandlerError {
    WebSocketError(events::WebSocketError),
    GetNodeError(GetNodeError),
    InvalidMessageError(String),
    ScabbardEventConnectionError(ScabbardEventConnectionError),
    EventIoError(EventIoError),
    EventProcessorError(String),
    SabreError(String),
    SawtoothError(String),
    SigningError(String),
    BatchSubmitError(String),
    ScabbardError(String),
}

impl Error for AppAuthHandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppAuthHandlerError::WebSocketError(err) => Some(err),
            AppAuthHandlerError::GetNodeError(err) => Some(err),
            AppAuthHandlerError::InvalidMessageError(_) => None,
            AppAuthHandlerError::ScabbardEventConnectionError(err) => Some(err),
            AppAuthHandlerError::EventIoError(err) => Some(err),
            AppAuthHandlerError::EventProcessorError(_) => None,
            AppAuthHandlerError::SabreError(_) => None,
            AppAuthHandlerError::SawtoothError(_) => None,
            AppAuthHandlerError::SigningError(_) => None,
            AppAuthHandlerError::BatchSubmitError(_) => None,
            AppAuthHandlerError::ScabbardError(_) => None,
        }
    }
}

impl fmt::Display for AppAuthHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppAuthHandlerError::WebSocketError(msg) => write!(f, "WebsocketError {}", msg),
            AppAuthHandlerError::GetNodeError(msg) => write!(f, "GetNodeError {}", msg),
            AppAuthHandlerError::InvalidMessageError(msg) => {
                write!(f, "The client received an invalid message: {}", msg)
            }
            AppAuthHandlerError::ScabbardEventConnectionError(msg) => {
                write!(f, "ScabbardEventConnectionError {}", msg)
            }
            AppAuthHandlerError::EventIoError(msg) => write!(f, "EventIoError {}", msg),
            AppAuthHandlerError::EventProcessorError(msg) => {
                write!(f, "Event processor error: {}", msg)
            }
            AppAuthHandlerError::SabreError(msg) => write!(
                f,
                "An error occurred while building a Sabre payload: {}",
                msg
            ),
            AppAuthHandlerError::SawtoothError(msg) => write!(
                f,
                "An error occurred while building a transaction or batch: {}",
                msg
            ),
            AppAuthHandlerError::SigningError(msg) => {
                write!(f, "A signing error occurred: {}", msg)
            }
            AppAuthHandlerError::BatchSubmitError(msg) => write!(
                f,
                "An error occurred while submitting a batch to the scabbard service: {}",
                msg
            ),
            AppAuthHandlerError::ScabbardError(msg) => {
                write!(f, "An error occurred in the Scabbard client: {}", msg)
            }
        }
    }
}

macro_rules! impl_from_sabre_errors {
    ($($x:ty),*) => {
        $(
            impl From<$x> for AppAuthHandlerError {
                fn from(e: $x) -> Self {
                    AppAuthHandlerError::SabreError(e.to_string())
                }
            }
        )*
    };
}

impl_from_sabre_errors!(
    CreateContractActionBuildError,
    CreateContractRegistryActionBuildError,
    CreateNamespaceRegistryActionBuildError,
    CreateNamespaceRegistryPermissionActionBuildError,
    SabreProtoConversionError,
    SabrePayloadBuildError
);

impl From<std::string::FromUtf8Error> for AppAuthHandlerError {
    fn from(err: std::string::FromUtf8Error) -> AppAuthHandlerError {
        AppAuthHandlerError::InvalidMessageError(format!("{}", err))
    }
}

impl From<events::WebSocketError> for AppAuthHandlerError {
    fn from(err: events::WebSocketError) -> Self {
        AppAuthHandlerError::WebSocketError(err)
    }
}

impl From<GetNodeError> for AppAuthHandlerError {
    fn from(err: GetNodeError) -> Self {
        AppAuthHandlerError::GetNodeError(err)
    }
}

impl From<ScabbardEventConnectionError> for AppAuthHandlerError {
    fn from(err: ScabbardEventConnectionError) -> Self {
        AppAuthHandlerError::ScabbardEventConnectionError(err)
    }
}

impl From<EventIoError> for AppAuthHandlerError {
    fn from(err: EventIoError) -> Self {
        AppAuthHandlerError::EventIoError(err)
    }
}

impl From<SigningError> for AppAuthHandlerError {
    fn from(err: SigningError) -> Self {
        AppAuthHandlerError::SigningError(err.to_string())
    }
}

impl From<ScabbardError> for AppAuthHandlerError {
    fn from(err: ScabbardError) -> Self {
        AppAuthHandlerError::ScabbardError(err.to_string())
    }
}
