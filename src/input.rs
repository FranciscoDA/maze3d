use raylib::prelude::*;
use KeyboardKey::{
    KEY_A, KEY_D, KEY_DOWN, KEY_LEFT, KEY_LEFT_CONTROL, KEY_LEFT_SHIFT, KEY_RIGHT, KEY_S, KEY_UP, KEY_W,
};

pub struct InputController {
    pub last_mouse_position: Vector2,

    pub key_move_forward: KeyboardKey,
    pub key_move_backward: KeyboardKey,
    pub key_strafe_left: KeyboardKey,
    pub key_strafe_right: KeyboardKey,

    pub key_walk: KeyboardKey,
    pub key_sprint: KeyboardKey,

    pub key_turn_left: KeyboardKey,
    pub key_turn_right: KeyboardKey,
    pub key_look_up: KeyboardKey,
    pub key_look_down: KeyboardKey,
}

fn key2f32(rl: &RaylibHandle, key: KeyboardKey) -> f32 {
    if rl.is_key_down(key) {
        1.0
    } else {
        0.0
    }
}
impl InputController {
    pub fn new(rl: &RaylibHandle) -> Self {
        Self {
            last_mouse_position: rl.get_mouse_position(),

            key_move_forward: KEY_W,
            key_move_backward: KEY_S,
            key_strafe_left: KEY_A,
            key_strafe_right: KEY_D,

            key_walk: KEY_LEFT_CONTROL,
            key_sprint: KEY_LEFT_SHIFT,

            key_turn_left: KEY_LEFT,
            key_turn_right: KEY_RIGHT,
            key_look_up: KEY_UP,
            key_look_down: KEY_DOWN,
        }
    }

    pub fn get_movement_vector(&self, rl: &RaylibHandle) -> Vector3 {
        let forward_movement =
            Vector3::forward() * (key2f32(rl, self.key_move_forward) - key2f32(rl, self.key_move_backward));
        let strafing_movement =
            Vector3::right() * (key2f32(rl, self.key_strafe_left) - key2f32(rl, self.key_strafe_right));
        return forward_movement + strafing_movement;
    }

    pub fn get_move_speed_modifier(
        &self,
        rl: &RaylibHandle,
        walk_speed: f32,
        normal_speed: f32,
        sprint_speed: f32,
    ) -> f32 {
        if rl.is_key_down(self.key_walk) {
            walk_speed
        } else if rl.is_key_down(self.key_sprint) {
            sprint_speed
        } else {
            normal_speed
        }
    }

    pub fn get_turning_angle(&self, rl: &RaylibHandle) -> f32 {
        self.last_mouse_position.x - rl.get_mouse_position().x + key2f32(rl, self.key_turn_left)
            - key2f32(rl, self.key_turn_right)
    }

    pub fn get_vertical_look_angle(&self, rl: &RaylibHandle) -> f32 {
        (self.last_mouse_position.y - rl.get_mouse_position().y) + key2f32(rl, self.key_look_up)
            - key2f32(rl, self.key_look_down)
    }
}
