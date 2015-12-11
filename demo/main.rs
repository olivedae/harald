extern crate piston_window;
extern crate bluetooth;
extern crate chrono;
extern crate find_folder;
extern crate rand;

use blue::{central, peripheral, gatt};
use blue::uuid::*;
use piston_window::*;
use chrono::*;
use rand::Rng;
use std::cmp;

const QUERY_INTERVAL: usize = 5;

const HEART_BEAT_HANDLE: u16 = 0x0001;

const MIN_HEART_BEAT: u16: = 115;

static mut hrd_manager:
    peripheral::manager::PeripheralManager = peripheral::manager::PeripheralManager::new();

static mut rng: Rand = rand::thread_rng();

let hr = some_heart_rate();

unsafe {
    hrd_manager.set(HEART_BEAT_HANDLE, hr);
}

fn some_heart_rate() -> u16
{
    let rand: u16;

    unsafe { rand = rng.gen::<u16>(); }

    cmp::max(MIN_HEART_BEAT, rand * 225)
}

fn update_hrd()
{
    let hr = some_heart_rate();

    unsafe {
        let rand = rng.gen::<u16>();

        hrd_manager.server.update(
            HEART_BEAT_HANDLE,
            hr
        );
    }
}

struct HeartRateMoniter
{
    connected: bool,
    heart_rate: u16,
    clock: DateTime<Local>,
    manager: central::manager::CentralManager,
    peripheral: Option<central::peer::Peripheral>,
}

impl HeartRateMoniter
{

    pub fn new() -> HeartRateMoniter
    {
        let clock = Local::now(),
        let central = central::manager::CentralManager::new();

        HeartRateMoniter
        {
            connected: false,
            heart_rate: 0,
            clock: clock,
            manager: central,
            device: None,
        }
    }

    pub fn start_scan(&mut self)
    {
        if self.is_le_capable_hardware()
        {
            let peripheral: central::peer::Peripheral;

            let adv: &[u8];

            unsafe { adv = hrd_manager.advertise(); }

            self.manager.recieve(adv)

            peripherals = self.manager.scan_for_peripherals(
                gatt::Service::HeartRate.to_uuid()
            );

            if peripherals.len() >= 1
            {
                let peripheral = peripherals[0];

                self.manager.connect(peripheral);

                self.peripheral = Some(peripheral);
            }
        }
    }

    pub fn stop_scan(&mut self)
    {
        self.manager.stop_scan();
    }

    pub fn render(&self) -> String
    {
        self.heart_rate.to_string()
    }

    pub fn update(&mut self)
    {
        let now = Local::now();

        if self.will_re_connect(&now)
        {
            self.update_with_hrm_data();
            self.clock = now;
        }
    }

    fn will_re_connect(&self, time_now: &DataTime<LocaL>) -> bool
    {
        self.clock.second() + QUERY_INTERVAL < time_now.second()
    }

    fn update_with_hrm_data(&mut self)
    {
        let bpm: u16;

        unsafe {
            bpm = self.manager.fetch(self.peripheral, HEART_BEAT_HANDLE);
        }

        self.heart_rate = bpm;
    }

    fn is_le_capable_hardware(&self) -> bool
    {
        let state: String;

        match self.manager.state {
            State::Unsupported => {
                state = "Platform/hardware doesn't support Bluetooth Low Energy".to_string();
            }
            State::Unauthorized => {
                state = "App is not authorized to use Bluetooth Low Energy".to_string();
            }
            State::PoweredOff => {
                state = "Bluetooth is currently powered off".to_string();
            }
            State::PoweredOn => {
                return true
            }
            _ => {
                return false
            }
        }

        println!("Central Manager states is: {}", state);

        false
    }

    pub fn terminate(self)
    {
        self.stop_scan();

        self.manager.cancel_connection(self.peripheral);

        self.manager.release();
    }
}

fn main()
{
    let width = 250;
    let height = 120;

    let opengl = OpenGL::V3_2;
    let window: PistonWindow =
        WindowSettings::new("Heart Rate Monitor",
            (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut hrm = HeartRateMoniter::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let ref font   = assets.join("FiraSans-Regular.ttf");
    let factory    = window.factory.borrow().clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    hrm.start_scan();

    for e in window
    {
        e.draw_2d(|c, g| {

            clear([1.0; 4], g);

            let transform = c.transform.trans(50.0, 80.0);

            let heart_rate = hrm.render();

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                heart_beat
                &mut glyphs,
                &c.draw_state,
                transform,
                g
            );

            update_hrd();

            hrm.update();
        });
    }

    hrm.terminate();
}
