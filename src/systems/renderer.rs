use ncurses::*;
use amethyst::ecs::{
    System,
    ReadStorage,
    ReadExpect
};
use crate::components::{
    ascii_representation::*,
    position::*
};
use std::sync::atomic::AtomicPtr;

use crate::resources::grid::*;


// Curses windows use *mut i8 as the library are just bindings to C.
// The execution shouldn't need an atomic reference as the rendering will be single-threaded but...
// Amethyst requires the systems to be passed between threads to paralelize it's execution so this
// Atomic Pointer is just a circunvent for the Type System as otherwise the compiler will start crying.
// All this **** because i wanted to keep an atribute of the window just in case I want to render
// inside a ncurses window
// Ahhhh. I love been a programmer, sometimes i feel like a magician.
type CursesWindowAtomicReference = AtomicPtr<i8>;

pub struct CursesRenderer{
    w: CursesWindowAtomicReference
}


impl CursesRenderer{

    pub fn new() -> Self{
        initscr();
        start_color();
        use_default_colors();
        for x in 1..20{
            init_pair(x, x, -1);
        }
        keypad(stdscr(), true);
        noecho();
        wclear(stdscr());

        CursesRenderer{
            w: AtomicPtr::from(stdscr())
        }
    }

    pub fn get_window_dimensions(&mut self) -> Dimensions{
        let mut max_x = 0;
        let mut max_y = 0;
        let win = *(self.w.get_mut());
        getmaxyx(win, &mut max_y, &mut max_x);
        Position::new(max_x, max_y)
    }

    fn render_char(&mut self, p: Position, tile: AsciiRepresentation){
        let color = tile.get_color();
        let char = tile.get_char();
        let (x, y) = p.get_coords();
        let win = *(self.w.get_mut());
        wmove(win, y, x);
        wattron(win, COLOR_PAIR(color));
        waddch(win, u32::from(char));
        wattroff(win, COLOR_PAIR(color));
    }

}

impl<'s> System<'s> for CursesRenderer{
    type SystemData = (
        ReadExpect<'s, Grid>,
        ReadStorage<'s, AsciiRepresentation>
    );

    fn run(&mut self, (grid, ascii): Self::SystemData) {
        let win = *(self.w.get_mut());
        wrefresh(win);
        let (x, y) = grid.get_dimensions().get_coords();
        for i in 0..x{
            for j in 0..y{
                let p = Position::new(i, j);
                let entity = grid.get_tile(p);
                let tile = match entity {
                    Some(e) => {
                        *ascii.get(e).unwrap()
                    }
                    None => AsciiRepresentation::default(),
                };
                self.render_char(p, tile);
            }
        }
    }
}
