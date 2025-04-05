use bevy::color::{Color, Srgba};

pub struct SystemColours;
impl SystemColours {
	/// Black <div style="background-color:#1b1221;height:20px;"></div>
	pub const BLACK: Color = Color::Srgba(Srgba::rgb(0.078, 0.047, 0.11));
	/// Murky Blue <div style="background-color:#094752;height:20px;"></div>
	pub const MURKY_BLUE: Color = Color::Srgba(Srgba::rgb(0.035, 0.278, 0.322));
}
