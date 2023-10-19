use anyhow::Result;
use tinyqoi::Qoi;

pub struct IconSet {
    pub windy: Qoi<'static>,
    pub clear: Qoi<'static>,
}

pub struct WeatherIcon {
    pub day: IconSet,
    pub night: IconSet,
    pub cloudy: IconSet,
}

pub struct WeatherIconSet {
    pub WIDTH: u32,
    pub HEIGHT: u32,
    pub clear: WeatherIcon,
    pub few_clouds: WeatherIcon,
    pub scattered: WeatherIcon,
    pub overcast: WeatherIcon,
    pub snow: WeatherIcon,
    pub rain: WeatherIcon,
    pub rain_mix: WeatherIcon,
    pub thunderstorm: WeatherIcon,
    pub drizzle: WeatherIcon,
    pub fog: WeatherIcon,
    pub smoke: WeatherIcon,
    pub dust: WeatherIcon,
    pub sand: WeatherIcon,
    pub volcanic: WeatherIcon,
    pub squalls: WeatherIcon,
    pub tornado: WeatherIcon,
}

macro_rules! icon {
    ($size:expr, $name:expr) => {
        Qoi::new(include_bytes!(concat!("qoi/", $size, "/", $name, ".qoi"))).unwrap()
    };
}

macro_rules! large_icon {
    ($name:expr) => {
        icon!("196x196", $name)
    };
}

macro_rules! small_icon {
    ($name:expr) => {
        icon!("64x64", $name)
    };
}

