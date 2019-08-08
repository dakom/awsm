pub struct Light {
    pub color: [f64;4],
    pub intensity: f64,
    pub settings: Option<LightSettings>,
}

pub enum LightSettings {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

pub struct DirectionalLight {
    direction: Option<f64>
}

pub struct PointLight {
    range: Option<f64>
}

pub struct SpotLight {
    direction: Option<f64>,
    angle_scale: f64,
    angle_offset: f64,
    range: f64
}

