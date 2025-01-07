use std::{io, net::Shutdown, path::Path};

use smol::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::unix::UnixStream,
};
use snafu::Snafu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthFlowState {
    Uninitialized,
    WaitForResponse,
    SendAuthResponse,
    SendEmptyResponse,
    Authenticated,
    Failed,
}

#[derive(Debug, Clone)]
pub enum RequestedAction {
    SendDataFromPrompt { prompt: String, visible: bool },
    DisplayMessage(String),
    None,
    ExitApplication,
}

#[derive(Debug, Clone)]
pub struct GreetdSession {
    socket: UnixStream,

    pub failed_attempts: u32,
    state: AuthFlowState,
}
impl GreetdSession {
    pub async fn new(socket_path: impl AsRef<Path>) -> io::Result<Self> {
        let socket = UnixStream::connect(socket_path).await?;
        Ok(Self {
            socket,
            failed_attempts: 0,
            state: AuthFlowState::Uninitialized,
        })
    }

    async fn read_response_packet(&mut self) -> io::Result<greetd_ipc::Response> {
        let mut packet_size = [0u8; 4];
        self.socket.read_exact(&mut packet_size).await?;
        let packet_size = u32::from_le_bytes(packet_size) as usize;

        let mut packet = vec![0u8; packet_size];
        self.socket.read_exact(&mut packet).await?;

        serde_json::from_slice(&packet).map_err(|de| io::Error::new(io::ErrorKind::InvalidData, de))
    }
    async fn send_packet(&mut self, packet: &greetd_ipc::Request) -> io::Result<()> {
        let packet = serde_json::to_vec(packet)
            .map_err(|se| io::Error::new(io::ErrorKind::InvalidData, se))?;
        let packet_size = (packet.len() as u32).to_le_bytes();

        self.socket.write_all(&packet_size).await?;
        self.socket.write_all(&packet).await?;
        Ok(())
    }

    pub async fn shutdown_session(&mut self) -> io::Result<()> {
        let packet = greetd_ipc::Request::CancelSession;
        self.send_packet(&packet).await?;
        self.socket.shutdown(Shutdown::Both)
    }

    pub async fn step_statemachine(
        &mut self,
        user: &str,
        data: Option<&str>,
        command: &str,
    ) -> Result<RequestedAction, GreeterError> {
        match self.state {
            AuthFlowState::Uninitialized => {
                let packet = greetd_ipc::Request::CreateSession {
                    username: user.to_string(),
                };
                self.send_packet(&packet).await?;
                self.state = AuthFlowState::WaitForResponse;
                Ok(RequestedAction::None)
            }
            AuthFlowState::WaitForResponse => {
                let response = self.read_response_packet().await?;
                match response {
                    greetd_ipc::Response::AuthMessage {
                        auth_message_type,
                        auth_message,
                    } => match auth_message_type {
                        greetd_ipc::AuthMessageType::Visible => {
                            self.state = AuthFlowState::SendAuthResponse;
                            Ok(RequestedAction::SendDataFromPrompt {
                                prompt: auth_message,
                                visible: true,
                            })
                        }
                        greetd_ipc::AuthMessageType::Secret => {
                            self.state = AuthFlowState::SendAuthResponse;
                            Ok(RequestedAction::SendDataFromPrompt {
                                prompt: auth_message,
                                visible: false,
                            })
                        }
                        greetd_ipc::AuthMessageType::Info | greetd_ipc::AuthMessageType::Error => {
                            self.state = AuthFlowState::SendEmptyResponse;
                            Ok(RequestedAction::DisplayMessage(auth_message))
                        }
                    },
                    greetd_ipc::Response::Error {
                        error_type,
                        description,
                    } => {
                        self.state = AuthFlowState::Failed;
                        Err(GreeterError::GreetdError {
                            message: format!("{:?}: {}", error_type, description),
                        })
                    }
                    greetd_ipc::Response::Success => {
                        self.state = AuthFlowState::Authenticated;
                        Ok(RequestedAction::None)
                    }
                }
            }
            AuthFlowState::SendEmptyResponse => {
                let packet = greetd_ipc::Request::PostAuthMessageResponse { response: None };
                self.send_packet(&packet).await?;
                self.state = AuthFlowState::WaitForResponse;
                Ok(RequestedAction::None)
            }
            AuthFlowState::SendAuthResponse => {
                let response = data.ok_or(GreeterError::MissingData)?;
                let packet = greetd_ipc::Request::PostAuthMessageResponse {
                    response: Some(response.to_string()),
                };
                self.send_packet(&packet).await?;
                self.state = AuthFlowState::WaitForResponse;
                Ok(RequestedAction::None)
            }

            AuthFlowState::Authenticated => {
                let packet = greetd_ipc::Request::StartSession {
                    cmd: vec![command.to_string()],
                    env: vec![],
                };
                self.send_packet(&packet).await?;
                Ok(RequestedAction::ExitApplication)
            }
            AuthFlowState::Failed => {
                _ = self.shutdown_session().await;
                Err(GreeterError::SessionShutdown)
            }
        }
    }
}

#[derive(Debug, Snafu)]
pub enum GreeterError {
    #[snafu(transparent)]
    Io { source: io::Error },
    /// Did not provide requested data
    MissingData,
    /// Non fatal authentication error
    FailedToAuthenticate,
    /// Possible fatal error
    GreetdError { message: String },
    /// Fatal error
    SessionShutdown,
}
