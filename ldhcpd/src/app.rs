use crate::{routes, LDHCPD};
use huntsman_http::{HTTPClientAddress, HTTPStatus, HTTP};
use oak::{error, info};

impl huntsman::App for LDHCPD {
    type Protocol = HTTP;

    type Client = HTTPClientAddress;

    async fn on_server_start(
        self: &std::sync::Arc<Self>,
        addresses: &[<Self::Protocol as huntsman::Protocol>::ListenAddress],
    ) {
        info!(self.init_logger, "Server listening on {:?}", addresses)
    }

    async fn on_client_connect<'a>(
        self: &'a std::sync::Arc<Self>,
        source: <Self::Protocol as huntsman::Protocol>::ClientAddress,
    ) -> Option<Self::Client> {
        info!(self.connection_logger, "Client connected from {}", source);
        Some(source)
    }

    async fn accept_error(
        self: &std::sync::Arc<Self>,
        error: <Self::Protocol as huntsman::Protocol>::ListenError,
    ) {
        error!(
            self.connection_logger,
            "Unable to accept a client - {}", error
        );
    }

    async fn handle_request<'a, 'b>(
        self: &'a std::sync::Arc<Self>,
        client: &'a mut Self::Client,
        request: <Self::Protocol as huntsman::Protocol>::Request<'b>,
    ) -> <Self::Protocol as huntsman::Protocol>::Response<'a> {
        let response = routes::route();

        info!(
            self.request_logger,
            huntsman_http::HTTPRequestDisplay::new(
                &request,
                *client,
                if self.log_reponses {
                    Some(response.status())
                } else {
                    None
                },
                self.log_headers,
                self.log_bodies
            )
        );

        response
    }

    async fn on_client_disconnect(self: &std::sync::Arc<Self>, client: &mut Self::Client) {
        info!(self.connection_logger, "Client disconnected ({})", client);
    }

    async fn read_error<'a>(
        self: &'a std::sync::Arc<Self>,
        client: &'a mut Self::Client,
        error: <Self::Protocol as huntsman::Protocol>::ReadError,
    ) -> Option<<Self::Protocol as huntsman::Protocol>::Response<'a>> {
        error!(
            self.connection_logger,
            "Unable to read request from {} - {}", client, error
        );
        Some(HTTPStatus::BadRequest.into())
    }

    async fn send_error(
        self: &std::sync::Arc<Self>,
        client: &mut Self::Client,
        error: <Self::Protocol as huntsman::Protocol>::SendError,
    ) {
        error!(
            self.connection_logger,
            "Unable to send response to {} - {}", client, error
        );
    }
}
