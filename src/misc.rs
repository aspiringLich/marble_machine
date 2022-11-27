pub mod marker {
    use bevy::prelude::Component;

    /// marks marble input sensors
    #[derive(Component)]
    pub struct InputSensor;
    /// marks marble output colliders
    #[derive(Component)]
    pub struct OutputSensor;
}
