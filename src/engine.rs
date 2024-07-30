use crate::display::{
    self,
    print_back_boundary,
    print_back_catch,
    print_back_four,
    print_front_boundary,
    print_front_shot,
    print_front_tip,
    print_init_back,
    print_init_front_shot,
    print_init_left_four,
    print_left_boundary,
    print_left_four,
    print_left_tip,
    print_init_right_four,
    print_right_boundary,
    print_right_four,
    print_right_tip,
};
use crate::bowler::{ BowlType, Bowler, BowlerType };
use crate::batter::{ Batter, ShotPower, ShotType };
use crate::utils::{ move_cursor, random_num };

pub struct Engine {
    pub game_end: bool,
    pub score: u32,
}
impl Engine {
    pub fn new(game_end: bool, score: u32) -> Self {
        Engine {
            game_end,
            score,
        }
    }

    fn left_spin_left_shot_high(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (1..=40).contains(&rng) {
                //40%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (41..=85).contains(&rng) {
                //45%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = random_num(1, 4) == 1;
                display::print_left_four(block); //20%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //15%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=30).contains(&rng) {
                //35%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=85).contains(&rng) {
                //55%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = (1..3).contains(&random_num(1, 9));
                display::print_left_four(block); //30%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //15%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        }
    }
    fn left_spin_left_shot_med(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (0..=25).contains(&rng) {
                //25%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=90).contains(&rng) {
                //65%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = random_num(1, 2) == 1;
                display::print_left_four(block); //50%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (91..=95).contains(&rng) {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    //80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //5%
                print_left_tip();
                self.score += random_num(0, 1);
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=20).contains(&rng) {
                //20%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (21..=85).contains(&rng) {
                //65%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = (1..=3).contains(&random_num(1, 5));
                display::print_left_four(block); //60%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (86..=90).contains(&rng) {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    // 80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //10%
                print_left_tip();
                self.score += random_num(0, 1);
            }
        }
    }
    fn left_spin_left_shot_low(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 10);
            if (1..=7).contains(&rng) {
                //70%
                print_left_tip();
            } else {
                //30%
                print_init_left_four();
                print_left_boundary();
                print_left_four(true); //100%
            }
        } else {
            let rng = random_num(1, 10);
            if (1..=8).contains(&rng) {
                //80%
                print_left_tip();
                self.score += random_num(0, 1);
            } else {
                //20%
                print_init_left_four();
                print_left_boundary();
                print_left_four(true); //100%
                self.score += random_num(1, 3);
            }
        }
    }

    fn right_spin_right_shot_high(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (1..=40).contains(&rng) {
                //40%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (41..=85).contains(&rng) {
                //45%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = random_num(1, 4) == 1;
                display::print_right_four(block); //20%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //15%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=30).contains(&rng) {
                //35%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=85).contains(&rng) {
                //55%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = (1..3).contains(&random_num(1, 9));
                display::print_right_four(block); //30%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //15%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        }
    }
    fn right_spin_right_shot_med(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (0..=25).contains(&rng) {
                //25%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=90).contains(&rng) {
                //65%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = random_num(1, 2) == 1;
                display::print_right_four(block); //50%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (91..=95).contains(&rng) {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    //80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //5%
                display::print_right_tip();
                self.score += random_num(0, 1);
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=20).contains(&rng) {
                //20%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (21..=85).contains(&rng) {
                //65%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = (1..=3).contains(&random_num(1, 5));
                display::print_right_four(block); //60%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (86..=90).contains(&rng) {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    // 80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //10%
                display::print_right_tip();
                self.score += random_num(0, 1);
            }
        }
    }
    fn right_spin_right_shot_low(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 10);
            if (1..=7).contains(&rng) {
                //70%
                display::print_right_tip();
            } else {
                //30%
                display::print_init_right_four();
                display::print_right_boundary();
                display::print_right_four(true); //100%
            }
        } else {
            let rng = random_num(1, 10);
            if (1..=8).contains(&rng) {
                //80%
                display::print_right_tip();
                self.score += random_num(0, 1);
            } else {
                //20%
                display::print_init_right_four();
                display::print_right_boundary();
                display::print_right_four(true); //100%
                self.score += random_num(1, 3);
            }
        }
    }

    fn straight_spin_straight_shot_high(&mut self, speed: u64) {
        if speed < 75 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=35).contains(&rng) {
                //35%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = random_num(1, 2) == 1;
                display::print_front_shot(four); //50%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (36..=90).contains(&rng) {
                //55%
                display::print_front_tip();
            } else {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 10) == 1 {
                    //10%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //90%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=25).contains(&rng) {
                //25%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = random_num(1, 3) == 1;
                display::print_front_shot(four); //33.3%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (26..=90).contains(&rng) {
                //65%
                display::print_front_tip();
            } else {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 10) == 1 {
                    //10%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //90%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        }
    }
    fn straight_spin_straight_shot_med(&mut self, speed: u64) {
        if speed < 75 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=25).contains(&rng) {
                //25%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = random_num(1, 5) == 1;
                display::print_front_shot(four); //20%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (26..=95).contains(&rng) {
                //%70%
                display::print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 10) == 1 {
                    //10%
                    display::print_back_four();
                } else {
                    //90%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=20).contains(&rng) {
                //20%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = random_num(1, 6) == 1;
                display::print_front_shot(four); //16.6%
                if !four {
                    self.score += random_num(1, 2);
                }
            } else if (21..=95).contains(&rng) {
                //75%
                display::print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 10) == 1 {
                    //10%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //90%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        }
    }
    fn straight_spin_straight_shot_low(&mut self, speed: u64) {
        if speed < 75 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=10).contains(&rng) {
                //10%
                print_init_front_shot();
                print_front_boundary();
                print_front_shot(false); //0%
                self.score += random_num(1, 2);
            } else if (11..95).contains(&rng) {
                //85%
                print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 20) == 1 {
                    //5%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //95%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=10).contains(&rng) {
                //10%
                print_init_front_shot();
                print_front_boundary();
                print_front_shot(false); //0%
                self.score += random_num(1, 2);
            } else {
                //90%
                print_front_tip();
            } //No edge
        }
    }

    fn left_fast_left_shot_high(&mut self, speed: u64) {
        if speed < 31 {
            let rng = random_num(1, 100);
            if (1..=65).contains(&rng) {
                //65%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = random_num(1, 2) == 1;
                display::print_left_six(catch); //50%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (66..=75).contains(&rng) {
                //10%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = random_num(1, 4) == 1;
                display::print_left_four(block); //20%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //25%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3% of dropping
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=55).contains(&rng) {
                //55%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = random_num(1, 2) == 1;
                display::print_left_six(catch); //50%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (56..=75).contains(&rng) {
                //20%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = (1..3).contains(&random_num(1, 9));
                display::print_left_four(block); //30%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //25%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3% of dropping
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        }
    }
    fn left_fast_left_shot_med(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (0..=40).contains(&rng) {
                //40%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (41..=85).contains(&rng) {
                //45%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = random_num(1, 3) == 1;
                display::print_left_four(block); //33.3%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (86..=95).contains(&rng) {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 5)) {
                    //60%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //40%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //5%
                print_left_tip();
                self.score += random_num(0, 1);
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=30).contains(&rng) {
                //30%
                display::print_init_left_six();
                display::print_left_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=80).contains(&rng) {
                //50%
                display::print_init_left_four();
                display::print_left_boundary();
                let block: bool = (1..=2).contains(&random_num(1, 5));
                display::print_left_four(block); //40%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (81..=90).contains(&rng) {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    // 80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //10%
                print_left_tip();
                self.score += random_num(0, 1);
            }
        }
    }
    fn left_fast_left_shot_low(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 10);
            if (1..=7).contains(&rng) {
                //70%
                print_left_tip();
            } else {
                //30%
                print_init_left_four();
                print_left_boundary();
                print_left_four((1..=4).contains(&random_num(1, 5))); //80% of blocking
            }
        } else {
            let rng = random_num(1, 10);
            if (1..=8).contains(&rng) {
                //80%
                print_left_tip();
                self.score += random_num(0, 1);
            } else {
                //20%
                print_init_left_four();
                print_left_boundary();
                print_left_four((1..=9).contains(&random_num(1, 10))); //90% of blocking
                self.score += random_num(1, 3);
            }
        }
    }

    fn right_fast_right_shot_high(&mut self, speed: u64) {
        if speed < 31 {
            let rng = random_num(1, 100);
            if (1..=65).contains(&rng) {
                //65%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = random_num(1, 2) == 1;
                display::print_right_six(catch); //50%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (66..=75).contains(&rng) {
                //10%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = random_num(1, 4) == 1;
                display::print_right_four(block); //20%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //25%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3% of dropping
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=55).contains(&rng) {
                //55%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = random_num(1, 2) == 1;
                display::print_right_six(catch); //50%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (56..=75).contains(&rng) {
                //20%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = (1..3).contains(&random_num(1, 9));
                display::print_right_four(block); //30%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else {
                //25%
                display::print_init_back();
                display::print_back_boundary();
                if (1..2).contains(&random_num(1, 3)) {
                    //66.6%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3% of dropping
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    print_back_four();
                    self.score += 4;
                }
            }
        }
    }
    fn right_fast_right_shot_med(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 100);
            if (0..=40).contains(&rng) {
                //40%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (41..=85).contains(&rng) {
                //45%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = random_num(1, 3) == 1;
                display::print_right_four(block); //33.3%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (86..=95).contains(&rng) {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 5)) {
                    //60%
                    let drop: bool = random_num(1, 3) == 1;
                    print_back_catch(drop); //33.3%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //40%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //5%
                print_right_tip();
                self.score += random_num(0, 1);
            }
        } else {
            let rng = random_num(1, 100);
            if (0..=30).contains(&rng) {
                //30%
                display::print_init_right_six();
                display::print_right_boundary();
                let catch: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_six(catch); //40%
                if catch {
                    self.game_end = true;
                } else {
                    self.score += 6;
                }
            } else if (31..=80).contains(&rng) {
                //50%
                display::print_init_right_four();
                display::print_right_boundary();
                let block: bool = (1..=2).contains(&random_num(1, 5));
                display::print_right_four(block); //40%
                if block {
                    self.score += random_num(1, 3);
                } else {
                    self.score += 4;
                }
            } else if (81..=90).contains(&rng) {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if (1..=3).contains(&random_num(1, 4)) {
                    // 80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); //50%
                    if !drop {
                        self.game_end = true;
                    }
                } else {
                    //20%
                    print_back_four();
                    self.score += 4;
                }
            } else {
                //10%
                print_right_tip();
                self.score += random_num(0, 1);
            }
        }
    }
    fn right_fast_right_shot_low(&mut self, speed: u64) {
        if speed < 75 {
            let rng = random_num(1, 10);
            if (1..=7).contains(&rng) {
                //70%
                print_right_tip();
            } else {
                //30%
                print_init_right_four();
                print_right_boundary();
                print_right_four((1..=4).contains(&random_num(1, 5))); //80% of blocking
            }
        } else {
            let rng = random_num(1, 10);
            if (1..=8).contains(&rng) {
                //80%
                print_right_tip();
                self.score += random_num(0, 1);
            } else {
                //20%
                print_init_right_four();
                print_right_boundary();
                print_right_four((1..=9).contains(&random_num(1, 10))); //90% of blocking
                self.score += random_num(1, 3);
            }
        }
    }

    fn straight_fast_straight_shot_high(&mut self, speed: u64) {
        if speed < 31 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=55).contains(&rng) {
                //55%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = (1..=4).contains(&random_num(1, 5));
                display::print_front_shot(four); //80%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (56..=90).contains(&rng) {
                //35%
                display::print_front_tip();
            } else {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 3) == 1 {
                    //33.3%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //66.6%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=45).contains(&rng) {
                //45%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = (1..=3).contains(&random_num(1, 5));
                display::print_front_shot(four); //60%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (46..=90).contains(&rng) {
                //45%
                display::print_front_tip();
            } else {
                //10%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 3) == 1 {
                    //33.3%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //66.6%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        }
    }
    fn straight_fast_straight_shot_med(&mut self, speed: u64) {
        if speed < 31 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=35).contains(&rng) {
                //35%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = (1..=2).contains(&random_num(1, 5));
                display::print_front_shot(four); //40%
                if !four {
                    self.score += random_num(1, 2);
                } else {
                    self.score += 4;
                }
            } else if (36..=95).contains(&rng) {
                //60%
                display::print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 5) == 1 {
                    //20%
                    display::print_back_four();
                } else {
                    //80%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=30).contains(&rng) {
                //30%
                display::print_init_front_shot();
                display::print_front_boundary();
                let four: bool = random_num(1, 5) == 1;
                display::print_front_shot(four); //20%
                if !four {
                    self.score += random_num(1, 2);
                }
            } else if (31..=95).contains(&rng) {
                //65%
                display::print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 6) == 1 {
                    //16.6%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //100 - 16.6% i can't do maths...
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        }
    }
    fn straight_fast_straight_shot_low(&mut self, speed: u64) {
        if speed < 31 {
            let rng: u64 = random_num(1, 100).into();
            if (1..=20).contains(&rng) {
                //20%
                print_init_front_shot();
                print_front_boundary();
                print_front_shot(random_num(1, 5) == 1); //50%
                self.score += random_num(1, 2);
            } else if (21..95).contains(&rng) {
                //75%
                print_front_tip();
            } else {
                //5%
                display::print_init_back();
                display::print_back_boundary();
                if random_num(1, 10) == 1 {
                    //10%
                    display::print_back_four();
                    self.score += 4;
                } else {
                    //90%
                    let drop: bool = random_num(1, 2) == 1;
                    print_back_catch(drop); // 50%
                    if !drop {
                        self.game_end = true;
                    }
                }
            }
        } else {
            let rng: u64 = random_num(1, 100).into();
            if (1..=15).contains(&rng) {
                //15%
                print_init_front_shot();
                print_front_boundary();
                print_front_shot(random_num(1, 20) == 1); //5%
                self.score += random_num(1, 2);
            } else {
                //85%
                print_front_tip();
            } //No edge
        }
    }

    pub fn run(&mut self, bowler: &Bowler, batter: &Batter, speed: u64) {
        if bowler.bowler_type == BowlerType::Spin {
            if bowler.bowl_type == BowlType::Left {
                //Left Spin
                if batter.shot == ShotType::Left {
                    // Left Shot
                    if batter.power == ShotPower::High {
                        self.left_spin_left_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.left_spin_left_shot_med(speed);
                    } else {
                        self.left_spin_left_shot_low(speed);
                    }
                } else if batter.shot == ShotType::Right {
                    let rng = random_num(1, 100);
                    if (1..=20).contains(&rng) {
                        //20%
                        self.left_spin_left_shot_high(speed);
                    } else if (21..=60).contains(&rng) {
                        //40%
                        print_left_tip();
                        self.score += 1;
                    } else if (61..=70).contains(&rng) {
                        //10%
                        print_init_back();
                        print_back_boundary();
                        let drop: bool = random_num(1, 2) == 1;
                        print_back_catch(drop); //50%
                        if !drop {
                            self.game_end = true;
                        }
                    }
                    //else Miss
                } //No edge on front shot if ball is left *spin*...
            } else if bowler.bowl_type == BowlType::Right {
                //Right Spin
                if batter.shot == ShotType::Right {
                    //Right Shot
                    if batter.power == ShotPower::High {
                        self.right_spin_right_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.right_spin_right_shot_med(speed);
                    } else {
                        self.right_spin_right_shot_low(speed);
                    }
                } else if batter.shot == ShotType::Left {
                    let rng = random_num(1, 100);
                    if (1..=20).contains(&rng) {
                        //20%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        self.right_spin_right_shot_high(speed);
                    } else if (21..=60).contains(&rng) {
                        //40%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        display::print_right_tip();
                    } else if (61..=70).contains(&rng) {
                        //10%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        display::print_init_back();
                        display::print_back_boundary();
                        let drop: bool = random_num(1, 2) == 1;
                        display::print_back_catch(drop); //50%
                        if !drop {
                            self.game_end = true;
                        }
                    }
                } //No edge on front shot if ball is right ...
            } else {
                //Straight Spin
                if batter.shot == ShotType::Straight {
                    //Straight Shot
                    if batter.power == ShotPower::High {
                        self.straight_spin_straight_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.straight_spin_straight_shot_med(speed);
                    } else {
                        self.straight_spin_straight_shot_low(speed);
                    }
                } else {
                    if random_num(1, 5) == 1 && batter.shot == ShotType::Left {
                        //20% Edge
                        display::print_straight_out(speed, false);
                        print_init_back();
                        print_back_boundary();
                        if random_num(1, 10) == 1 {
                            //10%
                            print_back_four();
                            self.score += 4;
                        } else {
                            //90%
                            let drop: bool = random_num(1, 3) != 1;
                            print_back_catch(drop); //33.3% chance of out
                            if !drop {
                                self.game_end = true;
                            }
                        }
                    } else {
                        //80%
                        let out: bool = random_num(1, 3) == 1; // 33.3%
                        display::print_straight_out(speed, out);
                        if out {
                            self.game_end = true; //OUT
                        }
                    }
                }
            }
        } else {
            if bowler.bowl_type == BowlType::Left {
                //Left Fast
                if batter.shot == ShotType::Left {
                    // Left Shot
                    if batter.power == ShotPower::High {
                        self.left_fast_left_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.left_fast_left_shot_med(speed);
                    } else {
                        self.left_fast_left_shot_low(speed);
                    }
                } else if batter.shot == ShotType::Right {
                    let rng = random_num(1, 100);
                    if (1..=20).contains(&rng) {
                        //20%
                        self.left_fast_left_shot_high(speed);
                    } else if (21..=60).contains(&rng) {
                        //40%
                        print_left_tip();
                        self.score += 1;
                    } else if (61..=70).contains(&rng) {
                        //10%
                        print_init_back();
                        print_back_boundary();
                        let drop: bool = random_num(1, 2) == 1;
                        print_back_catch(drop); //50%
                        if !drop {
                            self.game_end = true;
                        }
                    }
                    //else Miss
                } else if batter.shot == ShotType::Straight {
                    if random_num(1, 5) == 1 {
                        //20%
                        print_init_back();
                        print_back_boundary();
                        let drop: bool = random_num(1, 2) == 1;
                        print_back_catch(drop); // 50%
                        if !drop {
                            self.game_end = true;
                        }
                    }
                }
            } else if bowler.bowl_type == BowlType::Right {
                //Right fast
                if batter.shot == ShotType::Right {
                    //Right Shot
                    if batter.power == ShotPower::High {
                        self.right_fast_right_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.right_fast_right_shot_med(speed);
                    } else {
                        self.right_fast_right_shot_low(speed);
                    }
                } else if batter.shot == ShotType::Left {
                    let rng = random_num(1, 100);
                    if (1..=20).contains(&rng) {
                        //20%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        self.right_fast_right_shot_high(speed);
                    } else if (21..=60).contains(&rng) {
                        //40%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        display::print_right_tip();
                    } else if (61..=70).contains(&rng) {
                        //10%
                        move_cursor(61, 3);
                        print!(" "); //erase the ball
                        display::print_init_back();
                        display::print_back_boundary();
                        let drop: bool = random_num(1, 2) == 1;
                        display::print_back_catch(drop); //50%
                        if !drop {
                            self.game_end = true;
                        }
                    }
                } // No edge on front shot if ball is right...
            } else {
                //Straight Fast
                if batter.shot == ShotType::Straight {
                    //Straight Shot
                    if batter.power == ShotPower::High {
                        self.straight_fast_straight_shot_high(speed);
                    } else if batter.power == ShotPower::Medium {
                        self.straight_fast_straight_shot_med(speed);
                    } else {
                        self.straight_fast_straight_shot_low(speed);
                    }
                } else {
                    if random_num(1, 5) == 1 && batter.shot == ShotType::Left {
                        //20% Edge
                        display::print_straight_out(speed, false);
                        print_init_back();
                        print_back_boundary();
                        if random_num(1, 10) == 1 {
                            //10%
                            print_back_four();
                            self.score += 4;
                        } else {
                            //90%
                            let drop: bool = random_num(1, 3) != 1;
                            print_back_catch(drop); //33.3% chance of out
                            if !drop {
                                self.game_end = true;
                            }
                        }
                    } else {
                        //80%
                        let out: bool = random_num(1, 3) == 1; // 33.3%
                        display::print_straight_out(speed, out);
                        if out {
                            self.game_end = true; //OUT
                        }
                    }
                }
            }
        }
    }
}
