use egui::{
    plot::{Legend, Line, Plot, PlotPoints},
    Color32, Ui,
};

pub struct TemplateApp {
    m0: f64,
    v0: f64,
    m1: f64,
    v1: f64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            m0: 0.,
            v0: 0.,
            m1: 0.,
            v1: 0.,
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl TemplateApp {
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self { m0, v0, m1, v1 } = self;

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Object 1:");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(m0, -5.0..=5.0));
                        ui.label("Mass");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(v0, -5.0..=5.0));
                        ui.label("Velocity");
                    });
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Object 2:");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(m1, -5.0..=5.0));
                        ui.label("Mass");
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(v1, -5.0..=5.0));
                        ui.label("Velocity");
                    });
                });
            });
        });
    }

    fn preserved_momentum(&self) -> Line {
        let &Self { m0, v0, m1, v1, .. } = self;

        Line::new(PlotPoints::from_explicit_callback(
            move |x| {
                // m0v0 + m1v1 = m0x + m1y
                // m0v0 + m1v1 - m0x = m1y
                // (m0v0 + m1v1 - m0x)/m1 = y
                // m0(v0 - x)/m1 + v1 = y
                m0 * (v0 - x) / m1 + v1
            },
            ..,
            512,
        ))
        .color(Color32::RED)
        .width(3.)
    }

    fn preserved_energy(&self) -> (Line, Line) {
        let &Self { m0, v0, m1, v1, .. } = self;

        // m0v0^2 + m1v1^2 = m0x^2 + m1y^2

        // for y = 0:
        // d = m0x^2
        // d/m0 = x^2
        // x = sqrt(d/m0)

        // to find y given x:
        // d = m0x^2 + m1y^2
        // (d - m0x^2)/m1 = y^2
        // y = sqrt((d - m0x^2)/m1)

        let d = m0 * v0 * v0 + m1 * v1 * v1;
        // let bound = (d / m0).sqrt();

        let upper = Line::new(PlotPoints::from_explicit_callback(
            move |x| ((d - m0 * x * x) / m1).sqrt(),
            // -bound..=bound,
            ..,
            512,
        ))
        .color(Color32::BLUE)
        .width(3.);
        let lower = Line::new(PlotPoints::from_explicit_callback(
            move |x| -((d - m0 * x * x) / m1).sqrt(),
            // -bound..=bound,
            ..,
            512,
        ))
        .color(Color32::BLUE)
        .width(3.);

        (upper, lower)
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);

            let format_axis = |n, _a: &_| format!("{n} m/s");
            let plot = Plot::new("plot")
                .view_aspect(1.0)
                .data_aspect(1.0)
                .x_axis_formatter(format_axis)
                .y_axis_formatter(format_axis)
                .label_formatter(|_name, point| {
                    format!(
                        "v₁ = {v1:.3} m/s\nv₂ = {v2:.3} m/s",
                        v1 = point.x,
                        v2 = point.y
                    )
                })
                .legend(Legend::default());
            plot.show(ui, |plot_ui| {
                plot_ui.line(
                    self.preserved_momentum()
                        .name("States with preserved momentum"),
                );
                let (upper, lower) = self.preserved_energy();
                plot_ui.line(upper.name("States with preserved energy"));
                plot_ui.line(lower.name("States with preserved energy"));
            })
        });

        #[cfg(debug_assertions)]
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}
