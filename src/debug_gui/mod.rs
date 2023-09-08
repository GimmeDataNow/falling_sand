use egui::{ClippedPrimitive, Context, TexturesDelta, RichText};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;
use crate::chunk_manager::chunks::cells::{CELL_PROPERTIES, CellTypeProperties, CellType};

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

/// Example application state. A real application will need a lot more state than this.
struct Gui {
    /// Only show the egui window when true.
    window_open: bool,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window, coords:&mut (i32, i32), fps: &mut fps_counter::FPSCounter, paint_brush_toggle: &mut bool, paint_material: &mut CellType, toggle_simulation: &mut bool) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            self.gui.ui(egui_ctx, coords, fps, paint_brush_toggle, paint_material, toggle_simulation);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass: wgpu::RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}

impl Gui {
    /// Create a `Gui`.
    fn new() -> Self {
        Self { window_open: true }
    }

    /// Create the UI using egui.
    fn ui(&mut self,
         ctx: &Context, 
         coords: &mut (i32, i32), 
         fps: &mut fps_counter::FPSCounter,
         paint_brush_toggle: &mut bool,
         paint_material: &mut CellType,
         toggle_simulation: &mut bool) {
        //egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
        //    //egui::menu::bar(ui, |ui| {
        //    //    ui.menu_button("File", |ui| {
        //    //        if ui.button("About...").clicked() {
        //    //            self.window_open = true;
        //    //            ui.close_menu();
        //    //        }
        //    //    })
        //    //});
        //});

        egui::Window::new("Debug")
            .open(&mut self.window_open)
            .show(ctx, |ui| {
                ui.horizontal(|ui|{
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label(RichText::new("Position:").strong());
                    ui.spacing();
                    ui.label("x:");
                    ui.add(egui::DragValue::new(&mut coords.0));
                    ui.label("y:");
                    ui.add(egui::DragValue::new(&mut coords.1));
                });
                ui.horizontal(|ui: &mut egui::Ui|{
                    ui.label(egui::RichText::new("FPS: ").strong());
                    ui.label(format!("{:#?}", fps.tick()));

                });
        });

        egui::Window::new("World tools").open(&mut self.window_open).show(ctx, |ui: &mut egui::Ui| {
            ui.horizontal(|ui|{
                ui.label(RichText::new("Paintbrush:").strong());
                ui.checkbox(paint_brush_toggle, "");
                egui::ComboBox::from_label("")
                .selected_text(format!("{paint_material:?}"))
                .show_ui(ui, |ui: &mut egui::Ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    // i could dump this to a const function / a function that evaluates at compile time
                    for i in 0..CELL_PROPERTIES.len() {
                        let properties: &CellTypeProperties = CellTypeProperties::get_cell_properties_by_index(i);
                        &ui.selectable_value(paint_material, properties.cell_type, properties.name);
                    }
            });
            ui.checkbox(toggle_simulation, "toggle simulation")
            });
        });
    }
}
