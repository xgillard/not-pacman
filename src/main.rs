use pacman::{main_loop, BResult, BTermBuilder, State};

fn main() -> BResult<()> {
    let mut state = State::new();
    let (w, h) = state.load_file("map.txt").expect("could not load map");
    
    let context = BTermBuilder::new()
        .with_title("not pacman")
        .with_dimensions(w, h)
        .with_fps_cap(30.0)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("pacman32.png", 32, 32)
        .with_simple_console(w, h,"pacman32.png")
        .with_simple_console_no_bg(w, h,"pacman32.png")
        .build()?;

    main_loop(context, state)?;

    Ok(())
}
