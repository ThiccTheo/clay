use {
    ggez::{
        event::{self, EventHandler, EventLoop},
        Context, ContextBuilder,
    },
    std::{
        collections::VecDeque,
        fmt::{Debug, Formatter, Result as FormatResult},
    },
};

type State = Box<dyn EventHandler<Action>>;

enum Action {
    Create(State),
    Destroy,
    Change(State),
}

impl Debug for Action {
    fn fmt(&self, _fmtter: &mut Formatter<'_>) -> FormatResult {
        todo!()
    }
}

struct App {
    states: Vec<State>,
    actions: VecDeque<Action>,
}

impl App {
    pub fn new(initial_state: State) -> Self {
        Self {
            states: vec![initial_state],
            actions: VecDeque::default(),
        }
    }

    pub fn run(self, cfg: (Context, EventLoop<()>)) {
        event::run(cfg.0, cfg.1, self);
    }

    fn refresh(&mut self) {
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::Create(state) => self.states.push(state),
                Action::Destroy => {
                    self.states.pop();
                }
                Action::Change(state) => {
                    self.states.pop();
                    self.states.push(state);
                }
            }
        }
    }
}

impl EventHandler<()> for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ()> {
        self.refresh();
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
            .map_err(|action| {
                self.actions.push_back(action);
            })
    }
}

fn main() {
    App::new(todo!()).run(ContextBuilder::new("plug", "cuh").build().unwrap());
}
