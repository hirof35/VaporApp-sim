use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 320.0]),
        ..Default::default()
    };

    eframe::run_native(
        "飽和水蒸気量シミュレータ",
        options,
        Box::new(|cc| {
            // アプリ起動時に日本語フォントをセットアップする
            setup_japanese_font(&cc.egui_ctx);
            Box::<VaporApp>::default()
        }),
    )
}

/// 日本語フォントを設定する関数
fn setup_japanese_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Windows, Mac, Linux で一般的にインストールされている日本語フォントのパスを指定
    fonts.font_data.insert(
        "japanese_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "C:\\Windows\\Fonts\\msgothic.ttc" // Windows (MS ゴシック)
            // Macの場合はこちらになります（今回はWindowsの標準的なパスを優先しています）
            // "/System/Library/Fonts/FontsAvailableAtOldLocation/Hiragino Mincho ProN.ttc"
        )),
    );

    // 最優先で使うフォント（Proportional / Monospace 両方）として登録
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "japanese_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "japanese_font".to_owned());

    // 設定をコンテキストに反映
    ctx.set_fonts(fonts);
}

struct VaporApp {
    temperature: f32,
}

impl Default for VaporApp {
    fn default() -> Self {
        Self { temperature: 20.0 }
    }
}

impl eframe::App for VaporApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("飽和水蒸気量 シミュレーション");
            ui.add_space(10.0);

            ui.label("現在の温度を指定してください:");
            ui.add(egui::Slider::new(&mut self.temperature, -20.0..=50.0).suffix(" °C"));
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            let temp_f64 = self.temperature as f64;
            let vapor_pressure = 6.1078 * 10.0_f64.powf((7.5 * temp_f64) / (temp_f64 + 237.3));
            let amount = (217.0 * vapor_pressure) / (temp_f64 + 273.15);

            ui.horizontal(|ui| {
                ui.label("飽和水蒸気圧:");
                ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{:.2} hPa", vapor_pressure));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("飽和水蒸気量:");
                ui.label(
                    egui::RichText::new(format!("{:.2} g/m³", amount))
                        .size(24.0)
                        .strong()
                        .color(egui::Color32::LIGHT_GREEN)
                );
            });

            ui.add_space(20.0);
            let progress = (amount / 83.0) as f32; 
            ui.add(egui::ProgressBar::new(progress).text("空気中の水分保持能力イメージ"));
        });
    }
}