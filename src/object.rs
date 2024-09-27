use {
    super::{action::Action, id::Id, property::Property, world::World},
    ggez::Context,
    hashbrown::HashMap,
};

pub struct Object {
    properties: HashMap<u8, Box<dyn Property>>,
    tick: fn(&mut Object, World, &mut Context, &mut Option<Action>),
    id: u8,
}

impl Object {
    pub fn new<const N: usize>(
        marker: impl Id,
        props: [(u8, Box<dyn Property>); N],
        tick: fn(&mut Object, World, &mut Context, &mut Option<Action>),
    ) -> Self {
        let mut obj = Self {
            properties: HashMap::default(),
            tick,
            id: marker.m_id(),
        };
        obj.attach_many(props);
        obj
    }

    pub fn tick(&mut self, others: World, ctx: &mut Context, action: &mut Option<Action>) {
        (self.tick)(self, others, ctx, action);
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn get_single_ref<T: Property + Id>(&self) -> Option<&T> {
        self.properties
            .get(&T::id())
            .and_then(|prop| prop.as_any_ref().downcast_ref())
    }

    pub fn get_single_mut<T: Property + Id>(&mut self) -> Option<&mut T> {
        self.properties
            .get_mut(&T::id())
            .and_then(|prop| prop.as_any_mut().downcast_mut())
    }

    pub fn get_many_ref<const N: usize>(
        &self,
        prop_ids: [u8; N],
    ) -> [Option<&Box<dyn Property>>; N] {
        let mut props = [None; N];
        for i in 0..N {
            props[i] = self.properties.get(&prop_ids[i]);
        }
        props
    }

    pub fn get_many_mut<const N: usize>(
        &mut self,
        prop_ids: [&u8; N],
    ) -> Option<[&mut Box<dyn Property>; N]> {
        self.properties.get_many_mut(prop_ids)
    }

    pub fn attach_single(&mut self, prop: impl Property + Id) {
        self.properties.insert(prop.m_id(), Box::new(prop));
    }

    pub fn attach_many<const N: usize>(&mut self, props: [(u8, Box<dyn Property>); N]) {
        for prop in props {
            self.properties.insert(prop.0, prop.1);
        }
    }

    pub fn detach_single<T: Property + Id>(&mut self) -> Option<Box<dyn Property>> {
        self.properties.remove(&T::id())
    }

    pub fn detach_many<const N: usize>(
        &mut self,
        prop_ids: [u8; N],
    ) -> [Option<Box<dyn Property>>; N] {
        prop_ids.map(|prop_id| self.properties.remove(&prop_id))
    }

    pub fn has_single<T: Property + Id>(&self) -> bool {
        self.properties.contains_key(&T::id())
    }

    pub fn has_many<const N: usize>(&self, prop_ids: [u8; N]) -> bool {
        prop_ids
            .iter()
            .all(|prop_id| self.properties.contains_key(prop_id))
    }
}
