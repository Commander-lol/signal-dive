use bevy::color::{Color, Srgba};

pub struct SystemColours;
impl SystemColours {
    /// Murky Blue <div style="background-color:#094752;height:20px;"></div>
    pub const MURKY_BLUE: Color = Color::Srgba(Srgba::rgb(0.035, 0.278, 0.322));
}