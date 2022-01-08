use crate::prelude::*;

pub const SCREEN_WIDTH: i32 = 7;
pub const SCREEN_HEIGHT: i32 = 4;

pub const CONSOLE_TABLE: usize = 0;
pub const CONSOLE_GAME: usize = 1;
pub const CONSOLE_BUTTONS: usize = 2;
pub const CONSOLE_PANEL_FIXED: usize = 3;
pub const CONSOLE_PANEL_DYNAMIC: usize = 4;

pub const VALUE_RUST: usize = 52;
pub const BET_IMG: usize = 53;
pub const MAX_BET_IMG: usize = 54;
pub const RETRY_IMG: usize = 55;
pub const CREDIT_IMG: usize = 56;
pub const DOUBLE_IMG: usize = 57;
pub const RED_CARD_IMG: usize = 58;
pub const BLACK_CARD_IMG: usize = 59;

pub struct TRender {
    pub context: BTerm,
}

impl TRender {
    pub fn new() -> Self {
        Self {
            context: BTermBuilder::new()
                .with_title("AMSPoker v3.0.0 1996, 2021")
                .with_fps_cap(10.0)
                .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                .with_tile_dimensions(96, 144)
                .with_resource_path("resources/")
                .with_font("table672x576.png", 96, 144)
                .with_font("objects13x5x96x144.png", 96, 144)
                .with_font("cp437_96x144.png", 96, 144)
                .with_sparse_console(SCREEN_WIDTH, SCREEN_HEIGHT, "table672x576.png")
                .with_sparse_console(SCREEN_WIDTH, SCREEN_HEIGHT, "objects13x5x96x144.png")
                .with_sparse_console(SCREEN_WIDTH, SCREEN_HEIGHT, "objects13x5x96x144.png")
                .with_sparse_console_no_bg(SCREEN_WIDTH * 8, SCREEN_HEIGHT * 8, "cp437_96x144.png")
                .with_sparse_console_no_bg(SCREEN_WIDTH * 8, SCREEN_HEIGHT * 8, "cp437_96x144.png")
                .build()
                .unwrap(),
        }
    }
    pub fn display(&mut self) {
        // Table
        TTable::render(&mut self.context);

        // Game Deck and Double
        self.context.set_active_console(CONSOLE_GAME);
        self.context.set(0, 0, WHITE, BLACK, VALUE_RUST);
        self.context.set(6, 0, WHITE, BLACK, VALUE_RUST);

        // Buttons
        TButton::render(&mut self.context);

        // Panel
        TPanel::render(&mut self.context);
    }
}
