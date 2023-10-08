use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub timezone_offset: i64,
    pub current: Option<CurrentWeather>,
    pub minutely: Option<Vec<MinuteForecast>>,
    pub hourly: Option<Vec<HourlyForecast>>,
    pub daily: Option<Vec<DailyForecast>>,
    pub alerts: Option<Vec<Alert>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u16,
    pub humidity: u8,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u8,
    pub visibility: u16,
    pub wind_speed: f32,
    pub wind_deg: u16,
    pub wind_gust: Option<f32>,
    pub weather: Vec<WeatherCondition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinuteForecast {
    pub dt: u64,
    pub precipitation: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HourlyForecast {
    pub dt: u64,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u16,
    pub humidity: u8,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u8,
    pub visibility: u16,
    pub wind_speed: f32,
    pub wind_deg: u16,
    pub wind_gust: Option<f32>,
    pub weather: Vec<WeatherCondition>,
    pub pop: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DailyForecast {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub moonrise: u64,
    pub moonset: u64,
    pub moon_phase: f32,
    pub summary: String,
    pub temp: Temperature,
    pub feels_like: FeelsLikeTemperature,
    pub pressure: u16,
    pub humidity: u8,
    pub dew_point: f32,
    pub wind_speed: f32,
    pub wind_deg: u16,
    pub wind_gust: Option<f32>,
    pub weather: Vec<WeatherCondition>,
    pub clouds: u8,
    pub pop: f32,
    pub rain: Option<f32>,
    pub snow: Option<f32>,
    pub uvi: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Temperature {
    pub morn: f32,
    pub day: f32,
    pub eve: f32,
    pub night: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeelsLikeTemperature {
    pub morn: f32,
    pub day: f32,
    pub eve: f32,
    pub night: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherCondition {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alert {
    pub sender_name: String,
    pub event: String,
    pub start: u64,
    pub end: u64,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub enum WeatherConditionId {
    ThunderstormWithLightRain,
    ThunderstormWithRain,
    ThunderstormWithHeavyRain,
    LightThunderstorm,
    Thunderstorm,
    HeavyThunderstorm,
    RaggedThunderstorm,
    ThunderstormWithLightDrizzle,
    ThunderstormWithDrizzle,
    ThunderstormWithHeavyDrizzle,
    LightIntensityDrizzle,
    Drizzle,
    HeavyIntensityDrizzle,
    LightIntensityDrizzleRain,
    DrizzleRain,
    HeavyIntensityDrizzleRain,
    ShowerRainAndDrizzle,
    HeavyShowerRainAndDrizzle,
    ShowerDrizzle,
    LightRain,
    ModerateRain,
    HeavyIntensityRain,
    VeryHeavyRain,
    ExtremeRain,
    FreezingRain,
    LightIntensityShowerRain,
    ShowerRain,
    HeavyIntensityShowerRain,
    RaggedShowerRain,
    LightSnow,
    Snow,
    HeavySnow,
    Sleet,
    LightShowerSleet,
    ShowerSleet,
    LightRainAndSnow,
    RainAndSnow,
    LightShowerSnow,
    ShowerSnow,
    HeavyShowerSnow,
    Mist,
    Smoke,
    Haze,
    SandOrDustWhirls,
    Fog,
    Sand,
    Dust,
    VolcanicAsh,
    Squalls,
    Tornado,
    ClearSky,
    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    OvercastClouds,
}

impl WeatherCondition {
    pub fn get_condition(&self) -> Option<WeatherConditionId> {
        match self.id {
            200 => Some(WeatherConditionId::ThunderstormWithLightRain),
            201 => Some(WeatherConditionId::ThunderstormWithRain),
            202 => Some(WeatherConditionId::ThunderstormWithHeavyRain),
            210 => Some(WeatherConditionId::LightThunderstorm),
            211 => Some(WeatherConditionId::Thunderstorm),
            212 => Some(WeatherConditionId::HeavyThunderstorm),
            221 => Some(WeatherConditionId::RaggedThunderstorm),
            230 => Some(WeatherConditionId::ThunderstormWithLightDrizzle),
            231 => Some(WeatherConditionId::ThunderstormWithDrizzle),
            232 => Some(WeatherConditionId::ThunderstormWithHeavyDrizzle),
            300 => Some(WeatherConditionId::LightIntensityDrizzle),
            301 => Some(WeatherConditionId::Drizzle),
            302 => Some(WeatherConditionId::HeavyIntensityDrizzle),
            310 => Some(WeatherConditionId::LightIntensityDrizzleRain),
            311 => Some(WeatherConditionId::DrizzleRain),
            312 => Some(WeatherConditionId::HeavyIntensityDrizzleRain),
            313 => Some(WeatherConditionId::ShowerRainAndDrizzle),
            314 => Some(WeatherConditionId::HeavyShowerRainAndDrizzle),
            321 => Some(WeatherConditionId::ShowerDrizzle),
            500 => Some(WeatherConditionId::LightRain),
            501 => Some(WeatherConditionId::ModerateRain),
            502 => Some(WeatherConditionId::HeavyIntensityRain),
            503 => Some(WeatherConditionId::VeryHeavyRain),
            504 => Some(WeatherConditionId::ExtremeRain),
            511 => Some(WeatherConditionId::FreezingRain),
            520 => Some(WeatherConditionId::LightIntensityShowerRain),
            521 => Some(WeatherConditionId::ShowerRain),
            522 => Some(WeatherConditionId::HeavyIntensityShowerRain),
            531 => Some(WeatherConditionId::RaggedShowerRain),
            600 => Some(WeatherConditionId::LightSnow),
            601 => Some(WeatherConditionId::Snow),
            602 => Some(WeatherConditionId::HeavySnow),
            611 => Some(WeatherConditionId::Sleet),
            612 => Some(WeatherConditionId::LightShowerSleet),
            613 => Some(WeatherConditionId::ShowerSleet),
            615 => Some(WeatherConditionId::LightRainAndSnow),
            616 => Some(WeatherConditionId::RainAndSnow),
            620 => Some(WeatherConditionId::LightShowerSnow),
            621 => Some(WeatherConditionId::ShowerSnow),
            622 => Some(WeatherConditionId::HeavyShowerSnow),
            701 => Some(WeatherConditionId::Mist),
            711 => Some(WeatherConditionId::Smoke),
            721 => Some(WeatherConditionId::Haze),
            731 => Some(WeatherConditionId::SandOrDustWhirls),
            741 => Some(WeatherConditionId::Fog),
            751 => Some(WeatherConditionId::Sand),
            761 => Some(WeatherConditionId::Dust),
            762 => Some(WeatherConditionId::VolcanicAsh),
            771 => Some(WeatherConditionId::Squalls),
            781 => Some(WeatherConditionId::Tornado),
            800 => Some(WeatherConditionId::ClearSky),
            801 => Some(WeatherConditionId::FewClouds),
            802 => Some(WeatherConditionId::ScatteredClouds),
            803 => Some(WeatherConditionId::BrokenClouds),
            804 => Some(WeatherConditionId::OvercastClouds),
            _ => None,
        }
    }
}