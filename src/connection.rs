use quinn::{ClientConfig, Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::{error::Error, net::SocketAddr, sync::Arc};

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

  Ok(ClientConfig::with_root_certificates(Arc::new(certs))?)
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

  let mut server_config = ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
  let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
  transport_config.max_concurrent_uni_streams(0_u8.into());

  Ok((server_config, cert_der))
}
