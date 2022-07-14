// ------------- HOST VARIABLES -------------
// The time before starting a round when the round info is shown (in secs)
pub const HOST_INFO_DURATION: u32 = 3;

// ---------- PIXELATION VARIABLES ----------
// The time it takes before a higher quality image is drawn (in millis)
pub const PIXELATE_UPSCALE_TIME: f64 = 3000.0;
// The amount of pixels (y-axis) are used when starting the pixelate
pub const PIXELATE_START_PIXELS: u32 = 4;
// The amount by which the upscale time decreases every pixel when playing
pub const PIXELATE_PLAY_SPEED: f64 = 1.05;
// The amount by which the upscale time decreases every pixel when revealing
pub const PIXELATE_REVEAL_SPEED: f64 = 1.2;
// The shortest duration between pixel updates before larger steps are taken (in millis)
pub const PIXELATE_MAX_REFRESH: f64 = 1000.0 / 60.0;
