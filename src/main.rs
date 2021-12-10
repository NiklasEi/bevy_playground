use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(MyStates::Test)
        .add_system_set(
            SystemSet::on_update(MyStates::Test)
                .with_system(first.exclusive_system().before(TestLabel::A))
                .with_system(second.exclusive_system().after(TestLabel::A)),
        )
        .add_system(system_a.exclusive_system().label(TestLabel::A))
        .run();
}

fn first() {
    println!("1.");
}

fn second() {
    println!("2.");
}

#[derive(SystemLabel, Eq, Debug, Hash, Clone, PartialEq)]
pub enum TestLabel {
    A,
}

fn system_a() {
    println!("A");
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    Test,
}
