use crate::logic::Event;

mod controller;
mod logic;

fn main() {
    println!("Hello, elevator!");

    let mut controller = controller::LiftyController::new();
    let mut elevator = logic::Elevator::new();

    loop {
        println!("{:?}", elevator);

        let event = controller.receive();
        handle_event(&mut elevator, &mut controller, event);

        while let Some(future_event) = elevator.produce_future_event() {
            handle_event(&mut elevator, &mut controller, future_event);
        }
    }
}

fn handle_event(
    elevator: &mut logic::Elevator,
    controller: &mut controller::LiftyController,
    event: Event,
) {
    if let Some(command) = elevator.handle(event) {
        controller.send(command);
    }
}
