use crate::logic::Event;
use crate::states::{ElevatorAfterEvent, IdleElevator};

mod controller;
mod logic;
mod states;

fn main() {
    println!("Hello, elevator!");

    // let mut controller = controller::LiftyController::new();
    // let mut elevator = logic::Elevator::new();
    //
    // loop {
    //     println!("{:?}", elevator);
    //
    //     let event = controller.receive();
    //     handle_event(&mut elevator, &mut controller, event);
    //
    //     while let Some(future_event) = elevator.produce_future_event() {
    //         handle_event(&mut elevator, &mut controller, future_event);
    //     }
    // }

    let mut controller = controller::LiftyController::new();
    let mut elevator = ElevatorAfterEvent::Idle(IdleElevator::new());

    loop {
        println!("{:?}", elevator);
        let event = controller.receive();

        elevator = handle_event(elevator, &mut controller, event);

        while let Some(future_event) = elevator.produce_future_event() {
            elevator = handle_event(elevator, &mut controller, future_event);
        }
    }
}

// fn handle_event(
//     elevator: &mut logic::Elevator,
//     controller: &mut controller::LiftyController,
//     event: Event,
// ) {
//     if let Some(command) = elevator.handle(event) {
//         controller.send(command);
//     }
// }

fn handle_event(
    elevator: ElevatorAfterEvent, // Take ownership
    controller: &mut controller::LiftyController,
    event: Event,
) -> ElevatorAfterEvent {
    let (command, new_elevator) = elevator.handle(event);

    if let Some(command) = command {
        controller.send(command);
    }

    new_elevator
}
