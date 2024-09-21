use {
    super::{action::Action, state::State},
    ggez::{
        event::{self, EventHandler, EventLoop},
        Context,
    },
    std::collections::VecDeque,
};

pub struct App {
    states: Vec<Box<dyn State>>,
    actions: VecDeque<Action>,
}

impl App {
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            states: vec![initial_state],
            actions: VecDeque::default(),
        }
    }

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
