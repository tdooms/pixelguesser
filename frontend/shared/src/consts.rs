// ------------- HOST VARIABLES -------------
pub mod host {
    // The time before starting a round when the round info is shown (in secs)
    pub const INFO_DURATION: u32 = 3;
}

// ---------- PIXELATION VARIABLES ----------
pub mod pixelation {
    // The time it takes before a higher quality image is drawn (in millis) when playing
    pub const PLAY_UPSCALE: f64 = 3000.0;
    // The time it takes before a higher quality image is drawn (in millis) when revealing
    pub const REVEAL_UPSCALE: f64 = 500.0;
    // The amount by which the upscale time decreases every pixel when playing
    pub const PLAY_SPEED: f64 = 1.05;
    // The amount by which the upscale time decreases every pixel when revealing
    pub const REVEAL_SPEED: f64 = 1.6;
    // The amount of pixels (y-axis) are used when starting the pixelate
    pub const START_PIXELS: u32 = 4;
    // The shortest duration between pixel updates before larger steps are taken (in millis)
    pub const MAX_REFRESH: f64 = 1000.0 / 20.0;
}
