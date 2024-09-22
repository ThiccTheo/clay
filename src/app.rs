use {
    super::{action::Action, state::State},
    ggez::{
        event::{self, EventHandler, EventLoop},
        Context,
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

impl EventHandler<()> for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ()> {
        self.refresh(ctx);
        self.states
            .last_mut()
            .ok_or(())?
            .update(ctx)
            .map_err(|_| ())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
        self.states
            .last_mut()
            .ok_or(())?
            .draw(ctx)
            .map_err(|action| self.actions.push_back(action))
    }
}
