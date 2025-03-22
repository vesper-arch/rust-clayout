use raylib::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn clay_raylib_initialize() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Stupid rectangle die piece of shit")
        .vsync()
        .build();
    return (rl, thread)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
