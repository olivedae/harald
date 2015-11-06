/*
 * Demo of a simple Heart Rate Monitor application
 * written in Rust and using the Bluetooth crate. The
 * application simply establishes a connection with a peripheral
 * device which supports the Heart Rate Service and periodically
 * reconnects to update the text display of a user's
 * heart rate.
 *
 * One fact to point out is that this file is
 * currently unable to run due to Bluetooth
 * library being incomplete. It however does
 * serve as a projected view of interaction
 * with this library.
*/

extern crate piston_window;
extern crate bluetooth;
extern crate chrono;
extern crate find_folder;
extern crate rand;

/*
 * To begin, we load in required libraries and bring
 * into scope various features such as Bluetooth's
 * abstracted structures of CentralManager, PeripheralManager,
 * and a library to identify the services and characteristics
 * supported by a Device (commonly devices which perform as a server, thus
 * a peripheral).
*/

use bluetooth::{central, peripheral, gatt};
use bluetooth::uuid::*;
use piston_window::*;
use chrono::*;
use rand::Rng;
use std::cmp;

const QUERY_INTERVAL: usize = 5;

const HEART_BEAT_HANDLE: u16 = 0x0001;

const MIN_HEART_BEAT: u16: = 115;

/*
 * Currently, Bluetooth does not support
 * interactions with an OS's Bluetooth LE controller.
 *
 * The PeripheralManager is initialized to mimic
 * the manager in charge of the, to be found, heart
 * rate device. It is decalred as static to
 * allow the application access to it without
 * having to pass in is as arguments, thus
 * hiding this fact in a beneficial manner.
*/

static mut hrd_manager:
    peripheral::manager::PeripheralManager = peripheral::manager::PeripheralManager::new();

static mut rng: Rand = rand::thread_rng();

let hr = some_heart_rate();

unsafe {
    hrd_manager.set(HEART_BEAT_HANDLE, hr);
}

/*
 * Several functions are provided to
 * periodically update the peripheral device
 * with new heart rate data, thus allowing
 * an example as how to one can manage the external
 * state of a value.
*/

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

/*
 * The `HeartRateMoniter` structure is created
 * represent a heart rate monitor app which runs
 * over Bluetooth Low Energy
*/

struct HeartRateMoniter
{
    /*
     * Several access variables to be used
     * internally such as clock (controls when
     * the application reconnects) and variables
     * to be used externally such as `heart_rate`
     * and `connected`.
    */
    connected: bool,
    heart_rate: u16,
    clock: DateTime<Local>,
    /*
     * Initializes a generic client to be
     * used by the application.
    */
    manager: central::manager::CentralManager,
    /*
     * Stores an abstracted data structure
     * used to represent an external peripheral
     * found by the client manager. At the heart,
     * devices can be found by located by
     * their address UUID. A `PeripheralPeer`
     * abstracts this in addition to building a
     * server that stores previously located attributes and
     * supported profiles and characteristics.
     *
     * Ideally it will also allow for an application
     * to recieve/queue received
     * notifications and indications for this
     * peer.
    */
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

