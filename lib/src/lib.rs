use bevy::prelude::*;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            first
                .exclusive_system()
                .label(TestLabel::First)
                .before(TestLabel::Second),
        )
        .add_system(second.exclusive_system().label(TestLabel::Second));
    }
}

#[derive(SystemLabel, Eq, Debug, Hash, Clone, PartialEq)]
pub enum TestLabel {
    First,
    Second,
}

fn first() {
    println!("First system in lib");
}

fn second() {
    println!("second in lib");
}
