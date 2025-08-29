use crate::logic::{Command, Event, Indicator};
use std::net::UdpSocket;

// Address at which the simulator receives commands
const SIM_ADDRESS: &str = "127.0.0.1:10000";

// Address where the simulator sends events (that's me)
const CONTROL_ADDRESS: &str = "127.0.0.1:11000";

// Structure that abstracts details of the network simulator
// out and provides a nicer high-level interface to what's happening.
// Note: I'm calling this "LiftyController" to indicate that it is specifically
// written for the simulator.   Maybe we want to run the elevator under a
// MockController or some other kind of mechanism.

pub struct LiftyController {
    socket: UdpSocket,
    stopping: bool,
}

impl LiftyController {
    pub fn new() -> LiftyController {
        let controller = LiftyController {
            socket: UdpSocket::bind(CONTROL_ADDRESS).expect("Couldn't bind to control address"),
            stopping: false,
        };

        controller._send("R");

        controller
    }

    pub fn send(&mut self, command: Command) {
        match command {
            Command::MoveUp => self._send("MU"),
            Command::MoveDown => self._send("MD"),

            Command::StopAndOpen { floor, direction } => {
                self.stopping = true;
                self._send("S");

                self._send(&format!("CP{floor}"));

                match direction {
                    Indicator::Up => {
                        self._send(&format!("IU{floor}"));
                        self._send(&format!("CU{floor}"));
                    }
                    Indicator::Down => {
                        self._send(&format!("ID{floor}"));
                        self._send(&format!("CD{floor}"));
                    }
                    Indicator::Off => {}
                }
            }

            Command::OpenDoor { floor, direction } => {
                self._send("DO");
                self._send(&format!("CP{floor}"));

                match direction {
                    Indicator::Up => {
                        self._send(&format!("IU{floor}"));
                        self._send(&format!("CU{floor}"));
                    }
                    Indicator::Down => {
                        self._send(&format!("ID{floor}"));
                        self._send(&format!("CD{floor}"));
                    }
                    Indicator::Off => {}
                }
            }

            Command::ChangeIndicator { floor, direction } => match direction {
                Indicator::Up => {
                    self._send(&format!("IU{floor}"));
                    self._send(&format!("CU{floor}"));
                }
                Indicator::Down => {
                    self._send(&format!("ID{floor}"));
                    self._send(&format!("CD{floor}"));
                }
                Indicator::Off => {}
            },

            Command::RejectEvent(event) => match event {
                Event::Panel(floor) => {
                    self._send(&format!("CP{floor}"));
                }
                Event::Up(floor) => {
                    self._send(&format!("CU{floor}"));
                }
                Event::Down(floor) => {
                    self._send(&format!("CD{floor}"));
                }
                _ => (),
            },
        }
    }

    pub fn receive(&mut self) -> Event {
        let event = self._receive();
        let floor = event[event.len() - 1..event.len()].parse().unwrap();

        match &event[0..event.len() - 1] {
            "P" => Event::Panel(floor),
            "U" => Event::Up(floor),
            "D" => Event::Down(floor),
            "C" => {
                self._send(&format!("CI{floor}"));
                Event::Closed(floor)
            }
            "A" => {
                if self.stopping {
                    self.stopping = false;
                }
                Event::Arrived(floor)
            }
            "S" => {
                if self.stopping {
                    self.stopping = false;
                    self._send("DO");
                }
                self.receive()
            }
            "O" => self.receive(),
            _ => panic!("Unknown event"),
        }
    }

    // Low-level send/receive of raw Lifty commands (private)
    fn _send(&self, command: &str) {
        self.socket
            .send_to(command.as_bytes(), SIM_ADDRESS)
            .unwrap();
    }

    fn _receive(&mut self) -> String {
        let mut buffer = [0; 2000];
        let (n, _) = self.socket.recv_from(&mut buffer).unwrap();
        String::from_utf8(buffer[0..n].to_vec()).unwrap()
    }
}
