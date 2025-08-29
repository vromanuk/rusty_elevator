use crate::logic::{Command, Event, Indicator, HIGHEST_FLOOR, LOWEST_FLOOR, NUM_FLOORS};

#[derive(Debug, Clone)]
pub struct ElevatorData {
    pub floor: u8,
    destinations: [bool; NUM_FLOORS as usize],
    up_requests: [bool; NUM_FLOORS as usize],
    down_requests: [bool; NUM_FLOORS as usize],
}

impl ElevatorData {
    pub fn new() -> Self {
        ElevatorData {
            floor: LOWEST_FLOOR,
            destinations: [false; NUM_FLOORS as usize],
            up_requests: [false; NUM_FLOORS as usize],
            down_requests: [false; NUM_FLOORS as usize],
        }
    }

    pub fn set_destination(&mut self, floor: usize, value: bool) {
        self.destinations[floor] = value;
    }

    pub fn get_destination(&self, floor: usize) -> bool {
        self.destinations[floor]
    }

    pub fn set_up_request(&mut self, floor: usize, value: bool) {
        self.up_requests[floor] = value;
    }

    pub fn get_up_request(&self, floor: usize) -> bool {
        self.up_requests[floor]
    }

    pub fn set_down_request(&mut self, floor: usize, value: bool) {
        self.down_requests[floor] = value;
    }

    pub fn get_down_request(&self, floor: usize) -> bool {
        self.down_requests[floor]
    }

    pub fn have_requests(&self) -> bool {
        self.destinations.iter().any(|&x| x)
            || self.up_requests.iter().any(|&x| x)
            || self.down_requests.iter().any(|&x| x)
    }

    pub fn highest_request(&self) -> u8 {
        for floor in (LOWEST_FLOOR..=HIGHEST_FLOOR).rev() {
            if self.get_destination(floor as usize)
                || self.get_up_request(floor as usize)
                || self.get_down_request(floor as usize)
            {
                return floor;
            }
        }
        self.floor
    }

    pub fn lowest_request(&self) -> u8 {
        for floor in LOWEST_FLOOR..=HIGHEST_FLOOR {
            if self.get_destination(floor as usize)
                || self.get_up_request(floor as usize)
                || self.get_down_request(floor as usize)
            {
                return floor;
            }
        }
        self.floor
    }
}

// State structs
#[derive(Debug)]
pub struct IdleElevator {
    data: ElevatorData,
}

#[derive(Debug)]
pub struct MovingUpElevator {
    data: ElevatorData,
}

#[derive(Debug)]
pub struct MovingDownElevator {
    data: ElevatorData,
}

#[derive(Debug)]
pub struct OpenElevator {
    data: ElevatorData,
}

#[derive(Debug)]
pub struct OpenUpElevator {
    data: ElevatorData,
}

#[derive(Debug)]
pub struct OpenDownElevator {
    data: ElevatorData,
}

// Enums for state transitions (since Rust can't return different types from same method)
#[derive(Debug)]
pub enum ElevatorAfterEvent {
    Idle(IdleElevator),
    MovingUp(MovingUpElevator),
    MovingDown(MovingDownElevator),
    Open(OpenElevator),
    OpenUp(OpenUpElevator),
    OpenDown(OpenDownElevator),
}

impl ElevatorAfterEvent {
    pub fn handle(self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        match self {
            ElevatorAfterEvent::Idle(elevator) => elevator.handle(event),
            ElevatorAfterEvent::MovingUp(elevator) => elevator.handle(event),
            ElevatorAfterEvent::MovingDown(elevator) => elevator.handle(event),
            ElevatorAfterEvent::Open(elevator) => elevator.handle(event),
            ElevatorAfterEvent::OpenUp(elevator) => elevator.handle(event),
            ElevatorAfterEvent::OpenDown(elevator) => elevator.handle(event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        match self {
            ElevatorAfterEvent::Idle(elevator) => elevator.produce_future_event(),
            ElevatorAfterEvent::MovingUp(elevator) => elevator.produce_future_event(),
            ElevatorAfterEvent::MovingDown(elevator) => elevator.produce_future_event(),
            ElevatorAfterEvent::Open(elevator) => elevator.produce_future_event(),
            ElevatorAfterEvent::OpenUp(elevator) => elevator.produce_future_event(),
            ElevatorAfterEvent::OpenDown(elevator) => elevator.produce_future_event(),
        }
    }
}

impl IdleElevator {
    pub fn new() -> Self {
        IdleElevator {
            data: ElevatorData::new(),
        }
    }

    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        match event {
            Event::Panel(floor) => {
                if floor != self.data.floor {
                    self.data.set_destination(floor as usize, true);
                    if floor > self.data.floor {
                        (
                            Some(Command::MoveUp),
                            ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::MoveDown),
                            ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                        )
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Off,
                        }),
                        ElevatorAfterEvent::Open(OpenElevator { data: self.data }),
                    )
                }
            }
            Event::Up(floor) => {
                if floor != self.data.floor {
                    self.data.set_up_request(floor as usize, true);
                    if floor > self.data.floor {
                        (
                            Some(Command::MoveUp),
                            ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::MoveDown),
                            ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                        )
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                    )
                }
            }
            Event::Down(floor) => {
                if floor != self.data.floor {
                    self.data.set_down_request(floor as usize, true);
                    if floor > self.data.floor {
                        (
                            Some(Command::MoveUp),
                            ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::MoveDown),
                            ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                        )
                    }
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        None
    }
}

