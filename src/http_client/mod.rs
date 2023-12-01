use anyhow::{bail, Result};
use embedded_svc::{http::client::Client as HttpClient, io::Read, io::Write};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use std::str;

pub fn post(url: impl AsRef<str>, payload: &[u8]) -> Result<String> {
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    let headers = [("content-type", "application/octet-stream")];

    let mut client = HttpClient::wrap(connection);

    let mut request = client.post(url.as_ref(), &headers)?;
    request.write_all(&payload)?;
    request.flush()?;

    log::info!("-> POST {}", url.as_ref());
    let response = request.submit()?;
    let status = response.status();
    log::info!("<- {}", status);

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

pub fn get(url: impl AsRef<str>) -> Result<String> {
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    let mut client = HttpClient::wrap(connection);
    let request = client.get(url.as_ref())?;

    log::info!("-> GET {}", url.as_ref());
    let response = request.submit()?;
    let status = response.status();
    log::info!("<- {}", status);

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
