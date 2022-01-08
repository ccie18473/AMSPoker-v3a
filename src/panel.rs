use crate::prelude::*;

pub const MSG1: &str = "Bet then tap the Deck Card";
pub const MSG2: &str = "Select those you don't want";
pub const MSG3: &str = "Good Luck";
pub const MSG4: &str = "You won, select or credit";
pub const MSG5: &str = "You won, double or credit";
pub const MSG6: &str = "No more cards in the Deck";

pub struct TPanel {
    pub prize: String,
    pub value: usize,
    pub bets: isize,
    pub wins: isize,
    pub msg: String,
    pub credits: isize,
}

impl TPanel {
    pub fn new_panel() -> Self {
        Self {
            prize: "".to_string(),
            value: 0,
            bets: 0,
            wins: 0,
            msg: MSG1.to_string(),
            credits: CREDITS,
        }
    }
    pub fn render(ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_PANEL_FIXED);

        ctx.print_color(0, 11, BLUE, BLACK, "Royal Flush");
        ctx.print_color(0, 12, BLUE, BLACK, "Straight Flush");
        ctx.print_color(0, 13, BLUE, BLACK, "Four of a Kind");
        ctx.print_color(0, 14, BLUE, BLACK, "Full House ");
        ctx.print_color(0, 15, BLUE, BLACK, "Flush");
        ctx.print_color(0, 16, BLUE, BLACK, "Straight");
        ctx.print_color(0, 17, BLUE, BLACK, "Tree of a Kind");
        ctx.print_color(0, 18, BLUE, BLACK, "Two Pair");
        ctx.print_color(0, 19, BLUE, BLACK, "Pair of Aces ");

        ctx.print_color(28, 11, BLUE, BLACK, "Wins:");
        ctx.print_color(28, 13, BLUE, BLACK, "Credits:");
        ctx.print_color(28, 15, BLUE, BLACK, "Bets:");
    }
    pub fn refresh(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_PANEL_DYNAMIC);
        ctx.cls();

        ctx.print_color_centered(9, WHITE, BLACK, self.prize.to_string());
        ctx.print_color(21, 11, WHITE, BLACK, (self.value * 500).to_string());
        ctx.print_color(21, 12, WHITE, BLACK, (self.value * 80).to_string());
        ctx.print_color(21, 13, WHITE, BLACK, (self.value * 25).to_string());
        ctx.print_color(21, 14, WHITE, BLACK, (self.value * 10).to_string());
        ctx.print_color(21, 15, WHITE, BLACK, (self.value * 8).to_string());
        ctx.print_color(21, 16, WHITE, BLACK, (self.value * 5).to_string());
        ctx.print_color(21, 17, WHITE, BLACK, (self.value * 3).to_string());
        ctx.print_color(21, 18, WHITE, BLACK, (self.value * 2).to_string());
        ctx.print_color(21, 19, WHITE, BLACK, self.value.to_string());

        ctx.print_color(42, 11, WHITE, BLACK, self.wins.to_string());
        ctx.print_color(42, 13, WHITE, BLACK, self.credits.to_string());
        ctx.print_color(42, 15, WHITE, BLACK, self.bets.to_string());
        ctx.print_color(28, 17, RED, BLACK, self.msg.to_string());
    }
}
