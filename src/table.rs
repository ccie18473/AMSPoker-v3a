use crate::prelude::*;

const CARD_DECK_POS: usize = 0;
const CARD1_POS: usize = 1;
const CARD2_POS: usize = 2;
const CARD3_POS: usize = 3;
const CARD4_POS: usize = 4;
const CARD5_POS: usize = 5;
//const CARD_DOUBLE: usize = 6;
const BET_BTN_POS: usize = 21;
const BET_MAX_BTN_POS: usize = 22;
const RETRY_BTN_POS: usize = 23;
const CREDIT_BTN_POS: usize = 24;
const DOUBLE_BTN_POS: usize = 25;
const RED_BTN_POS: usize = 26;
const BLACK_BTN_POS: usize = 27;

pub struct TTable {
    pub game: TGame,
}

impl TTable {
    pub fn new_table() -> Self {
        Self {
            game: TGame::new_game(),
        }
    }
    pub fn render(ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_TABLE);
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                ctx.set(x, y, GREEN, BLACK, x + (y * SCREEN_WIDTH));
            }
        }
    }
    pub fn table_pos(&mut self, x: i32, y: i32) -> usize {
        (x + (y * SCREEN_WIDTH)) as usize
    }
    pub fn process_mouse(&mut self, ctx: &mut BTerm, pos: Point) {
        if self.table_pos(pos.x, pos.y) == CARD_DECK_POS {
            if self.game.state() == 1 && self.game.panel.bets > 0 {
                self.game.draw_five();
                self.game.check_prizes();
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == CARD1_POS {
            if self.game.state() == 2 {
                TCard::card_selected(&mut self.game.card1);
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == CARD2_POS {
            if self.game.state() == 2 {
                TCard::card_selected(&mut self.game.card2);
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == CARD3_POS {
            if self.game.state() == 2 {
                TCard::card_selected(&mut self.game.card3);
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == CARD4_POS {
            if self.game.state() == 2 {
                TCard::card_selected(&mut self.game.card4);
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == CARD5_POS {
            if self.game.state() == 2 {
                TCard::card_selected(&mut self.game.card5);
                self.game.render(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == BET_BTN_POS {
            if self.game.state() == 1 {
                // State 1
                if self.game.panel.bets < 10 {
                    TButton::blink(ctx, pos.x, pos.y);
                    self.game.panel.credits -= 1;
                    self.game.panel.bets += 1;
                    self.game.panel.value += 1;
                    self.game.panel.refresh(ctx);
                }
            }
        } else if self.table_pos(pos.x, pos.y) == BET_MAX_BTN_POS {
            if self.game.state() == 1 {
                // State 1
                if self.game.panel.bets < 10 {
                    TButton::blink(ctx, pos.x, pos.y);
                    self.game.panel.credits = self.game.panel.credits - (10 - self.game.panel.bets);
                    self.game.panel.bets = self.game.panel.bets + (10 - self.game.panel.bets);
                    self.game.panel.value = 10;
                    self.game.panel.refresh(ctx);
                }
            }
        } else if self.table_pos(pos.x, pos.y) == RETRY_BTN_POS {
            if self.game.state() == 2 {
                // State 2
                TButton::blink(ctx, pos.x, pos.y);
                self.game.draw_again();
                self.game.check_prizes();
                self.game.render(ctx);
                if self.game.flag3 {
                } else {
                    self.game.end_of_play();
                    self.game.panel.refresh(ctx);
                }
            }
        } else if self.table_pos(pos.x, pos.y) == CREDIT_BTN_POS {
            if self.game.state() == 3 || self.game.state() == 4 {
                // State 3 or State 4
                TButton::blink(ctx, pos.x, pos.y);
                self.game.panel.credits = self.game.panel.credits + self.game.panel.wins;
                self.game.panel.msg = MSG1.to_string();
                self.game.end_of_play();
                self.game.panel.refresh(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == DOUBLE_BTN_POS {
            // State 3
            if self.game.state() == 3 {
                // Move to State 4
                TButton::blink(ctx, pos.x, pos.y);
                self.game.flag4 = true;
                self.game.panel.msg = MSG3.to_string();
                self.game.panel.refresh(ctx);
            }
        } else if self.table_pos(pos.x, pos.y) == RED_BTN_POS {
            // State 4
            if self.game.state() == 4 {
                TButton::blink(ctx, pos.x, pos.y);
                self.game.card6 = self.game.draw_card();
                if self.game.card6 == TCard::default() {
                    return;
                }
                self.game.render_double(ctx);
                if self.game.card6.color() == SUIT_COLOR_RED {
                    self.game.panel.wins = 2 * self.game.panel.wins;
                    self.game.panel.msg = MSG5.to_string();
                    self.game.panel.refresh(ctx);
                } else {
                    self.game.end_of_play();
                    self.game.panel.refresh(ctx);
                }
            }
        } else if self.table_pos(pos.x, pos.y) == BLACK_BTN_POS {
            // State 4
            if self.game.state() == 4 {
                TButton::blink(ctx, pos.x, pos.y);
                self.game.card6 = self.game.draw_card();
                if self.game.card6 == TCard::default() {
                    return;
                }
                self.game.render_double(ctx);
                if self.game.card6.color() == SUIT_COLOR_BLACK {
                    self.game.panel.wins = 2 * self.game.panel.wins;
                    self.game.panel.msg = MSG5.to_string();
                    self.game.panel.refresh(ctx);
                } else {
                    self.game.end_of_play();
                    self.game.panel.refresh(ctx);
                }
            }
        } else {
            println!("Tapped Table");
        }
    }
}
