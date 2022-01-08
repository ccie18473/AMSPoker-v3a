use crate::prelude::*;
pub struct TButton {}

impl TButton {
    pub fn new_buttons() -> Self {
        Self {}
    }
    pub fn render(ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_BUTTONS);

        ctx.set(0, 3, WHITE, BLACK, BET_IMG);
        ctx.set(1, 3, WHITE, BLACK, MAX_BET_IMG);
        ctx.set(2, 3, WHITE, BLACK, RETRY_IMG);
        ctx.set(3, 3, WHITE, BLACK, CREDIT_IMG);
        ctx.set(4, 3, WHITE, BLACK, DOUBLE_IMG);
        ctx.set(5, 3, WHITE, BLACK, RED_CARD_IMG);
        ctx.set(6, 3, WHITE, BLACK, BLACK_CARD_IMG);
    }
    pub fn blink(ctx: &mut BTerm, x: i32, y: i32) {
        ctx.set_active_console(CONSOLE_BUTTONS);

        ctx.set(x, y, WHITE, BLACK, VALUE_RUST);

        // Not supported by WASM
        //thread::sleep(Duration::from_millis(100));
    }
}
