#[derive(Debug)]
enum Direction {
    Up,
    Down
}

#[derive(Debug)]
enum CarDoor {
    Opened,
    Closed
}

type floor = i32;

#[derive(Debug)]
enum Button {
    FloorButton(floor),
    DirectionalButton(Direction)
}

#[derive(Debug)]
enum Event {
    Door(CarDoor),
    CarArrived(floor),
    ButtonPressed(Button)
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::Door(CarDoor::Opened)
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::Door(CarDoor::Closed)
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::DirectionalButton(dir))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::FloorButton(floor))
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
