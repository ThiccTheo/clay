use {
    super::{action::Action, state::State},
    ggez::{
        event::{self, EventHandler, EventLoop},
        graphics::{Canvas, Color, DrawParam},
        Context, GameResult,
    },
    std::{collections::VecDeque, iter::FromIterator},
};

/// Manages the app states.
///
/// Schedules update and draw calling.
pub struct App {
    states: Vec<Box<dyn State>>,
    actions: VecDeque<Action>,
}

impl App {
    /// Creates a new `App`.
    ///
    /// * `initial_state` - First state to be run when application starts.
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            actions: VecDeque::from_iter([Action::Create(initial_state)]),
            states: Vec::default(),
        }
    }

    /// Runs the `App`.
    ///
    /// Must be called after `App` creation, otherwise, the application
    /// window will not spawn.
    ///
    /// * `cfg` - Necessary configuration objects for app to run and tasks to be scheduled.
    /// Can be obtained in many ways.
    ///
    /// # Example
    /// ```
    /// fn main() {
    ///     App::new(Box::new(Playing::default())).run(
    ///         ContextBuilder::new("", "")
    ///             .window_setup(WindowSetup::default().title("Hi Mom!"))
    ///             .window_mode(WindowMode::default().dimensions(1280., 720.))
    ///             .build()
    ///             .unwrap()
    ///     );
    /// }
    /// ```
    pub fn run(self, cfg: (Context, EventLoop<()>)) {
        event::run(cfg.0, cfg.1, self);
    }

    fn refresh(&mut self, ctx: &mut Context) {
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::Create(state) => {
                    self.states.push(state);
                    self.states.last_mut().unwrap().enter(ctx);
                }
                Action::Destroy => {
                    self.states.pop().unwrap().exit(ctx);
                }
                Action::Change(state) => {
                    self.states.pop().unwrap().exit(ctx);
                    self.states.push(state);
                    self.states.last_mut().unwrap().enter(ctx);
                }
            }
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.refresh(ctx);
        let mut action = None;
        let cur_state = self.states.last_mut().unwrap();
        for i in 0..cur_state.objects().len() {
            let (before, tmp) = cur_state.objects().split_at_mut(i);
            let (this, after) = tmp.split_first_mut().unwrap();
            let others = before.iter_mut().chain(after.iter_mut());
            this.tick(others, ctx, &mut action);
        }
        cur_state.objects().retain(|obj| obj.is_active());
        if let Some(action) = action {
            self.actions.push_back(action);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let cur_state = self.states.last_mut().unwrap();
        let (objs, batches) = cur_state.package();
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        for obj in objs.iter().filter(|obj| obj.is_visible()) {
            let Some(batch) = batches.get_mut(&obj.id()) else {
                continue;
            };
            batch.push(
                DrawParam {
                    src: batch.uv_rect(obj.sprite_sheet_index()),
                    transform: obj.transform().unwrap_or_default(),
                    z: obj.id().0.into(),
                    ..Default::default()
                }
                .dest(ggez::glam::Vec2::splat(10.)),
            );
        }
        batches.values_mut().for_each(|batch| {
            canvas.draw(batch.instance_arr(), DrawParam::default());
            batch.clear()
        });
        canvas.finish(ctx)
    }
}
