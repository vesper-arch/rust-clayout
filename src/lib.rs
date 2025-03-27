struct ClayColor(u8, u8, u8);

struct ClaySizingModeFixed {
    x: i32,
    y: i32,
}

struct ClayBorderRadius(u32, u32, u32, u32);

enum ClayChildLayoutDirection {
    LeftToRight,
    TopToBottom,
}

struct ClayObject {
    sizing: ClaySizingModeFixed,
    color: ClayColor,
    layout_direction: ClayChildLayoutDirection,
    border_radius: ClayBorderRadius,
    padding: (u32, u32, u32, u32)
}

mod clay_raylib {
    use raylib::prelude::*;
    use super::*;

    pub fn raylib_init() {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("hell")
            .build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
             
            d.clear_background(Color::WHITE);
        }
    }

    pub fn draw_object(test_obj: ClayObject, mut draw_handle: RaylibDrawHandle) {
        draw_handle.draw_rectangle(5, 5, test_obj.sizing.x, test_obj.sizing.y, Color::BLUEVIOLET);
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
