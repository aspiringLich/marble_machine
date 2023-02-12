use crate::*;

pub enum ZOrder {
    Background,
    Tracer,
    Marble,
    InputComponent,
    OutputComponent,
    BodyComponent,
    IndicatorComponent,
    Border,
    Interactive,
    HoverIndicator = 100,
}

impl ZOrder {
    pub fn f32(self) -> f32 {
        self as u32 as f32
    }
}

impl std::ops::Add<Vec3> for ZOrder {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + Vec3::Z * self.f32()
    }
}

impl std::ops::Add<ZOrder> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: ZOrder) -> Self::Output {
        self + Vec3::Z * rhs.f32()
    }
}
