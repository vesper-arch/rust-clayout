pub mod clay_main {
    // ClayContext is the goddamn backbone of this whole library. It lets functions look at the
    // current open elements so the UI Heirarchy can be constructed. This will be extended to store
    // the info of the layout itself but that's a lotta work and i dont give a shit right now.
    // There should only be a single one of these in existence at any given time. If there are
    // multiple uhh shit's gonna break.
    struct ClayContext {
        layout_elements: Vec<Node>,

        open_layout_elements: Vec<usize>
    }

    impl ClayContext {
        fn get_last_opened_element(&mut self) -> Option<&mut Node> {
            let last_opened_element_index: usize = *self.open_layout_elements.last().expect("There are no currently opened elements");
            
            Some(self.layout_elements.get_mut(last_opened_element_index).unwrap())
        }
    }

    struct Node {
        parent: Option<usize>,
        element: ClayElement,
        child_elements: Vec<usize>
    }

    impl Node {
       fn new(element: ClayElement, parent: usize) -> Self {
           Node {
               parent: Some(parent),
               element,
               child_elements: vec![]
           }
       }

       fn get_parent_element<'a>(&self, context: &'a mut ClayContext) -> &'a mut Node {
           context.layout_elements.get_mut(self.parent.unwrap()).unwrap()
       }
    }

    pub struct Color( u8, u8, u8, u8 );

    pub enum ChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum SizingMode {
        Fixed(i32),
        Fit,
        Grow
    }

    pub struct Sizing {
        width: SizingMode,
        height: SizingMode
    }

    impl Sizing {
        fn new(width: SizingMode, height: SizingMode) -> Self {
            Sizing {width, height}
        }

        fn both(size: SizingMode) -> Self {
            Sizing {width: size, height: size}
        }
    }

    pub struct CornerRadius {
        top_right: i32,
        top_left: i32,
        bottom_left: i32,
        bottom_right: i32
    }

    impl CornerRadius {
        fn new(top_right: i32, top_left: i32, bottom_left: i32, bottom_right: i32) -> Self {
            CornerRadius {top_right, top_left, bottom_left, bottom_right}
        }

        fn all(radius: i32) -> Self {
            CornerRadius {top_right: radius, top_left: radius, bottom_left: radius, bottom_right: radius}
        }
    }

    pub struct Padding {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    impl Padding {
        fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
            Padding {left, right, top, bottom}
        }

        fn hv(left_right: i32, top_bottom: i32) -> Self {
            Padding {left: left_right, right: left_right, top: top_bottom, bottom: top_bottom}
        }

        fn all(padding: i32) -> Self {
            Padding {left: padding, right: padding, top: padding, bottom: padding}
        }
    }

    pub struct ChildGap(pub f32);

    pub enum ChildXAlignment { AlignXLeft, AlignXCenter, AlignXRight }
    pub enum ChildYAlignment { AlignYTop, AlignYCenter, AlignYBottom }

    pub struct ChildAlignment {
        x: ChildXAlignment,
        y: ChildYAlignment,
    }

    impl ChildAlignment {
        fn default() -> Self {
            ChildAlignment {x: ChildXAlignment::AlignXLeft, y: ChildYAlignment::AlignYTop}
        }

        fn new(x_align: ChildXAlignment, y_align: ChildYAlignment) -> Self {
            ChildAlignment {x: x_align, y: y_align}
        }
        
    }

    pub struct LayoutConfig {
        pub sizing: Sizing,
        pub padding: Padding,
        pub child_gap: ChildGap,
        pub layout_direction: ChildLayoutDirection,
        pub child_alignment: ChildAlignment
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            LayoutConfig {
                sizing: Sizing::both(SizingMode::Fit),
                padding: Padding::all(0),
                child_gap: ChildGap(0.0),
                layout_direction: ChildLayoutDirection::LeftToRight,
                child_alignment: ChildAlignment::default()
            }
        }
    }

    pub(crate) struct ClayElement {
        pub(crate) id: Option<String>,
        pub(crate) layout: LayoutConfig,
        pub(crate) color: Color,
        pub(crate) corner_radius: CornerRadius,
        // fields for finalized positions and sizes. Not exposed to the user
        pub(crate) final_size_y: f32,
        pub(crate) final_size_x: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
    }

    impl Default for ClayElement {
        fn default() -> Self {
            ClayElement {
                id: None,
                layout: LayoutConfig::default(),
                color: Color(0, 0, 0, 255),
                corner_radius: CornerRadius::all(0),
                final_pos_y: 0.0,
                final_pos_x: 0.0,
                final_size_y: 0.0,
                final_size_x: 0.0,
            }
        }
    }

    // This is what the user will be working with instead of creating the ClayObject directly.
    // Basic idea is to make this as convenient as possible and then have a function to translate
    // this to the actual ClayObject struct.
    pub struct ElementDeclaration {
        pub element_id: Option<String>,
        pub layout: LayoutConfig,
        pub background_color: Color,
        pub corner_radius: CornerRadius,
        // in the future this will have configs for floating, scroll, border, and custom elements
    }

    fn open_element(context: &mut ClayContext) {
        let mut new_element = ClayElement::default();
        let new_element_index = context.layout_elements.len();
        let mut parent_element: usize = 0;
        
        if context.open_layout_elements.len() > 0 {
            context.layout_elements.last_mut().unwrap().child_elements.push(new_element_index);
            parent_element = *context.open_layout_elements.last().unwrap();
        }

        context.open_layout_elements.push(new_element_index);
        context.layout_elements.push(Node::new(new_element, parent_element));
    }

    fn configure_open_element(context: &mut ClayContext, config: ElementDeclaration) {
        let last_opened_element_index = *context.open_layout_elements.last().unwrap();
        let last_opened_element = context.layout_elements.get_mut(last_opened_element_index).unwrap();

        last_opened_element.element.id = config.element_id;
        last_opened_element.element.color = config.background_color;
        last_opened_element.element.layout = config.layout;
        last_opened_element.element.corner_radius = config.corner_radius;
    }

    fn close_element(context: &mut ClayContext) {
        // Size containers
        let last_opened_element = context.get_last_opened_element().unwrap();
        let parent_element = last_opened_element.get_parent_element(context);
        if parent_element.element.layout.sizing.width == SizingMode::Fit {
            parent_element.element.final_size_x += last_opened_element.element.layout.sizing.width;
        }

        context.open_layout_elements.pop();
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

    // pub fn draw_object(test_obj: &mut clay_main::ClayElement, mut draw_handle: RaylibDrawHandle) {
    //     test_obj.calculate_size();
    //     test_obj.calculate_position();
    //     draw_handle.draw_rectangle_rounded(Rectangle { x: test_obj.final_pos_x, y: test_obj.final_pos_y,
    //         width: test_obj.final_size_x, height: test_obj.final_size_y},
    //         test_obj.border_radius.0,
    //         1,
    //         Color::VIOLET);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 4, 6);
    }
}
