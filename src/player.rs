use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
#[derive(Component, Reflect)]
pub struct PlayerState {
    pub new_direction: Option<MoveDirection>,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            new_direction: None,
        }
    }
}

pub fn player_reset(
    mut player_q: Query<&mut PlayerState>,
) {
    let Ok(mut st) = player_q.get_single_mut()
        else { return; };

    st.new_direction = None;
}

pub fn player_input(
    mut player_q: Query<&mut PlayerState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut st) = player_q.get_single_mut()
        else {
            trace!("No player found");
            return;
        };

    st.new_direction = None;

    if keyboard_input.pressed(KeyCode::KeyA) {
        st.new_direction = Some(MoveDirection::Left);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        st.new_direction = Some(MoveDirection::Up);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        st.new_direction = Some(MoveDirection::Right);
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        st.new_direction = Some(MoveDirection::Down);
    }

    if let Some(dir) = st.new_direction {
        debug!("player dir:{dir:?}");
    }
}