    /*
     * Indicates to the application to
     * begin scanning and locate a device which
     * supports the Heart Rate service, defined by
     * the Bluetooth SIG.
    */
    pub fn start_scan(&mut self)
    {
        /*
         * Queries the controller of the machine
         * running the application whether it supports
         * Bluetooth Low Energy.
        */
        if self.is_le_capable_hardware()
        {
            let peripheral: central::peer::Peripheral;

            let adv: &[u8];

            /*
             * Since queries to machine's controller are
             * currently unsupported we generate an advertisement
             * packet directly by the peer manager following the
             * Generic Access Profile (GAP).
             *
             * Our slave device takes the peripheral and
             * broadcaster role define by GAP and thus
             * emits advertisements and responds to requests
             * (such a bonding, active scanning) made by a
             * central device.
             *
             * Emitted advertisement packets contain
             * flags to inform a device scanning about
             * the state of the advertisement in addition to
             * the state of the broadcaster. A short list of
             * defined modes include broadcast, (non, limited)discoverable,
             * and (non)bondable.
             *
             * These can then be translated to various procedures
             * for both central and peripheral devices to follow.
             *
             * Discovery:
             * Refers to how devices are to be displayed
             * by a user interface. Non-disoverable is self explanatory,
             * however, confusion comes between limited and general. The
             * limited discovery flag is sent for devices who have
             * just turned on and want to be connected to and
             * sends out advertisements a reasonable interval. This
             * allows for a device to be fairly certain this
             * is the device it is to be connected with. Limited
             * discovery lasts for short period of time, somewhere between
             * 250 - 500 milliseconds.
             *
             * General discovery mode is entered after limited and is
             * similar to limited except it can be in this mode for
             * an unlimited amount of time, in addition has a
             * longer advertising interval.
             *
             * Allows for devices to make intelligent decisions on
             * the order of available devices to connect with.
             *
             * Bonding:
             * Typically devices bond when they wish
             * to transmit encrypted data to one another
             * wish to preform additional
             * security procedures.
            */
            unsafe { adv = hrd_manager.advertise(); }

            /*
             * Again, Bluetooth's transport
             * layer and layers below are avoided by directly passing
             * data to devices. Ideally this will represent
             * the interface that a manager structure receives information
             * from its controller.
             *
             * When a `CentralManager` receives an advertisement
             * it parses the PDU and moves relevant data and state
             * to a developer-friendly `PeripheralPeer` structure
             * and stores it for later use.
            */
            self.manager.recieve(adv)

            /*
             * The manager is instructed to
             * scan for peripherals specifying
             * an option for a supported service
             * (since rarely is every Bluetooth
             * device nearby wanted). This is
             * supported by GATT structures
             * which allows discovery of
             * attributes, supported services,
             * and various characteristics in an organized
             * manner. This becomes handy for parsing and
             * interaction between two devices that
             * involves each others' state.
             *
             * It also exposes developer-friendly names
             * for various services, characteristics, and
             * descriptions defined by the Bluetooth SIG
             * as opposed to raw 16 bit or 128 bit values.
             *
             * In this scenario peripherals are searched for
             * that support the HeartRate service and pass that
             * as an argument to the function. A
             * vector of found peripherals wrapped in
             * the Option<> structure is returned since there may be
             * no peers located.
             *
             * Returned peripherals contain
             * only information provided by
             * the information contained in a devices
             * advertisement data. This varies and depends
             * on what the device wants to send.
             * This can include attributes such as
             * a user-friendly name for
             * the device and the transmit power
             * and more abstract information
             * such as several of the
             * significant services the device supports
             * and wants to make known of.
             *
             * Returned peripherals are in the order
             * of relative closeness to a scanner
             * where the first element is the closest
             * and last is the furthest.
            */
            peripherals = self.manager.scan_for_peripherals(
                gatt::Service::HeartRate.to_uuid()
            );

            /*
             * Although the manager has
             * received one valid
             * advertisement a more usual
             * case is to check that it
             * has
            */
            match peripherals {
                Some(peripherals) => {
                    let peripheral = peripherals[0];
                    
                    /*
                     * Using the selected peripheral the
                     * manager is instructed
                     * to bond with it.
                     *
                     * @todo
                     * Discussion of the bonding process
                     */
                    self.manager.bond(peripheral);

                    /*
                     * The structure's
                     * fields are updated
                     * appropriately
                     */
                    self.connected = true;
                    self.peripheral = Some(peripheral);
                }.
                None => {}
            };
        }
    }

    pub fn stop_scan(&mut self)
    {
        self.manager.stop_scan();
    }

    /*
     * Since the application
     * is fairly simple (only an updating text
     * display), this value is returned
     * when requested.
    */
    pub fn render(&self) -> String
    {
        self.heart_rate.to_string()
    }

    /*
     * The implementation of
     * this application and graphic package
     * used involves an event loop
     * which continually iterates until
     * it escapes (whether on close or an unresolvable error
     * arises which causes it to panic).
    */
    pub fn update(&mut self)
    {
        let now = Local::now();

        /*
         * To avoid continually requesting
         * requesting to
         * connect with the peripheral (will use
         * fairly significant amount of resources)
         * a connection gap of 5 seconds is added.
         *
         * @todo
         * do not do this; this is a bad design
         * fix this
        */
        if self.will_re_connect(&now)
        {
            /*
             * Updates the time since the last
             * reconnection in addition to updating
             * itself to the current state of
             * the peer device.
            */
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

    /*
     * Example of a developer may interact with the
     * library by first determining whether
     * the machine running the application
     * is capable of Bluetooth Low Energy
     * communications.
     *
     * Currently, will always return true; however,
     * calls go down to the controller and is requested
     * from the device's link manager.
     *
     * If something isn't correct a message is
     * printed saying what's going on.
     * From here, developers may prompt the user to
     * turn Bluetooth on or inform them that
     * they are unable to use it and why and return false.
    */
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

    /*
     * Not necessarily to be used for when the application
     * is to be turned closed since
     * Rust will allow for deallocation
     * of structures once a variable goes out of scope.
     *
     * @todo
     * figure out what I meant above
     *
     * Allows for a type of reset of the application.
    */
    pub fn terminate(self)
    {
        self.stop_scan();

        self.manager.cancel_bond(self.peripheral);
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
    hrm.stop_scan();

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