impl MovingUpElevator {
    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        match event {
            Event::Panel(floor) => {
                self.data.set_destination(floor as usize, true);
                (None, ElevatorAfterEvent::MovingUp(self))
            }
            Event::Up(floor) => {
                self.data.set_up_request(floor as usize, true);
                (None, ElevatorAfterEvent::MovingUp(self))
            }
            Event::Down(floor) => {
                self.data.set_down_request(floor as usize, true);
                (None, ElevatorAfterEvent::MovingUp(self))
            }
            Event::Arrived(floor) => {
                self.data.floor = floor;

                if self.data.get_destination(floor as usize) {
                    if self.data.highest_request() > floor
                        || self.data.get_up_request(floor as usize)
                    {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                        )
                    } else if self.data.have_requests() && self.data.highest_request() <= floor {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Off,
                            }),
                            ElevatorAfterEvent::Open(OpenElevator { data: self.data }),
                        )
                    }
                } else if self.data.get_up_request(floor as usize)
                    && self.data.highest_request() == floor
                {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                    )
                } else {
                    (None, ElevatorAfterEvent::MovingUp(self))
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        Some(Event::Arrived(self.data.floor + 1))
    }
}

impl MovingDownElevator {
    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        // Clear destination when entering moving down state
        self.data.set_destination(self.data.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                self.data.set_destination(floor as usize, true);
                (None, ElevatorAfterEvent::MovingDown(self))
            }
            Event::Up(floor) => {
                self.data.set_up_request(floor as usize, true);
                (None, ElevatorAfterEvent::MovingDown(self))
            }
            Event::Down(floor) => {
                self.data.set_down_request(floor as usize, true);
                (None, ElevatorAfterEvent::MovingDown(self))
            }
            Event::Arrived(floor) => {
                self.data.floor = floor;

                if self.data.get_destination(floor as usize) {
                    if self.data.lowest_request() < floor
                        || self.data.get_down_request(floor as usize)
                    {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                        )
                    } else if self.data.have_requests() && self.data.lowest_request() >= floor {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::StopAndOpen {
                                floor,
                                direction: Indicator::Off,
                            }),
                            ElevatorAfterEvent::Open(OpenElevator { data: self.data }),
                        )
                    }
                } else if self.data.get_down_request(floor as usize) {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                    )
                } else if self.data.get_up_request(floor as usize)
                    && self.data.lowest_request() == floor
                {
                    (
                        Some(Command::StopAndOpen {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                    )
                } else {
                    (None, ElevatorAfterEvent::MovingDown(self))
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        Some(Event::Arrived(self.data.floor - 1))
    }
}

// Open State Implementation
impl OpenElevator {
    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        // Clear destination when entering open state
        self.data.set_destination(self.data.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.data.floor {
                    self.data.set_destination(floor as usize, true);
                    if floor > self.data.floor {
                        (
                            Some(Command::ChangeIndicator {
                                floor,
                                direction: Indicator::Up,
                            }),
                            ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                        )
                    } else {
                        (
                            Some(Command::ChangeIndicator {
                                floor,
                                direction: Indicator::Down,
                            }),
                            ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                        )
                    }
                } else {
                    (
                        Some(Command::RejectEvent(event)),
                        ElevatorAfterEvent::Open(self),
                    )
                }
            }
            Event::Up(floor) => {
                if floor != self.data.floor {
                    self.data.set_up_request(floor as usize, true);
                    (None, ElevatorAfterEvent::Open(self))
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                    )
                }
            }
            Event::Down(floor) => {
                if floor != self.data.floor {
                    self.data.set_down_request(floor as usize, true);
                    (None, ElevatorAfterEvent::Open(self))
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor,
                            direction: Indicator::Down,
                        }),
                        ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                    )
                }
            }
            Event::Closed(_) => {
                if !self.data.have_requests() {
                    (
                        None,
                        ElevatorAfterEvent::Idle(IdleElevator { data: self.data }),
                    )
                } else {
                    let next = self.data.highest_request();
                    if next > self.data.floor {
                        (
                            Some(Command::MoveUp),
                            ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                        )
                    } else if next < self.data.floor {
                        (
                            Some(Command::MoveDown),
                            ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                        )
                    } else {
                        panic!("I'm supposed to move!")
                    }
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        Some(Event::Closed(self.data.floor))
    }
}

