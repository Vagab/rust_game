mod r_drop;

use r_drop::RDrop;
use ggez::event::{EventHandler, KeyMods};
use ggez::{Context, GameResult};
use ggez::graphics::{clear, present, DrawMode, FillOptions, Rect, BLACK, draw, DrawParam, MeshBuilder, Color};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::f32;
use ggez::input::keyboard::KeyCode;
use crate::game::character::gravity_affected::GravityAffected;

pub struct RainMaker {
    rng: ThreadRng,

    drops: Vec<RDrop>,
    step: f32,
}

impl EventHandler for RainMaker {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for d in &mut self.drops {
            d.fall(self.step)
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, BLACK);

        let mut builder = MeshBuilder::new();
        for d in &self.drops {
            let (w, h) = d.get_wh();
            builder.rectangle(
                DrawMode::Fill(FillOptions::DEFAULT),
                Rect { x: d.x, y: d.y, w, h },
                Color::new(1., 1., 1., 0.05),
            );
        }

        let mesh = builder.build(ctx)?;
        draw(ctx, &mesh, DrawParam::default())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            KeyCode::Down => self.step += 5.,
            KeyCode::Up => self.step -= 5.,
            _ => (),
        }
    }
}

impl RainMaker {
    // drops are evenly spread across layers
    pub fn new(n_drops: usize) -> Self {
        let mut rng = thread_rng();
        let drops = (0..n_drops)
            .map(|_| RDrop::new(&mut rng))
            .collect();

        Self {
            rng,
            drops,
            step: 10.,
        }
    }
}
