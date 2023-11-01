use eframe::egui;

pub fn custom_font(cc: &eframe::CreationContext<'_>){
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "OPPOSans-L".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/fonts/OPPOSans-L.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "OPPOSans-L".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("OPPOSans-L".to_owned());

    // Tell egui to use these fonts:
    cc.egui_ctx.set_fonts(fonts);
}