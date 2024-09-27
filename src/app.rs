use {
    super::{
        action::Action, activity::Activity, sprite_sheet_index::SpriteSheetIndex, state::State,
        transform::Transform, visibility::Visibility,
    },
    ggez::{
        event::{self, EventHandler, EventLoop},
        graphics::{Canvas, Color, DrawParam, Rect, Transform as Transformation},
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
        cur_state
            .objects()
            .retain(|obj| obj.has_single::<Activity>());
        if let Some(action) = action {
            self.actions.push_back(action);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let cur_state = self.states.last_mut().unwrap();
        let (objs, batches) = cur_state.package();
        let (win_width, win_height) = ctx.gfx.size();
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgba(250, 235, 215, 255));
        canvas.set_screen_coordinates(Rect::new(
            -win_width / 2.,
            win_height / 2.,
            win_width,
            -win_height,
        ));
        for obj in objs.iter().filter(|obj| obj.has_single::<Visibility>()) {
            let Some(batch) = batches.get_mut(&obj.id()) else {
                continue;
            };
            let xform = obj
                .get_single_ref::<Transform>()
                .cloned()
                .unwrap_or_default();
            batch.push(DrawParam {
                src: batch.uv_rect(
                    obj.get_single_ref::<SpriteSheetIndex>()
                        .map_or(0, |idx| idx.0),
                ),
                transform: Transformation::Values {
                    dest: xform.translation.truncate().into(),
                    rotation: xform.rotation,
                    scale: xform.scale.into(),
                    offset: (batch.sub_img_size() / 2. * xform.scale).into(),
                },
                z: xform.translation.z as i32,
                ..Default::default()
            });
        }
        batches
            .values_mut()
            .filter(|batch| !batch.instance_arr().instances().is_empty())
            .for_each(|batch| {
                canvas.draw(
                    batch.instance_arr(),
                    DrawParam::default().z(batch.instance_arr().instances().first().unwrap().z),
                );
                batch.clear()
            });
        canvas.finish(ctx)
    }
}
