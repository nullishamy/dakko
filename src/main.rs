#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{mpsc, Arc},
    thread,
};
use tokio::{runtime::Runtime, sync::RwLock};

mod app;
mod http;
mod components;

// When compiling natively:
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let (req_tx, req_rx) = mpsc::channel::<http::HttpRequest>();
    let (res_tx, res_rx) = mpsc::channel::<http::HttpResponse>();

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    let handle = thread::spawn(move || {
        let state = Arc::new(RwLock::new(http::HttpState {
            client: None,
            tx: res_tx.clone(),
            app_data: None,
            egui_ctx: None,
        }));

        while let Ok(msg) = req_rx.recv() {
            http::handle(&state, msg, &rt);
        }
    });

    // let html = r#"
    // <p>
    //     OMG! Today I learned that what my mother termed my &quot;moods&quot; were 
    //     <a href="https://toot.wales/tags/ActuallyAutistic" rel="tag">
    //         #<span>ActuallyAutistic</span>
    //     </a> 
    //     shutdowns.
    // </p>
    // <p>
    //     <a href="https://mysoulbalm.blog/2022/01/11/autistic-shutdowns-guide-for-neurodivergent-adults/?mc_cid=8ce0972776&amp;mc_eid=629ae20a31">
    //         <span>https://</span>
    //         <span>mysoulbalm.blog/2022/01/11/aut</span>
    //         <span>istic-shutdowns-guide-for-neurodivergent-adults/?mc_cid=8ce0972776&amp;mc_eid=629ae20a31</span>
    //     </a>
    // </p>

    // "#;
    // components::content::ContentComponent::new(html.to_string());

    eframe::run_native(
        "dakko",
        native_options,
        Box::new(move |cc| Box::new(app::MainApp::new(cc, (req_tx.clone(), res_rx)))),
    )
}
