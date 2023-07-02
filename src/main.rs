#![deny(clippy::all)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::print_stdout
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::future_not_send,
    clippy::implicit_return,
    clippy::similar_names,
    clippy::blanket_clippy_restriction_lints,
    clippy::module_name_repetitions
)]

#[macro_use]
mod macros;
mod common;
mod constants;
mod helpers;

use crate::constants::default_values::DefaultValues;
use crate::constants::pins_values::PinValues;
use crate::constants::strings::Strings;
use crate::helpers::logs::fern_log::setup_logging;
use crate::helpers::parsers::setting_files::SettingFiles;
use log::{debug, error, warn};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::process::Command;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{process, thread};
use sysinfo::{ProcessExt, Signal, System, SystemExt};

fn main() -> anyhow::Result<()> {
    let settings = SettingFiles::new();
    let settings_arc = Arc::new(settings);

    setup_logging(settings_arc.config.settings.enable_logs_to_file)?;

    log::debug!("Launching {}...", Strings::APP_NAME);

    run(settings_arc)?;

    Ok(())
}

fn is_process_running(p_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    for _ in sys.processes_by_name(p_name) {
        return true;
    }

    false
}

fn print_all_running_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        println!("PID: {}", pid);
        println!("Name: {}", process.name());
        // Add more fields as needed, such as CPU usage, memory usage, etc.
        println!("---------------------------------------");
    }
}

fn kill_process(p_name: &str) {
    let mut sys = System::new_all();
    sys.refresh_all();

    for p in sys.processes_by_name(p_name) {
        p.kill_with(Signal::Kill);
    }
}

fn wait_for_pull_up_button_press(pin: &InputPin) {
    let gpio_pin = pin;

    loop {
        let is_low = gpio_pin.is_low();

        if is_low {
            break;
        }

        sleep(Duration::from_millis(50));
    }
}

fn wait_for_pull_up_button_release(pin: &InputPin) {
    let gpio_pin = pin;

    loop {
        let is_high = gpio_pin.is_high();

        if is_high {
            break;
        }

        sleep(Duration::from_millis(50));
    }
}

fn blink_led(pin: &mut OutputPin) {
    let gpio_pin = pin;

    for _ in 0..20 {
        let is_high = gpio_pin.is_set_high();

        if is_high {
            gpio_pin.set_low();
        } else {
            gpio_pin.set_high();
        }

        sleep(Duration::from_millis(100));
    }

    gpio_pin.set_high();
}

