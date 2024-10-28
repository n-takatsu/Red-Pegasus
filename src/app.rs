use winit::window::Window;
use std::sync::{Arc, Mutex};

// アプリケーションの状態を管理する構造体
pub struct State {
    pub color: [f32; 4], // キューブの色をRGBAで表現
}

impl State {
    // 新しいStateインスタンスを生成（初期色: 赤）
    pub fn new() -> Self {
        Self {
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }

    // 色を更新するメソッド
    pub fn update_color(&mut self, new_color: [f32; 4]) {
        self.color = new_color;
    }
}

// レンダリングと状態管理を統括する構造体
pub struct App {
    pub state: Arc<Mutex<State>>,  // アプリケーションの状態を保持
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    #[allow(dead_code)]
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
}

impl App {
    // 新しいAppインスタンスを初期化
    pub async fn new(window: &Window) -> Self {
        // wgpuの初期設定
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        // Surface設定
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        // シェーダーモジュールの作成
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        // パイプラインの作成
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            state: Arc::new(Mutex::new(State::new())),
            device,
            queue,
            surface,
            config,
            render_pipeline,
        }
    }

    // 状態の更新
    pub fn update(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.update_color([0.0, 1.0, 0.0, 1.0]);
    }

    // ウィンドウサイズが変更された際に呼び出すメソッド
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(&self.device, &self.config);
    }

    // 描画処理
    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.state.lock().unwrap().color[0] as f64,
                            g: self.state.lock().unwrap().color[1] as f64,
                            b: self.state.lock().unwrap().color[2] as f64,
                            a: self.state.lock().unwrap().color[3] as f64,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
