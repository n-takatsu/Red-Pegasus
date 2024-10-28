mod app;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use app::App;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // イベントループとウィンドウのセットアップ
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Redux-style State Management")
            .build(&event_loop)
            .unwrap();

        let mut app = App::new(&window).await;

        // イベントループ
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                // ウィンドウがリサイズされた場合
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    app.resize(new_size.width, new_size.height);
                }

                // ウィンドウの閉じるボタンが押された場合
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                // ウィンドウの再描画要求があった場合
                Event::RedrawRequested(_) => {
                    app.update();
                    app.render();
                }

                _ => {}
            }
        });
    });
}
