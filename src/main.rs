use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(MyStates::Test)
        .add_system_set(
            SystemSet::on_update(MyStates::Test)
                .with_system(first.exclusive_system().before(TestLabel::First))
                .with_system(
                    second
                        .exclusive_system()
                        .after(TestLabel::First)
                        .before(TestLabel::Second),
                )
                .with_system(third.exclusive_system().after(TestLabel::Second)),
        )
        .add_system(
            system_a
                .exclusive_system()
                .label(TestLabel::First)
                .before(TestLabel::Second),
        )
        .add_system(system_b.exclusive_system().label(TestLabel::Second))
        .run();
}

fn first() {
    println!("1.");
}

fn second() {
    println!("2.");
}

fn third() {
    println!("3.");
}

#[derive(SystemLabel, Eq, Debug, Hash, Clone, PartialEq)]
pub enum TestLabel {
    First,
    Second,
}

fn system_a() {
    println!("A");
}

fn system_b() {
    println!("B");
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    Test,
}
