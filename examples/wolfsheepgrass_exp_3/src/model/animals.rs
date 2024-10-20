use krabmaga::engine::location::Int2D;
use krabmaga::engine::location::Real2D;
// T: TODO check if we can solve this strange error with import
// T: of Component and get rid of this ugly 
// T: and apparently useless import of bevy_ecs
// T: see that for a description of the problem:
// T: https://github.com/bevyengine/bevy/issues/3659
use krabmaga::engine::bevy_ecs as bevy_ecs;
use krabmaga::engine::Component;





// T: TODO check if it is the best way to store data about agents.
// T: probably we can find a more conveniente way to partitionate 
// T: from the multithreading point of view......
#[derive(Component, Copy, Clone)]
pub struct Wolf {
    pub id: u64,
    pub energy: f64,
}

#[derive(Component, Copy, Clone)]
pub struct Sheep {
    pub id: u64,
    pub energy: f64,
}

// T: this component is used to save the last location of the animals
#[derive(Component, Copy, Clone)]
pub struct LastLocation(pub Option<Int2D>);

// T: this component contains the updated location of these agents
#[derive(Component, Copy, Clone)]
pub struct Location(pub Int2D);

// T: this marker component is used to mark all the Agents beetween Entities
// T: TODO verify if it is necessary and starting use it in the case
#[derive(Component, Copy, Clone)]
pub struct Agent {
    pub id: u64,
}