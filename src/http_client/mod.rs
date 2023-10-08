use std::str;
use anyhow::{Result, bail};
use embedded_svc::http::client::Client;
use embedded_svc::io::Read;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

pub fn get(url: impl AsRef<str>) -> Result<String> {
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    let mut client = Client::wrap(connection);
    let request = client.get(url.as_ref())?;

    let response = request.submit()?;
    let status = response.status();

    match status {
        200..=299 => {
            let mut buf = [0_u8; 256];
            let mut reader = response;
            let mut response_text = String::new();
            loop {
                if let Ok(size) = Read::read(&mut reader, &mut buf) {
                    if size == 0 {
                        break;
                    }
                    response_text.push_str(str::from_utf8(&buf[..size])?);
                }
            }
            Ok(response_text)
        }
        _ => bail!("Unexpected response code: {} from {}", status, url.as_ref()),
    }
}