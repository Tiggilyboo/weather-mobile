# Weather Mobile
A simple GTK weather app to play with Linux Mobile development in GTK4 and OpenWeather API.

## Dependances
Requires the following dev packages (names vary by linux distribution package manager):

- gtk4
- curl
- cairo
- pango
- openssl
- graphene

## Install
Requires [rust install](https://www.rust-lang.org/tools/install)
```bash
cargo install --path .
```

## Build
As above, this is built in rust, so grab the latest stable toolkit, then just:
```bash
cargo run
```

## Features

### Alerts
Show any current alerts in the selected area

![Alerts](https://i.imgur.com/6lnRtlM.png)

### Current
A breif overview of the current weather in the selected area.

### Hourly
A 24 hour glance at the upcoming weather, includes status, temperature, precipitation and gusting conditions for each hour.

![Hourly](https://i.imgur.com/hZXgIiv.png)

### Weekly
A 8 day forecast view of the upcoming weather, includes status, temperature (by time of day), wind speed, direction and gusting conditions. 
Also includes sunset and sunrise times for each day of the week.

![Weekly](https://i.imgur.com/Kqdamvd.png)

### Preferences
- Save and restore the last viewed location to be seen the next time you open the application. 
- Save and restore the units of measure (Imperial / Metric)

![Search](https://i.imgur.com/qSk4vD6.png)
![Preferences](https://i.imgur.com/QqieI8A.png)
