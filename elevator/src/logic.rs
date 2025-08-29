pub const LOWEST_FLOOR: u8 = 1;
pub const HIGHEST_FLOOR: u8 = 5;

const NUM_FLOORS: u8 = HIGHEST_FLOOR - LOWEST_FLOOR + 1;

#[derive(Debug, PartialEq)]
pub enum Command {
    MoveUp,
    MoveDown,
    StopAndOpen { floor: u8, direction: Indicator },
    OpenDoor { floor: u8, direction: Indicator },
    ChangeIndicator { floor: u8, direction: Indicator },
    RejectEvent(Event),
}

#[derive(Debug, PartialEq)]
pub enum Indicator {
    Up,
    Down,
    Off,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Panel(u8),
    Up(u8),
    Down(u8),
    Arrived(u8),
    Closed(u8),
}

#[derive(Debug, Clone)]
pub enum ElevatorState {
    Idle,
    MovingUp,
    MovingDown,
    Open,
    OpenUp,
    OpenDown,
}

#[derive(Debug)]
pub struct Elevator {
    state: ElevatorState,
    pub floor: u8,
    destinations: [bool; NUM_FLOORS as usize],
    up_requests: [bool; NUM_FLOORS as usize],
    down_requests: [bool; NUM_FLOORS as usize],
}

impl Elevator {
    pub fn new() -> Self {
        let elevator = Elevator {
            state: ElevatorState::Idle,
            floor: LOWEST_FLOOR,
            destinations: [false; NUM_FLOORS as usize],
            up_requests: [false; NUM_FLOORS as usize],
            down_requests: [false; NUM_FLOORS as usize],
        };
        elevator.check_invariants();
        elevator
    }

    pub fn handle(&mut self, event: Event) -> Option<Command> {
        let (command, new_state) = match &self.state {
            ElevatorState::Idle => self.handle_idle(event),
            ElevatorState::MovingUp => self.handle_moving_up(event),
            ElevatorState::MovingDown => self.handle_moving_down(event),
            ElevatorState::Open => self.handle_open(event),
            ElevatorState::OpenUp => self.handle_open_up(event),
            ElevatorState::OpenDown => self.handle_open_down(event),
        };

        self.state = new_state;
        self.check_invariants();
        command
    }

