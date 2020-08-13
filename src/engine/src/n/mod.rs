pub struct N {
    name: String,
    lazyness: f32,
}

impl N {
    pub fn new(name: String) -> N {
        N {
            name: name,
            lazyness: 0.0,
        }
    }
}
