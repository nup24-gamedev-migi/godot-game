use std::time::Duration;

use crate::prelude::*;
use crate::player::PlayerState;

const PLAYER_WALK_TIME: Duration = Duration::from_millis(120);

#[derive(Clone, Debug)]
#[derive(Resource, Reflect)]
pub struct InGameState {
    pub animation_timer: Timer,
}

impl InGameState {
    pub fn new() -> Self {
        Self {
            animation_timer: Timer::new(
                Duration::ZERO,
                TimerMode::Once,
            ),
        }
    }
}

pub fn react_to_input(
    time: Res<Time>,
    mut ingame_state: ResMut<InGameState>,
    mut player_q: Query<&mut PlayerState>,
) {
    ingame_state.animation_timer.tick(time.delta());

    let Ok(mut player) = player_q.get_single_mut()
        else { return; };

    let (animating, got_input) = (
        !ingame_state.animation_timer.finished(),
        player.new_direction.is_some()
    );

    match (animating, got_input) {
        (false, true) => {
            trace!("Launching animation timer");
            ingame_state.animation_timer.set_duration(PLAYER_WALK_TIME);
            ingame_state.animation_timer.reset();
        },
        (false, false) => (),
        (true, _) => {
            trace!("Input rejected");
            player.new_direction = None;
        },
    }
}