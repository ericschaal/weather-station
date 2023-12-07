use crate::icons::{IconSize, WeatherIcon, WeatherIconSet, ICONS};
use crate::owm::model::{CurrentWeather, DailyForecast, WeatherConditionId};
use tinyqoi::Qoi;

pub fn get_icon<'a>(icon: &'a WeatherIcon, night: bool, cloudy: bool, windy: bool) -> &Qoi<'a> {
    if cloudy {
        if windy {
            &icon.cloudy.windy
        } else {
            &icon.cloudy.clear
        }
    } else if night {
        if windy {
            &icon.night.windy
        } else {
            &icon.night.clear
        }
    } else {
        if windy {
            &icon.day.windy
        } else {
            &icon.day.clear
        }
    }
}

pub fn get_icon_for_daily_forecast<'a>(size: IconSize, forecast: &'a DailyForecast) -> &'a Qoi<'a> {
    let icons = match size {
        IconSize::Small => &ICONS.small,
        IconSize::Large => &ICONS.large,
    };
    let is_cloudy = forecast.clouds >= 60;
    let is_windy = forecast.wind_speed >= 32.2 || forecast.wind_gust.unwrap_or(0.0) >= 40.2;

    let condition = forecast.weather[0]
        .get_condition()
        .unwrap_or(WeatherConditionId::ClearSky);

    get_icon_for_condition(icons, condition, is_cloudy, is_windy, false)
}

pub fn get_icon_for_current_weather<'a>(
    size: IconSize,
    current: &'a CurrentWeather,
) -> &'a Qoi<'a> {
    let icons = match size {
        IconSize::Small => &ICONS.small,
        IconSize::Large => &ICONS.large,
    };

    let is_cloudy = current.clouds >= 60;
    let is_windy = current.wind_speed >= 32.2 || current.wind_gust.unwrap_or(0.0) >= 40.2;
    let is_night = current.dt >= current.sunset || current.dt <= current.sunrise;

    let condition = current.weather[0]
        .get_condition()
        .unwrap_or(WeatherConditionId::ClearSky);

    // info!("id: {}, is_night: {}, is_windy: {}, is_cloudy: {}", current.weather[0].id, is_night, is_windy, is_cloudy);

    get_icon_for_condition(icons, condition, is_cloudy, is_windy, is_night)
}

fn get_icon_for_condition<'a>(
    icons: &'a WeatherIconSet,
    condition: WeatherConditionId,
    is_cloudy: bool,
    is_windy: bool,
    is_night: bool,
) -> &'a Qoi<'a> {
    match condition {
        WeatherConditionId::ThunderstormWithLightRain => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ThunderstormWithRain => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ThunderstormWithHeavyRain => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::LightThunderstorm => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::Thunderstorm => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::HeavyThunderstorm => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::RaggedThunderstorm => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ThunderstormWithLightDrizzle => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ThunderstormWithDrizzle => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ThunderstormWithHeavyDrizzle => {
            get_icon(&icons.thunderstorm, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::LightIntensityDrizzle => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::Drizzle => get_icon(&icons.drizzle, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavyIntensityDrizzle => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::LightIntensityDrizzleRain => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::DrizzleRain => get_icon(&icons.drizzle, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavyIntensityDrizzleRain => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ShowerRainAndDrizzle => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::HeavyShowerRainAndDrizzle => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ShowerDrizzle => {
            get_icon(&icons.drizzle, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::LightRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::ModerateRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavyIntensityRain => {
            get_icon(&icons.rain, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::VeryHeavyRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::ExtremeRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::FreezingRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::LightIntensityShowerRain => {
            get_icon(&icons.rain, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ShowerRain => get_icon(&icons.rain, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavyIntensityShowerRain => {
            get_icon(&icons.rain, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::RaggedShowerRain => {
            get_icon(&icons.rain, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::LightSnow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::Snow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavySnow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::Sleet => get_icon(&icons.rain_mix, is_night, is_cloudy, is_windy),
        WeatherConditionId::LightShowerSleet => {
            get_icon(&icons.rain_mix, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::ShowerSleet => get_icon(&icons.rain_mix, is_night, is_cloudy, is_windy),
        WeatherConditionId::LightRainAndSnow => {
            get_icon(&icons.rain_mix, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::RainAndSnow => get_icon(&icons.rain_mix, is_night, is_cloudy, is_windy),
        WeatherConditionId::LightShowerSnow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::ShowerSnow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::HeavyShowerSnow => get_icon(&icons.snow, is_night, is_cloudy, is_windy),
        WeatherConditionId::Mist => get_icon(&icons.fog, is_night, is_cloudy, is_windy),
        WeatherConditionId::Smoke => get_icon(&icons.smoke, is_night, is_cloudy, is_windy),
        WeatherConditionId::Haze => get_icon(&icons.fog, is_night, is_cloudy, is_windy),
        WeatherConditionId::SandOrDustWhirls => {
            get_icon(&icons.sand, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::Fog => get_icon(&icons.fog, is_night, is_cloudy, is_windy),
        WeatherConditionId::Sand => get_icon(&icons.sand, is_night, is_cloudy, is_windy),
        WeatherConditionId::Dust => get_icon(&icons.dust, is_night, is_cloudy, is_windy),
        WeatherConditionId::VolcanicAsh => get_icon(&icons.volcanic, is_night, is_cloudy, is_windy),
        WeatherConditionId::Squalls => get_icon(&icons.squalls, is_night, is_cloudy, is_windy),
        WeatherConditionId::Tornado => get_icon(&icons.tornado, is_night, is_cloudy, is_windy),
        WeatherConditionId::ClearSky => get_icon(&icons.clear, is_night, is_cloudy, is_windy),
        WeatherConditionId::FewClouds => get_icon(&icons.few_clouds, is_night, is_cloudy, is_windy),
        WeatherConditionId::ScatteredClouds => {
            get_icon(&icons.scattered, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::BrokenClouds => {
            get_icon(&icons.scattered, is_night, is_cloudy, is_windy)
        }
        WeatherConditionId::OvercastClouds => {
            get_icon(&icons.scattered, is_night, is_cloudy, is_windy)
        }
    }
}
