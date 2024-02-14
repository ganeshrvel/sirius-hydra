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
use std::process::{Command, Output};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{process, thread};
use sysinfo::{PidExt, ProcessExt, Signal, System, SystemExt};

fn main() -> anyhow::Result<()> {
    let settings = SettingFiles::new();
    let settings_arc = Arc::new(settings);

    setup_logging(settings_arc.config.settings.enable_logs_to_file)?;

    log::debug!("Launching {}...", Strings::APP_NAME);

    run(settings_arc)?;

    Ok(())
}

#[ignore = "dead_code"]
fn is_process_running(p_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    for _ in sys.processes_by_name(p_name) {
        return true;
    }

    false
}

fn is_ffmpeg_running() -> anyhow::Result<bool> {
    let processes: Vec<&str> = vec![
        Strings::FFMPEG_PROCESS_SHELL_COMMAND,
        Strings::FFMPEG_PROCESS_NAME,
        Strings::FFPLAY_PROCESS_NAME,
    ];

    for p in processes {
        if let Ok(result) = is_processes_running_find_by_command(p) {
            if result {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn find_pids_by_command(search_pattern: &str) -> anyhow::Result<Vec<u32>> {
    let command_string = r#"ps axww -o pid,command | awk '{printf "%s;", $1; for(i=2;i<=NF;i++) printf "%s ", $i; print ""}'"#;

    let ps_output: Output = Command::new("sh").arg("-c").arg(command_string).output()?;

    // Check if the command was executed successfully
    if !ps_output.status.success() {
        let error = String::from_utf8_lossy(&ps_output.stderr).to_string();
        return Err(anyhow::Error::msg(format!(
            "[find_processes_by_command] Failed to run the ps command: {}",
            error
        )));
    }

    // Convert the output to a UTF-8 string
    let ps_output_str = std::str::from_utf8(&ps_output.stdout)?;

    // Iterate over lines, extract PID where the command matches the search pattern
    let mut pids = Vec::new();
    for line in ps_output_str.lines().skip(1) {
        // Skip header row
        if let Some((pid_str, command)) = line.split_once(';') {
            if command.contains(search_pattern) {
                match pid_str.parse::<u32>() {
                    Ok(pid) => pids.push(pid),
                    Err(_) => error!(
                        "[find_processes_by_command] Failed to parse PID: {}",
                        pid_str
                    ),
                }
            }
        }
    }

    Ok(pids)
}

fn is_processes_running_find_by_command(search_pattern: &str) -> anyhow::Result<bool> {
    let matching_processes = find_pids_by_command(search_pattern)?;

    return Ok(matching_processes.len() > 0);
}

fn kill_running_processes_by_command(search_pattern: &str) -> anyhow::Result<()> {
    let mut sys = System::new_all();
    let matching_processes = find_pids_by_command(search_pattern)?;

    for pid in matching_processes {
        let p = sys.process(sysinfo::Pid::from_u32(pid));
        match p {
            Some(pd) => {
                pd.kill_with(Signal::Kill);
            }
            _ => {}
        }
    }

    return Ok(());
}

fn kill_ffmpeg() -> anyhow::Result<()> {
    let processes: Vec<&str> = vec![
        Strings::FFMPEG_PROCESS_SHELL_COMMAND,
        Strings::FFMPEG_PROCESS_NAME,
        Strings::FFPLAY_PROCESS_NAME,
    ];

    for p in processes {
        kill_running_processes_by_command(p)?
    }

    return Ok(());
}

#[ignore = "dead_code"]
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

#[ignore = "dead_code"]
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
            if !is_ffmpeg_running()? {
                debug!("[radio] no instance of ffmpeg found");

                debug!("[radio] creating new instance of ffmpeg...");
                debug!("[radio] playing audio using the ffmpeg instance...");

                let radio_url = &settings_cloned.config.settings.radio_url;
                let command = format!(
                    "{} -reconnect 1 -reconnect_streamed 1 -reconnect_delay_max 5 -i {} -acodec copy -f wav - | {} -",
                    Strings::FFMPEG_EXECUTABLE, radio_url, Strings::FFPLAY_EXECUTABLE
                );

                // final command:
                // sh -c 'ffmpeg -reconnect 1 -reconnect_streamed 1 -reconnect_delay_max 5 -i https://example.com/playlist.m3u8 -acodec copy -f wav - | ffplay -'
                let c = Command::new("sh").arg("-c").arg(command).spawn();

                match c {
                    Ok(_) => {}
                    Err(err) => {
                        error!("[radio] [0001][ffmpeg] process error: {:?}", err);
                    }
                }
            }

            thread::sleep(::std::time::Duration::from_millis(20000));
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

        kill_ffmpeg()?;

        // kill the power led
        program_power_led.set_low();

        process::exit(1);
    });

    loop {
        thread::sleep(::std::time::Duration::from_secs(10));
    }
}