    fn handle_idle(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        match event {
            Event::Panel(floor) => {
                if floor != self.floor {
                    self.set_destination(floor as usize, true);
                    if floor > self.floor {
                        (Some(Command::MoveUp), ElevatorState::MovingUp)
                    } else {
                        (Some(Command::MoveDown), ElevatorState::MovingDown)
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Off,
                        }),
                        ElevatorState::Open,
                    )
                }
            }
            Event::Up(floor) => {
                if floor != self.floor {
                    self.set_up_request(floor as usize, true);
                    if floor > self.floor {
                        (Some(Command::MoveUp), ElevatorState::MovingUp)
                    } else {
                        (Some(Command::MoveDown), ElevatorState::MovingDown)
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorState::OpenUp,
                    )
                }
            }
            Event::Down(floor) => {
                if floor != self.floor {
                    self.set_down_request(floor as usize, true);
                    if floor > self.floor {
                        (Some(Command::MoveUp), ElevatorState::MovingUp)
                    } else {
                        (Some(Command::MoveDown), ElevatorState::MovingDown)
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorState::OpenDown,
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn handle_moving_up(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        match event {
            Event::Panel(floor) => {
                self.set_destination(floor as usize, true);
                (None, ElevatorState::MovingUp)
            }
            Event::Up(floor) => {
                self.set_up_request(floor as usize, true);
                (None, ElevatorState::MovingUp)
            }
            Event::Down(floor) => {
                self.set_down_request(floor as usize, true);
                (None, ElevatorState::MovingUp)
            }
            Event::Arrived(floor) => {
                self.floor = floor;
                if self.get_destination(floor as usize) {
                    if self.highest_request() > floor || self.get_up_request(floor as usize) {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorState::OpenUp,
                        )
                    } else if self.have_requests() && self.highest_request() <= floor {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorState::OpenDown,
                        )
                    } else {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Off,
                            }),
                            ElevatorState::Open,
                        )
                    }
                } else if self.get_up_request(floor as usize) && self.highest_request() == floor {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorState::OpenDown,
                    )
                } else {
                    (None, ElevatorState::MovingUp)
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn handle_moving_down(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        self.set_destination(self.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                self.set_destination(floor as usize, true);
                (None, ElevatorState::MovingDown)
            }
            Event::Up(floor) => {
                self.set_up_request(floor as usize, true);
                (None, ElevatorState::MovingDown)
            }
            Event::Down(floor) => {
                self.set_down_request(floor as usize, true);
                (None, ElevatorState::MovingDown)
            }
            Event::Arrived(floor) => {
                self.floor = floor;
                if self.get_destination(floor as usize) {
                    if self.lowest_request() < floor || self.get_down_request(floor as usize) {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorState::OpenDown,
                        )
                    } else if self.have_requests() && self.lowest_request() >= floor {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorState::OpenUp,
                        )
                    } else {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Off,
                            }),
                            ElevatorState::Open,
                        )
                    }
                } else if self.get_down_request(floor as usize) {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorState::OpenDown,
                    )
                } else if self.get_up_request(floor as usize) && self.lowest_request() == floor {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorState::OpenUp,
                    )
                } else {
                    (None, ElevatorState::MovingDown)
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn handle_open(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        self.set_destination(self.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.floor {
                    self.set_destination(floor as usize, true);
                    if floor > self.floor {
                        (
                            Some(Command::ChangeIndicator {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorState::OpenUp,
                        )
                    } else {
                        (
                            Some(Command::ChangeIndicator {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorState::OpenDown,
                        )
                    }
                } else {
                    (Some(Command::RejectEvent(event)), ElevatorState::Open)
                }
            }
            Event::Up(floor) => {
                if floor != self.floor {
                    self.set_up_request(floor as usize, true);
                    (None, ElevatorState::Open)
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorState::OpenUp,
                    )
                }
            }
            Event::Down(floor) => {
                if floor != self.floor {
                    self.set_down_request(floor as usize, true);
                    (None, ElevatorState::Open)
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorState::OpenDown,
                    )
                }
            }
            Event::Closed(_) => {
                if !self.have_requests() {
                    (None, ElevatorState::Idle)
                } else {
                    let next = self.highest_request();
                    if next > self.floor {
                        (Some(Command::MoveUp), ElevatorState::MovingUp)
                    } else if next < self.floor {
                        (Some(Command::MoveDown), ElevatorState::MovingDown)
                    } else {
                        panic!("I'm supposed to move!")
                    }
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn handle_open_up(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        self.set_destination(self.floor as usize, false);
        self.set_up_request(self.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.floor {
                    self.set_destination(floor as usize, true);
                    (None, ElevatorState::OpenUp)
                } else {
                    (Some(Command::RejectEvent(event)), ElevatorState::OpenUp)
                }
            }
            Event::Up(floor) => {
                if floor != self.floor {
                    self.set_up_request(floor as usize, true);
                    (None, ElevatorState::OpenUp)
                } else {
                    (Some(Command::RejectEvent(event)), ElevatorState::OpenUp)
                }
            }
            Event::Down(floor) => {
                self.set_down_request(floor as usize, true);
                (None, ElevatorState::OpenUp)
            }
            Event::Closed(floor) => {
                if !self.have_requests() {
                    (None, ElevatorState::Idle)
                } else if self.highest_request() > floor {
                    (Some(Command::MoveUp), ElevatorState::MovingUp)
                } else if self.highest_request() < floor {
                    (Some(Command::MoveDown), ElevatorState::MovingDown)
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorState::OpenDown,
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn handle_open_down(&mut self, event: Event) -> (Option<Command>, ElevatorState) {
        self.set_destination(self.floor as usize, false);
        self.set_down_request(self.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.floor {
                    self.set_destination(floor as usize, true);
                    (None, ElevatorState::OpenDown)
                } else {
                    (Some(Command::RejectEvent(event)), ElevatorState::OpenDown)
                }
            }
            Event::Up(floor) => {
                self.set_up_request(floor as usize, true);
                (None, ElevatorState::OpenDown)
            }
            Event::Down(floor) => {
                if floor != self.floor {
                    self.set_down_request(floor as usize, true);
                    (None, ElevatorState::OpenDown)
                } else {
                    (Some(Command::RejectEvent(event)), ElevatorState::OpenDown)
                }
            }
            Event::Closed(floor) => {
                if !self.have_requests() {
                    (None, ElevatorState::Idle)
                } else if self.lowest_request() < floor {
                    (Some(Command::MoveDown), ElevatorState::MovingDown)
                } else if self.lowest_request() > floor {
                    (Some(Command::MoveUp), ElevatorState::MovingUp)
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor: self.floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorState::OpenUp,
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    fn check_invariants(&self) {
        match &self.state {
            ElevatorState::Idle => {
                // Should not be idle if there are requests
                assert!(!self.have_requests());
            }
            ElevatorState::MovingUp => {
                assert!(self.floor < HIGHEST_FLOOR);
                assert!(self.have_requests());
                assert!(self.highest_request() > self.floor);
            }
            ElevatorState::MovingDown => {
                assert!(self.floor > LOWEST_FLOOR);
                assert!(self.have_requests());
                assert!(self.lowest_request() < self.floor);
            }
            ElevatorState::Open => {
                assert!(!self.get_up_request(self.floor as usize));
                assert!(!self.get_down_request(self.floor as usize));
                for floor in LOWEST_FLOOR..HIGHEST_FLOOR + 1 {
                    assert!(!self.get_destination(floor as usize));
                }
            }
            ElevatorState::OpenUp => {
                assert!(self.floor < HIGHEST_FLOOR);
                assert!(!self.get_up_request(self.floor as usize));
                assert!(!self.get_destination(self.floor as usize));
            }
            ElevatorState::OpenDown => {
                assert!(self.floor > LOWEST_FLOOR);
                assert!(!self.get_down_request(self.floor as usize));
                assert!(!self.get_destination(self.floor as usize));
            }
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        match &self.state {
            ElevatorState::Open | ElevatorState::OpenUp | ElevatorState::OpenDown => {
                Some(Event::Closed(self.floor))
            }
            ElevatorState::MovingUp => Some(Event::Arrived(self.floor + 1)),
            ElevatorState::MovingDown => Some(Event::Arrived(self.floor - 1)),
            ElevatorState::Idle => None,
        }
    }

    pub fn set_destination(&mut self, floor: usize, status: bool) {
        self.destinations[floor - LOWEST_FLOOR as usize] = status;
    }

    pub fn get_destination(&self, floor: usize) -> bool {
        self.destinations[floor - LOWEST_FLOOR as usize]
    }

    pub fn set_up_request(&mut self, floor: usize, status: bool) {
        self.up_requests[floor - LOWEST_FLOOR as usize] = status;
    }

    pub fn get_up_request(&self, floor: usize) -> bool {
        self.up_requests[floor - LOWEST_FLOOR as usize]
    }

    pub fn set_down_request(&mut self, floor: usize, status: bool) {
        self.down_requests[floor - LOWEST_FLOOR as usize] = status;
    }

    pub fn get_down_request(&self, floor: usize) -> bool {
        self.down_requests[floor - LOWEST_FLOOR as usize]
    }

    pub fn have_requests(&self) -> bool {
        self.destinations.iter().any(|x| *x)
            || self.up_requests.iter().any(|x| *x)
            || self.down_requests.iter().any(|x| *x)
    }

    fn has_request_at_floor(&self, floor_index: usize) -> bool {
        self.destinations[floor_index]
            || self.up_requests[floor_index]
            || self.down_requests[floor_index]
    }

    pub fn highest_request(&self) -> u8 {
        (0..self.destinations.len())
            .rev()
            .find(|&i| self.has_request_at_floor(i))
            .map(|i| i as u8 + LOWEST_FLOOR)
            .unwrap_or(LOWEST_FLOOR)
    }

    pub fn lowest_request(&self) -> u8 {
        (0..self.destinations.len())
            .find(|&i| self.has_request_at_floor(i))
            .map(|i| i as u8 + LOWEST_FLOOR)
            .unwrap_or(LOWEST_FLOOR)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const FUZZSTEPS: usize = 1000000;

    fn random_possible_event(elev: &Elevator) -> Event {
        // If the current state leads to some future event (like changing a floor),
        // we'll prefer to do that most of the time.
        if let Some(event) = elev.produce_future_event() {
            if fastrand::f64() < 0.75 {
                return event;
            }
        }
        // Otherwise, press a random button
        match fastrand::usize(..3) {
            0 => Event::Panel(fastrand::u8(LOWEST_FLOOR..HIGHEST_FLOOR + 1)),
            1 => Event::Up(fastrand::u8(LOWEST_FLOOR..HIGHEST_FLOOR)),
            2 => Event::Down(fastrand::u8(LOWEST_FLOOR + 1..HIGHEST_FLOOR + 1)),
            _ => panic!(),
        }
    }

    #[test]
    fn fuzz_test() {
        let mut elevator = Elevator::new();
        for _ in 0..FUZZSTEPS {
            let evt = random_possible_event(&elevator);
            elevator.handle(evt);
        }
    }
}