fn run(settings: Arc<SettingFiles>) -> anyhow::Result<()> {
    let mut program_power_led = Gpio::new()?
        .get(PinValues::PROGRAM_POWER_LED)?
        .into_output_high();

    let radio_restart_pin = Gpio::new()?
        .get(PinValues::RADIO_RESTART_BTN)?
        .into_input_pullup();

    let shutdown_btn = Gpio::new()?
        .get(PinValues::SHUTDOWN_BTN)?
        .into_input_pullup();

    let mut os_loaded_led = Gpio::new()?
        .get(PinValues::OS_LOADED_LED)?
        .into_output_high();

    program_power_led.set_high();

    thread::spawn(move || -> anyhow::Result<()> {
        debug!("[radio] initializing...");
        debug!("[radio] sleeping the thread...");
        thread::sleep(::std::time::Duration::from_millis(2000));
        let settings_cloned = settings.clone();

        loop {
            if !is_process_running(Strings::CHROMIUM_PROCESS_NAME) {
                debug!("[radio] no instance of chromium found");

                debug!("[radio] creating new instance of chromium...");
                debug!("[radio] playing audio using the chromium instance...");

                let radio_url = &settings_cloned.config.settings.radio_url;
                let base_url = &settings_cloned.config.settings.radio_streaming_website_url;

                let encoded_query_string = url::form_urlencoded::Serializer::new(String::new())
                    .append_pair("radio_url", radio_url)
                    .finish();

                let url_with_encoded_query = format!("{}?{}", base_url, encoded_query_string);

                // command:
                // GOOGLE_API_KEY="xxxxxxxxxxxx" && GOOGLE_DEFAULT_CLIENT_ID="xxxxxxxxxxxx" && GOOGLE_DEFAULT_CLIENT_SECRET="xxxxxxxxxxxx" && chromium-browser --headless --disable-gpu --remote-debugging-port=9222 <https://website-name.com?radio_url=https://radio-url.com/playlist.m3u8> --new-window --start-maximized --incognito --disable-features=PreloadMediaEngagementData,AutoplayIgnoreWebAudio,MediaEngagementBypassAutoplayPolicies  --autoplay-policy=no-user-gesture-required
                let c = Command::new(Strings::CHROMIUM_EXECUTABLE)
                    .env("GOOGLE_API_KEY", &settings_cloned.config.settings.google_api_key)
                    .env("GOOGLE_DEFAULT_CLIENT_ID", &settings_cloned.config.settings.google_default_client_id)
                    .env("GOOGLE_DEFAULT_CLIENT_SECRET", &settings_cloned.config.settings.google_default_client_secret)
                    .arg("--headless")
                    .arg("--disable-gpu")
                    .arg("--remote-debugging-port=9222")
                    .arg(&url_with_encoded_query)
                    .arg("--new-window")
                    .arg("--start-maximized")
                    .arg("--incognito")
                    .arg("--disable-features=PreloadMediaEngagementData,AutoplayIgnoreWebAudio,MediaEngagementBypassAutoplayPolicies")
                    .arg("--autoplay-policy=no-user-gesture-required")
                    .spawn();

                match c {
                    Ok(_) => {}
                    Err(err) => {
                        error!("[radio] [0001][chromium] process error: {:?}", err);
                    }
                }
            }

            thread::sleep(::std::time::Duration::from_millis(5000));
        }
    });

    thread::spawn(move || -> anyhow::Result<()> {
        // set the `os loaded led` to high
        os_loaded_led.set_high();

        loop {
            debug!("[shutdown button] watching for the button press...");
            wait_for_pull_up_button_press(&shutdown_btn);
            debug!("[shutdown button] button pressed");
            let shutdown_btn_pressed_time = chrono::Local::now();

            wait_for_pull_up_button_release(&shutdown_btn);
            let elapsed_shutdown_btn_pressed_time_ms =
                (chrono::Local::now() - shutdown_btn_pressed_time).num_milliseconds();
            debug!(
                "[shutdown button] button released after `{}` milliseconds",
                elapsed_shutdown_btn_pressed_time_ms
            );

            // blink the `os loaded led` to show that the shutdown or reboot is in progress
            blink_led(&mut os_loaded_led);

            if elapsed_shutdown_btn_pressed_time_ms >= DefaultValues::RESTART_BTN_PRESS_TIME_MS {
                debug!(
                    "[shutdown button] restarting the OS since the button was pressed for \
    more than `{}` milliseconds",
                    DefaultValues::RESTART_BTN_PRESS_TIME_MS
                );

                // command: sudo reboot
                let res = Command::new("sudo").arg("reboot").output();

                if let Err(err) = res {
                    error!("[shutdown button] [0002] process error: {:?}", err);
                }
            } else {
                debug!("[shutdown button] shutting down the OS");

                // command: sudo shutdown now
                let res = Command::new("sudo").arg("shutdown").arg("now").output();
                if let Err(err) = res {
                    error!("[shutdown button] [0002] process error: {:?}", err);
                }
            }

            thread::sleep(::std::time::Duration::from_millis(1));
        }
    });

    thread::spawn(move || -> anyhow::Result<()> {
        debug!("[radio shutdown button] watching for the button press...");

        wait_for_pull_up_button_press(&radio_restart_pin);
        debug!("[radio shutdown button] button pressed");
        let radio_shutdown_btn_pressed_time = chrono::Local::now();

        wait_for_pull_up_button_release(&radio_restart_pin);
        let elapsed_radio_shutdown_btn_pressed_time_ms =
            (chrono::Local::now() - radio_shutdown_btn_pressed_time).num_milliseconds();
        debug!(
            "[radio shutdown button] button released after `{}` milliseconds",
            elapsed_radio_shutdown_btn_pressed_time_ms
        );

        debug!("[radio shutdown button] shutting down the program",);

        kill_process(Strings::CHROMIUM_PROCESS_NAME);

        // kill the power led
        program_power_led.set_low();

        process::exit(1);
    });

    loop {
        thread::sleep(::std::time::Duration::from_secs(10));
    }
}
