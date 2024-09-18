use bot::Bot;
use commands::{Command, Mlen};
use log::trace;
use mdl::{CellState, Matrix};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Harmonic {
    Low,
    High,
}
#[derive(Debug)]
pub struct State {
    pub bots: Vec<Option<Bot>>,
    pub harmonic: Harmonic,
    pub matrix: Matrix,
    pub energy: i64,
    pub energy_spend_type: HashMap<&'static str, i64>,
    pub current_bot_count: usize,
}

impl State {
    pub fn new(max_bots: u8, matrix: Matrix) -> Self {
        let mut bots: Vec<Option<Bot>> = (0..max_bots).map(|_| None).collect();
        bots[0] = Some(Bot::initial(max_bots));
        Self {
            bots,
            harmonic: Harmonic::Low,
            matrix,
            energy: 0,
            energy_spend_type: HashMap::new(),
            current_bot_count: 1,
        }
    }

    pub fn apply_energy(&mut self, energy_type: &'static str, energy: i64) {
        self.energy += energy;
        self.energy_spend_type
            .entry(energy_type)
            .and_modify(|v| *v += energy)
            .or_insert(energy);
    }

    pub fn end_step(&mut self) {
        let energy = self.matrix.r.pow(3)
            * match self.harmonic {
                Harmonic::Low => 3,
                Harmonic::High => 30,
            };

        self.apply_energy("step", energy as i64);
        self.apply_energy("active_bot", 20 * self.current_bot_count as i64);
    }
}

pub trait StateCommand {
    fn apply(&self, state: &mut State) -> anyhow::Result<()>;

    fn is_bot_command(&self) -> bool;
}

impl StateCommand for Command {
    fn apply(&self, state: &mut State) -> anyhow::Result<()> {
        match self {
            Command::Halt => Ok(()),
            Command::Wait => Ok(()),
            Command::Flip => match state.harmonic {
                Harmonic::Low => {
                    println!("harmonic flip to High");
                    state.harmonic = Harmonic::High;
                    Ok(())
                }
                Harmonic::High => {
                    println!("harmonic flip to Low");
                    state.harmonic = Harmonic::Low;
                    Ok(())
                }
            },
            Command::SMove(m) => {
                trace!("smove {m:?}");
                match &mut state.bots[0] {
                    None => {}
                    Some(bot) => {
                        bot.apply_position_diff(&m.lld);
                        state.apply_energy("smove", 2 * m.lld.mlen() as i64);
                    }
                }

                Ok(())
            }
            Command::LMove(m) => {
                state.apply_energy("lmove", 2 * (m.sld1.mlen() + 2 + m.sld2.mlen()) as i64);
                Ok(())
            }
            Command::FusionP(_) => Ok(()),
            Command::FusionS(_) => Ok(()),
            Command::Fission(_) => Ok(()),
            Command::Fill(fill) => {
                trace!("fill {fill:?}");
                let _ = fill.nd;
                match &state.bots[0] {
                    None => {}
                    Some(bot) => {
                        let place = bot.get_position_by_diff(&fill.nd);
                        state.matrix.set(
                            place.x as usize,
                            place.y as usize,
                            place.z as usize,
                            CellState::Fill,
                        );
                        state.apply_energy("fill", 12); // todo, check if already filled
                    }
                }

                Ok(())
            }
            Command::Void(void) => {
                trace!("void {void:?}");
                let _ = void.nd;
                state.apply_energy("fill", 12); // todo, check if already void
                Ok(())
            }
            Command::GFill(_) => Ok(()),
            Command::GVoid(_) => Ok(()),
        }
    }

    fn is_bot_command(&self) -> bool {
        match self {
            Command::Halt => false,
            Command::Wait => false,
            Command::Flip => false,
            _ => true,
        }
    }
}
