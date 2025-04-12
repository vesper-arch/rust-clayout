pub mod clay_main {
    pub struct ClayColor(pub u8, pub u8, pub u8);
    

    // ClayContext is the goddamn backbone of this whole library. It lets functions look at the
    // current open elements so the UI Heirarchy can be constructed. This will be extended to store
    // the info of the layout itself but that's a lotta work and i dont give a shit right now.
    // There should only be a single one of these in existence at any given time. If there are
    // multiple uhh shit's gonna break.
    struct ClayContext<'a> {
        layout_elements: Vec<&'a ClayObject<'a>>,

        open_layout_elements: Vec<&'a ClayObject<'a>>
    }

    impl ClayContext<'_> {
        pub fn get_current_context(&'_ self) -> Self {
            return Self {
                layout_elements: self.layout_elements.clone(),
                open_layout_elements: self.open_layout_elements.clone()
            }
        } 

        pub fn set_context(&mut self, context: Self) {
            *self = context;
        }
    }

    pub enum ClayChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    pub enum ClayObjectSizing {
        Fixed(f32, f32),
        Fit(f32, f32),
        Grow(f32, f32)
    }

    pub struct ClayBorderRadius(pub f32);
    pub struct ClayPadding(pub f32, pub f32, pub f32, pub f32);

    pub struct ClayObject<'a> {
        pub sizing: ClayObjectSizing,
        pub color: ClayColor,
        pub layout_direction: ClayChildLayoutDirection,
        pub border_radius: ClayBorderRadius,
        pub padding: ClayPadding,
        // fields for finalized positions and sizes. Not exposed to the user
        pub(crate) final_size_y: f32,
        pub(crate) final_size_x: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
        
        pub(crate) child_elements: Vec<&'a Self>
    }

    impl ClayObject<'_> {
        pub fn new(sizing: ClayObjectSizing, color: ClayColor, layout_direction: ClayChildLayoutDirection, border_radius: ClayBorderRadius, padding: ClayPadding) -> Self {
            Self {
                sizing,
                color,
                layout_direction,
                border_radius,
                padding,
                final_size_y: 0.0,
                final_size_x: 0.0,
                final_pos_x: 0.0,
                final_pos_y: 0.0,

                child_elements: vec![],
            }
        }

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

    impl Default for ClayObject<'_> {
        fn default() -> ClayObject<'static> {
            ClayObject {
                sizing: ClayObjectSizing::Fixed(0.0, 0.0),
                color: ClayColor(255, 255, 255),
                layout_direction: ClayChildLayoutDirection::LeftToRight,
                border_radius: ClayBorderRadius(0.0),
                padding: ClayPadding(0.0, 0.0, 0.0, 0.0),
                final_pos_y: 0.0,
                final_pos_x: 0.0,
                final_size_y: 0.0,
                final_size_x: 0.0,
                child_elements: vec![]
            }
        }
    }

    fn open_element() {
        let current_context = ClayContext::get_current_context();

        let new_element = ClayObject::default();
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

    pub fn draw_object(test_obj: &mut clay_main::ClayObject, mut draw_handle: RaylibDrawHandle) {
        test_obj.calculate_size();
        test_obj.calculate_position();
        draw_handle.draw_rectangle_rounded(Rectangle { x: test_obj.final_pos_x, y: test_obj.final_pos_y,
            width: test_obj.final_size_x, height: test_obj.final_size_y},
            test_obj.border_radius.0,
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