impl OpenUpElevator {
    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        // Clear destination and up request when entering open up state
        self.data.set_destination(self.data.floor as usize, false);
        self.data.set_up_request(self.data.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.data.floor {
                    self.data.set_destination(floor as usize, true);
                    (None, ElevatorAfterEvent::OpenUp(self))
                } else {
                    (
                        Some(Command::RejectEvent(event)),
                        ElevatorAfterEvent::OpenUp(self),
                    )
                }
            }
            Event::Up(floor) => {
                if floor != self.data.floor {
                    self.data.set_up_request(floor as usize, true);
                    (None, ElevatorAfterEvent::OpenUp(self))
                } else {
                    (
                        Some(Command::RejectEvent(event)),
                        ElevatorAfterEvent::OpenUp(self),
                    )
                }
            }
            Event::Down(floor) => {
                self.data.set_down_request(floor as usize, true);
                (None, ElevatorAfterEvent::OpenUp(self))
            }
            Event::Closed(floor) => {
                if !self.data.have_requests() {
                    (
                        None,
                        ElevatorAfterEvent::Idle(IdleElevator { data: self.data }),
                    )
                } else if self.data.highest_request() > floor {
                    (
                        Some(Command::MoveUp),
                        ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                    )
                } else if self.data.highest_request() < floor {
                    (
                        Some(Command::MoveDown),
                        ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                    )
                } else {
                    (
                        Some(Command::OpenDoor {
                            floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorAfterEvent::OpenDown(OpenDownElevator { data: self.data }),
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        Some(Event::Closed(self.data.floor))
    }
}

impl OpenDownElevator {
    pub fn handle(mut self, event: Event) -> (Option<Command>, ElevatorAfterEvent) {
        // Clear destination and down request when entering open down state
        self.data.set_destination(self.data.floor as usize, false);
        self.data.set_down_request(self.data.floor as usize, false);

        match event {
            Event::Panel(floor) => {
                if floor != self.data.floor {
                    self.data.set_destination(floor as usize, true);
                    (None, ElevatorAfterEvent::OpenDown(self))
                } else {
                    (
                        Some(Command::RejectEvent(event)),
                        ElevatorAfterEvent::OpenDown(self),
                    )
                }
            }
            Event::Up(floor) => {
                self.data.set_up_request(floor as usize, true);
                (None, ElevatorAfterEvent::OpenDown(self))
            }
            Event::Down(floor) => {
                if floor != self.data.floor {
                    self.data.set_down_request(floor as usize, true);
                    (None, ElevatorAfterEvent::OpenDown(self))
                } else {
                    (
                        Some(Command::RejectEvent(event)),
                        ElevatorAfterEvent::OpenDown(self),
                    )
                }
            }
            Event::Closed(floor) => {
                if !self.data.have_requests() {
                    (
                        None,
                        ElevatorAfterEvent::Idle(IdleElevator { data: self.data }),
                    )
                } else if self.data.lowest_request() < floor {
                    (
                        Some(Command::MoveDown),
                        ElevatorAfterEvent::MovingDown(MovingDownElevator { data: self.data }),
                    )
                } else if self.data.lowest_request() > floor {
                    (
                        Some(Command::MoveUp),
                        ElevatorAfterEvent::MovingUp(MovingUpElevator { data: self.data }),
                    )
                } else {
                    (
                        Some(Command::ChangeIndicator {
                            floor: self.data.floor,
                            direction: Indicator::Up,
                        }),
                        ElevatorAfterEvent::OpenUp(OpenUpElevator { data: self.data }),
                    )
                }
            }
            _ => panic!("Unexpected event: {:?}", event),
        }
    }

    pub fn produce_future_event(&self) -> Option<Event> {
        Some(Event::Closed(self.data.floor))
    }
}
