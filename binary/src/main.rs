use bevy::prelude::*;
use lib::{LibPlugin, TestLabel};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LibPlugin)
        .add_state(MyStates::Test)
        .add_system_set(
            SystemSet::on_update(MyStates::Test)
                .with_system(first.before(TestLabel::First))
                .with_system(second.after(TestLabel::First).before(TestLabel::Second))
                .with_system(third.after(TestLabel::Second)),
        )
        .run();
}

fn first() {
    println!("First system in bin");
}

fn second() {
    println!("second in bin");
}

fn third() {
    println!("third in bin");
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    Test,
}