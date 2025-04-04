pub mod clay_main {
    pub struct ClayColor(pub u8, pub u8, pub u8);

    // Always the same size.
    pub struct ClaySizingModeFixed(pub f32, pub f32);
    // Grows to fill the remaining availble space in the parent container
    pub struct ClaySizingModeGrow(pub f32, pub f32);
    // Fits to the sizes of all child objects
    pub struct ClaySizingModeFit(pub f32, pub f32);

    pub enum ClayChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    pub enum ClayObjectSizing {
        Fixed(ClaySizingModeFixed),
        Fit(ClaySizingModeFit),
        Grow(ClaySizingModeGrow)
    }

    pub struct ClayObject {
        pub sizing: ClayObjectSizing,
        pub color: ClayColor,
        pub layout_direction: ClayChildLayoutDirection,
        pub border_radius: f32,
        pub padding: (u32, u32, u32, u32),
        // fields for finalized positions and sizes. Not exposed to the user
        pub(crate) final_size_y: f32,
        pub(crate) final_size_x: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
    }

    impl ClayObject {
        pub fn calculate_size(&mut self) {
            match &self.sizing {
                ClayObjectSizing::Fixed(sizes) => {self.final_size_x = sizes.0; self.final_size_y = sizes.1},
                ClayObjectSizing::Fit(sizes) => {self.final_size_x = sizes.0; self.final_size_y = sizes.1},
                ClayObjectSizing::Grow(sizes) => {self.final_size_x = sizes.0; self.final_size_y = sizes.1},
            }
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
        draw_handle.draw_rectangle_rounded(Rectangle { x: 5.0, y: 5.0, width: test_obj.final_size_x, height: test_obj.final_size_y},
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
