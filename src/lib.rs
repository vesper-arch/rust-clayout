pub mod clay_main {
    pub struct ClayColor(pub u8, pub u8, pub u8);

    pub struct ClaySizingModeFixed {
        pub x: i32,
        pub y: i32,
    }

    pub enum ClayChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    pub struct ClayObject {
        pub sizing: ClaySizingModeFixed,
        pub color: ClayColor,
        pub layout_direction: ClayChildLayoutDirection,
        pub border_radius: f32,
        pub padding: (u32, u32, u32, u32)
    }
}

pub mod clay_raylib {
    use raylib::prelude::*;
    use crate::clay_main;

    pub fn raylib_init() -> (RaylibHandle, RaylibThread){
        let (rl, thread) = raylib::init()
            .size(640, 480)
            .title("hell")
            .build();

        return (rl, thread)
    }

    pub fn draw_object(test_obj: &clay_main::ClayObject, mut draw_handle: RaylibDrawHandle) {
        draw_handle.draw_rectangle_rounded(Rectangle { x: 5.0, y: 5.0, width: test_obj.sizing.x as f32, height: test_obj.sizing.y as f32},
            test_obj.border_radius,
            1,
            Color::WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 4, 6);
    }
}
