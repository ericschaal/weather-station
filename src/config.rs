#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    owm_api_key: &'static str,
    #[default("Montreal, Quebec")]
    location_name: &'static str,
    #[default(45.5019)]
    latitude: f32,
    #[default(-73.5674)]
    longitude: f32,
    #[default(8)]
    hours_to_draw: usize,
}
