use quinn::{ClientConfig, Endpoint, ServerConfig, TransportConfig};
use quinn::crypto::rustls::{QuicClientConfig, QuicServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::{error::Error, net::SocketAddr, sync::Arc};

const ALPN_RTP_OVER_QUIC: &[u8] = b"roq";

#[allow(unused)]
pub fn make_client_endpoint(
  bind_addr: SocketAddr,
  server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
  let client_cfg = configure_client(server_certs)?;
  let mut endpoint = Endpoint::client(bind_addr)?;
  endpoint.set_default_client_config(client_cfg);
  Ok(endpoint)
}

fn configure_client(
server_certs: &[&[u8]],
) -> Result<ClientConfig, Box<dyn Error + Send + Sync + 'static>> {
  let mut certs = rustls::RootCertStore::empty();
  for cert in server_certs {
    certs.add(CertificateDer::from(*cert))?;
  }

  let mut client_crypto = rustls::ClientConfig::builder()
    .with_root_certificates(certs)
    .with_no_client_auth();
  client_crypto.alpn_protocols = vec![ALPN_RTP_OVER_QUIC.to_vec()];

  let mut client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));

  let mut transport = TransportConfig::default();
  transport.max_concurrent_uni_streams(1024u32.into()); // allow stream
  transport.datagram_receive_buffer_size(Some(1024)); // allow datagram

  client_config.transport_config(Arc::new(transport));

  Ok(client_config)
}

#[allow(unused)]
pub fn make_server_endpoint(
  bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
  let (server_config, server_cert) = configure_server()?;
  let endpoint = Endpoint::server(server_config, bind_addr)?;
  Ok((endpoint, server_cert))
}

fn configure_server(
) -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
  let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
  let cert_der = CertificateDer::from(cert.cert);
  let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

  let mut server_crypto = rustls::ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(vec![cert_der.clone()], priv_key.into())?;
  server_crypto.alpn_protocols = vec![ALPN_RTP_OVER_QUIC.to_vec()];

  let mut server_config = ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

  let mut transport = TransportConfig::default();
  transport.max_concurrent_uni_streams(1024u32.into());
  transport.datagram_receive_buffer_size(Some(1024));

  server_config.transport_config(Arc::new(transport));

  Ok((server_config, cert_der))
}
