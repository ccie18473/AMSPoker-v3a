mod button;
mod card;
mod deck;
mod game;
mod panel;
mod render;
mod table;

const MOUSE_LEFT_BTN: usize = 0;

mod prelude {
    pub use crate::button::*;
    pub use crate::card::*;
    pub use crate::deck::*;
    pub use crate::game::*;
    pub use crate::panel::*;
    pub use crate::render::*;
    pub use crate::table::*;
    pub use bracket_lib::prelude::*;
    pub use std::thread;
    pub use std::time::Duration;
}

use prelude::*;

embedded_resource!(TILE_FONT1, "../resources/table672x576.png");
embedded_resource!(TILE_FONT2, "../resources/objects13x5x96x144.png");
embedded_resource!(TILE_FONT3, "../resources/cp437_96x144.png");

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Playing,
}

struct State {
    table: TTable,
    mode: Mode,
}

impl State {
    fn new() -> Self {
        Self {
            table: TTable::new_table(),
            mode: Mode::Waiting,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_TABLE);

        if self.mode == Mode::Waiting {
            let mouse_point = INPUT.lock().mouse_tile(CONSOLE_TABLE);
            TButton::render(ctx);

            if INPUT.lock().is_mouse_button_pressed(MOUSE_LEFT_BTN) {
                self.mode = Mode::Playing;
                if ctx.left_click {
                    TTable::process_mouse(&mut self.table, ctx, mouse_point);
                }
            }
        } else {
            self.mode = Mode::Waiting;
        }
    }
}
fn main() -> BError {
    link_resource!(TILE_FONT1, "resources/table672x576.png");
    link_resource!(TILE_FONT2, "resources/objects13x5x96x144.png");
    link_resource!(TILE_FONT3, "resources/cp437_96x144.png");

    let mut render = TRender::new();
    render.display();

    main_loop(render.context, State::new())
}
