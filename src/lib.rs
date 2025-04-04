pub mod clay_main {
    pub struct ClayColor(pub u8, pub u8, pub u8);

    // // Always the same size.
    // pub struct ClaySizingModeFixed(pub f32, pub f32);
    // // Grows to fill the remaining availble space in the parent container
    // pub struct ClaySizingModeGrow(pub f32, pub f32);
    // // Fits to the sizes of all child objects
    // pub struct ClaySizingModeFit(pub f32, pub f32);

    pub enum ClayChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    pub enum ClayObjectSizing {
        Fixed(f32, f32),
        Fit(f32, f32),
        Grow(f32, f32)
    }

    pub struct ClayObject {
        pub sizing: ClayObjectSizing,
        pub color: ClayColor,
        pub layout_direction: ClayChildLayoutDirection,
        pub border_radius: f32,
        pub padding: (f32, f32, f32, f32),
        // fields for finalized positions and sizes. Not exposed to the user
        pub(crate) final_size_y: f32,
        pub(crate) final_size_x: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
    }

    impl ClayObject {
        pub fn calculate_size(&mut self) {
            match &self.sizing {
                ClayObjectSizing::Fixed(x, y) => {self.final_size_x = *x; self.final_size_y = *y},
                ClayObjectSizing::Fit(x, y) => {self.final_size_x = *x; self.final_size_y = *y},
                ClayObjectSizing::Grow(x, y) => {self.final_size_x = *x; self.final_size_y = *y},
            }
        }

        pub fn calculate_position(&mut self) {
            self.final_pos_x = self.padding.0;
            self.final_pos_y = self.padding.1;
        }
    }
}

pub mod clay_raylib {
    use raylib::prelude::*;
    use crate::clay_main;

    pub fn raylib_init() -> (RaylibHandle, RaylibThread){
        let (rl, thread) = raylib::init()
            .size(640, 480)
            .title("hell")
            .resizable()
            .build();

        return (rl, thread)
    }

    pub fn draw_object(test_obj: &clay_main::ClayObject, mut draw_handle: RaylibDrawHandle) {
        draw_handle.draw_rectangle_rounded(Rectangle { x: test_obj.final_pos_x, y: test_obj.final_pos_y,
            width: test_obj.final_size_x, height: test_obj.final_size_y},
            test_obj.border_radius,
            1,
            Color::VIOLET);
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
