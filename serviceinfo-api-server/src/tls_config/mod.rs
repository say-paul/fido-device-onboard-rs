// #[cfg(feature = "openssl")]
mod server_config {
    use openssl::ssl::{SslContext, SslFiletype, SslMethod};
    use std::path::Path;

    pub type Acceptor = openssl::ssl::SslContext;

    fn tls_acceptor_impl<P: AsRef<Path>>(cert_file: P, key_file: P) -> Acceptor {
        let mut builder = SslContext::builder(SslMethod::tls_server()).unwrap();
        builder
            .set_certificate_file(cert_file, SslFiletype::ASN1)
            .unwrap();
        builder
            .set_private_key_file(key_file, SslFiletype::ASN1)
            .unwrap();
        builder.build()
    }

    pub fn tls_acceptor() -> Acceptor {
        tls_acceptor_impl("local.cert", "local.key")
    }
}

pub use server_config::*;
