use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::Rng;
use std::time::{Duration, Instant};

use thermal_pinpad::{ring_intensity, ThermalColorMapper, ThermalKeypad, ThermalPalette};

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 480;
const BUTTON_WIDTH: u32 = 70;
const BUTTON_HEIGHT: u32 = 50;
const BUTTON_GAP: u32 = 15;
const KEYPAD_MARGIN_X: u32 = 35;
const KEYPAD_MARGIN_Y: u32 = 100;
const LABELS: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

struct State {
    keypad: ThermalKeypad,
    palette: ThermalPalette,
    color_mapper: ThermalColorMapper,
    start: Instant,
    demo: bool,
    last_demo: Instant,
    status: String,
}

impl State {
    fn new() -> Self {
        Self {
            keypad: ThermalKeypad::new(),
            palette: ThermalPalette::default(),
            color_mapper: ThermalColorMapper::new(),
            start: Instant::now(),
            demo: false,
            last_demo: Instant::now() - Duration::from_secs(10),
            status: "Click buttons or press D for demo".to_string(),
        }
    }

    fn ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    fn rect(&self, row: usize, col: usize) -> Rectangle {
        let x = KEYPAD_MARGIN_X + (col as u32) * (BUTTON_WIDTH + BUTTON_GAP);
        let y = KEYPAD_MARGIN_Y + (row as u32) * (BUTTON_HEIGHT + BUTTON_GAP);
        Rectangle::new(Point::new(x as i32, y as i32), Size::new(BUTTON_WIDTH, BUTTON_HEIGHT))
    }

    fn at_point(&self, p: Point) -> Option<char> {
        for row in 0..4 {
            for col in 0..3 {
                if self.rect(row, col).contains(p) {
                    return Some(LABELS[row][col]);
                }
            }
        }
        None
    }

    fn press(&mut self, label: char) {
        self.keypad.press(label, self.ms());
        self.status = format!("Pressed: {}", label);
    }

    fn random_code(&mut self) {
        let mut rng = rand::thread_rng();
        let now = self.ms();
        let mut used = std::collections::HashSet::new();
        for i in 0..4 {
            let d = loop {
                let n = rng.gen_range(0..10);
                let c = char::from_digit(n, 10).unwrap();
                if used.insert(c) || used.len() >= 10 { break c; }
            };
            self.keypad.press(d, now + (i as u64 * 200));
        }
        self.status = "Demo: Random code entered".to_string();
    }

    fn reset(&mut self) {
        self.keypad.reset();
        self.status = "Reset".to_string();
    }

    fn cycle(&mut self) {
        self.palette = match self.palette {
            ThermalPalette::SplinterCell => ThermalPalette::Classic,
            ThermalPalette::Classic => ThermalPalette::Ironbow,
            ThermalPalette::Ironbow => ThermalPalette::HotCold,
            ThermalPalette::HotCold => ThermalPalette::SplinterCell,
        };
        self.color_mapper.set_palette(self.palette);
        self.status = format!("{:?}", self.palette);
    }

    fn toggle(&mut self) {
        self.demo = !self.demo;
        if self.demo {
            self.last_demo = Instant::now();
        }
        self.status = if self.demo { "Demo ON" } else { "Demo OFF" }.to_string();
    }

    fn update(&mut self) {
        if self.demo && self.last_demo.elapsed() >= Duration::from_secs(8) {
            self.random_code();
            self.last_demo = Instant::now();
        }
    }

    fn draw(&self, display: &mut SimulatorDisplay<Rgb888>) {
        display.clear(Rgb888::BLACK).unwrap();
        let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

        Text::new("Thermal Pinpad - Splinter Cell", Point::new(10, 20), style).draw(display).unwrap();
        Text::new("D: Demo | R: Reset | P: Palette", Point::new(10, 40), style).draw(display).unwrap();
        Text::new(&self.status, Point::new(10, 60), style).draw(display).unwrap();

        let cfg = self.keypad.config();
        let now = self.ms();
        let ints = self.keypad.intensities(now);

        for row in 0..4 {
            for col in 0..3 {
                let label = LABELS[row][col];
                let rect = self.rect(row, col);
                let intensity = ints.iter().find(|(l, _)| *l == label).map(|(_, i)| *i).unwrap_or(0.0);

                let bg = if intensity >= cfg.min_visible_intensity {
                    self.color_mapper.intensity_to_rgb(intensity)
                } else {
                    Rgb888::new(20, 20, 40)
                };

                rect.into_styled(
                    PrimitiveStyleBuilder::new().fill_color(bg).stroke_color(Rgb888::new(60, 60, 80)).stroke_width(2).build()
                ).draw(display).unwrap();

                if intensity >= cfg.min_visible_intensity {
                    let center = rect.center();
                    let max_r = (BUTTON_WIDTH.min(BUTTON_HEIGHT) / 2) as i32;
                    for ring in 0..cfg.num_rings {
                        let ri = ring_intensity(intensity, ring, cfg.num_rings);
                        if ri >= cfg.min_visible_intensity {
                            let rc = self.color_mapper.intensity_to_rgb(ri);
                            let r = (max_r as f32 * (ring as f32 + 1.0) / cfg.num_rings as f32) as u32;
                            Circle::with_center(center, r).into_styled(
                                PrimitiveStyleBuilder::new().stroke_color(rc).stroke_width(1).build()
                            ).draw(display).unwrap();
                        }
                    }
                }

                Text::new(&label.to_string(), Point::new(rect.center().x - 3, rect.center().y + 4), style).draw(display).unwrap();
            }
        }

        Text::new(&format!("{:?}", self.palette), Point::new(10, DISPLAY_HEIGHT as i32 - 30), style).draw(display).unwrap();
        Text::new(if self.demo { "Demo: ON" } else { "Demo: OFF" }, Point::new(10, DISPLAY_HEIGHT as i32 - 15), style).draw(display).unwrap();
    }
}

fn main() {
    let out = OutputSettingsBuilder::new().pixel_spacing(1).build();
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Thermal Pinpad", &out);
    let mut state = State::new();

    loop {
        state.update();
        state.draw(&mut display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    use embedded_graphics_simulator::sdl2::Keycode;
                    match keycode {
                        Keycode::D => state.toggle(),
                        Keycode::R => state.reset(),
                        Keycode::P => state.cycle(),
                        Keycode::Escape => break,
                        _ => {}
                    }
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    if let Some(label) = state.at_point(point) {
                        state.press(label);
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}