impl WeatherIconSet {
    pub fn new() -> Result<Self> {
        let clear = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-sunny"),
                windy: large_icon!("wi-day-windy"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-clear"),
                windy: large_icon!("wi-night-clear"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-cloud"),
                windy: large_icon!("wi-cloudy-windy"),
            },
        };
        let few_clouds = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-cloudy"),
                windy: large_icon!("wi-day-cloudy-windy"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-cloudy"),
                windy: large_icon!("wi-night-alt-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-cloud"),
                windy: large_icon!("wi-cloudy-windy"),
            },
        };
        let scattered = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-cloud"),
                windy: large_icon!("wi-cloudy-windy"),
            },
            night: IconSet {
                clear: large_icon!("wi-cloud"),
                windy: large_icon!("wi-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-cloud"),
                windy: large_icon!("wi-cloudy-windy"),
            },
        };
        let overcast = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-cloudy"),
                windy: large_icon!("wi-cloudy-windy"),
            },
            night: IconSet {
                clear: large_icon!("wi-cloudy"),
                windy: large_icon!("wi-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-cloudy"),
                windy: large_icon!("wi-cloudy-windy"),
            },
        };
        let snow = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-snow"),
                windy: large_icon!("wi-day-snow-wind"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-snow"),
                windy: large_icon!("wi-night-alt-snow-wind"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-snow"),
                windy: large_icon!("wi-snow-wind"),
            },
        };
        let rain = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-rain"),
                windy: large_icon!("wi-day-rain-wind"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-rain"),
                windy: large_icon!("wi-night-alt-rain-wind"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-rain"),
                windy: large_icon!("wi-rain-wind"),
            },
        };
        let rain_mix = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-rain-mix"),
                windy: large_icon!("wi-day-rain-mix"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-rain-mix"),
                windy: large_icon!("wi-night-alt-rain-mix"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-rain-mix"),
                windy: large_icon!("wi-rain-mix"),
            },
        };
        let drizzle = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-showers"),
                windy: large_icon!("wi-day-showers"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-showers"),
                windy: large_icon!("wi-night-alt-showers"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-showers"),
                windy: large_icon!("wi-showers"),
            },
        };
        let thunderstorm = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-thunderstorm"),
                windy: large_icon!("wi-day-thunderstorm"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-alt-thunderstorm"),
                windy: large_icon!("wi-night-alt-thunderstorm"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-thunderstorm"),
                windy: large_icon!("wi-thunderstorm"),
            },
        };
        let fog = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-day-fog"),
                windy: large_icon!("wi-day-fog"),
            },
            night: IconSet {
                clear: large_icon!("wi-night-fog"),
                windy: large_icon!("wi-night-fog"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-fog"),
                windy: large_icon!("wi-fog"),
            },
        };
        let smoke = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-smoke"),
                windy: large_icon!("wi-smoke"),
            },
            night: IconSet {
                clear: large_icon!("wi-smoke"),
                windy: large_icon!("wi-smoke"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-smoke"),
                windy: large_icon!("wi-smoke"),
            },
        };
        let dust = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-dust"),
                windy: large_icon!("wi-dust"),
            },
            night: IconSet {
                clear: large_icon!("wi-dust"),
                windy: large_icon!("wi-dust"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-dust"),
                windy: large_icon!("wi-dust"),
            },
        };
        let sand = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-sandstorm"),
                windy: large_icon!("wi-sandstorm"),
            },
            night: IconSet {
                clear: large_icon!("wi-sandstorm"),
                windy: large_icon!("wi-sandstorm"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-sandstorm"),
                windy: large_icon!("wi-sandstorm"),
            },
        };
        let squalls = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-cloudy-gusts"),
                windy: large_icon!("wi-cloudy-gusts"),
            },
            night: IconSet {
                clear: large_icon!("wi-cloudy-gusts"),
                windy: large_icon!("wi-cloudy-gusts"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-cloudy-gusts"),
                windy: large_icon!("wi-cloudy-gusts"),
            },
        };
        let tornado = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-tornado"),
                windy: large_icon!("wi-tornado"),
            },
            night: IconSet {
                clear: large_icon!("wi-tornado"),
                windy: large_icon!("wi-tornado"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-tornado"),
                windy: large_icon!("wi-tornado"),
            },
        };
        let volcanic = WeatherIcon {
            day: IconSet {
                clear: large_icon!("wi-volcano"),
                windy: large_icon!("wi-volcano"),
            },
            night: IconSet {
                clear: large_icon!("wi-volcano"),
                windy: large_icon!("wi-volcano"),
            },
            cloudy: IconSet {
                clear: large_icon!("wi-volcano"),
                windy: large_icon!("wi-volcano"),
            },
        };

        Ok(Self {
            HEIGHT: 196,
            WIDTH: 196,
            clear,
            few_clouds,
            scattered,
            overcast,
            snow,
            rain,
            rain_mix,
            thunderstorm,
            drizzle,
            fog,
            smoke,
            dust,
            sand,
            volcanic,
            squalls,
            tornado,
        })
    }

    pub fn new_small() -> Result<Self> {
        let clear = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-sunny"),
                windy: small_icon!("wi-day-windy"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-clear"),
                windy: small_icon!("wi-night-clear"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-cloud"),
                windy: small_icon!("wi-cloudy-windy"),
            },
        };
        let few_clouds = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-cloudy"),
                windy: small_icon!("wi-day-cloudy-windy"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-cloudy"),
                windy: small_icon!("wi-night-alt-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-cloud"),
                windy: small_icon!("wi-cloudy-windy"),
            },
        };
        let scattered = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-cloud"),
                windy: small_icon!("wi-cloudy-windy"),
            },
            night: IconSet {
                clear: small_icon!("wi-cloud"),
                windy: small_icon!("wi-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-cloud"),
                windy: small_icon!("wi-cloudy-windy"),
            },
        };
        let overcast = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-cloudy"),
                windy: small_icon!("wi-cloudy-windy"),
            },
            night: IconSet {
                clear: small_icon!("wi-cloudy"),
                windy: small_icon!("wi-cloudy-windy"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-cloudy"),
                windy: small_icon!("wi-cloudy-windy"),
            },
        };
        let snow = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-snow"),
                windy: small_icon!("wi-day-snow-wind"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-snow"),
                windy: small_icon!("wi-night-alt-snow-wind"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-snow"),
                windy: small_icon!("wi-snow-wind"),
            },
        };
        let rain = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-rain"),
                windy: small_icon!("wi-day-rain-wind"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-rain"),
                windy: small_icon!("wi-night-alt-rain-wind"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-rain"),
                windy: small_icon!("wi-rain-wind"),
            },
        };
        let rain_mix = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-rain-mix"),
                windy: small_icon!("wi-day-rain-mix"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-rain-mix"),
                windy: small_icon!("wi-night-alt-rain-mix"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-rain-mix"),
                windy: small_icon!("wi-rain-mix"),
            },
        };
        let drizzle = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-showers"),
                windy: small_icon!("wi-day-showers"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-showers"),
                windy: small_icon!("wi-night-alt-showers"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-showers"),
                windy: small_icon!("wi-showers"),
            },
        };
        let thunderstorm = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-thunderstorm"),
                windy: small_icon!("wi-day-thunderstorm"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-alt-thunderstorm"),
                windy: small_icon!("wi-night-alt-thunderstorm"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-thunderstorm"),
                windy: small_icon!("wi-thunderstorm"),
            },
        };
        let fog = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-day-fog"),
                windy: small_icon!("wi-day-fog"),
            },
            night: IconSet {
                clear: small_icon!("wi-night-fog"),
                windy: small_icon!("wi-night-fog"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-fog"),
                windy: small_icon!("wi-fog"),
            },
        };
        let smoke = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-smoke"),
                windy: small_icon!("wi-smoke"),
            },
            night: IconSet {
                clear: small_icon!("wi-smoke"),
                windy: small_icon!("wi-smoke"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-smoke"),
                windy: small_icon!("wi-smoke"),
            },
        };
        let dust = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-dust"),
                windy: small_icon!("wi-dust"),
            },
            night: IconSet {
                clear: small_icon!("wi-dust"),
                windy: small_icon!("wi-dust"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-dust"),
                windy: small_icon!("wi-dust"),
            },
        };
        let sand = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-sandstorm"),
                windy: small_icon!("wi-sandstorm"),
            },
            night: IconSet {
                clear: small_icon!("wi-sandstorm"),
                windy: small_icon!("wi-sandstorm"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-sandstorm"),
                windy: small_icon!("wi-sandstorm"),
            },
        };
        let squalls = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-cloudy-gusts"),
                windy: small_icon!("wi-cloudy-gusts"),
            },
            night: IconSet {
                clear: small_icon!("wi-cloudy-gusts"),
                windy: small_icon!("wi-cloudy-gusts"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-cloudy-gusts"),
                windy: small_icon!("wi-cloudy-gusts"),
            },
        };
        let tornado = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-tornado"),
                windy: small_icon!("wi-tornado"),
            },
            night: IconSet {
                clear: small_icon!("wi-tornado"),
                windy: small_icon!("wi-tornado"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-tornado"),
                windy: small_icon!("wi-tornado"),
            },
        };
        let volcanic = WeatherIcon {
            day: IconSet {
                clear: small_icon!("wi-volcano"),
                windy: small_icon!("wi-volcano"),
            },
            night: IconSet {
                clear: small_icon!("wi-volcano"),
                windy: small_icon!("wi-volcano"),
            },
            cloudy: IconSet {
                clear: small_icon!("wi-volcano"),
                windy: small_icon!("wi-volcano"),
            },
        };

        Ok(Self {
            HEIGHT: 64,
            WIDTH: 64,
            clear,
            few_clouds,
            scattered,
            overcast,
            snow,
            rain,
            rain_mix,
            thunderstorm,
            drizzle,
            fog,
            smoke,
            dust,
            sand,
            volcanic,
            squalls,
            tornado,
        })
    }
}
