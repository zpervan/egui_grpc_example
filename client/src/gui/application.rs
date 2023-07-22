use std::default::Default;
use std::sync::{Arc, Mutex};

use eframe::Frame;
use egui::Context;
use tonic_web_wasm_client::Client;
use wasm_bindgen_futures::spawn_local;
use crate::proto::rpc::{DummyRequest, test_client::*};

pub fn create_connection() -> TestClient<Client> {
    log::info!("Creating connection to gRPC server");

    let base_url = "http://localhost:3500".to_string();
    let wasm_client = Client::new(base_url);
    TestClient::new(wasm_client)
}

pub struct TestApp {
    client: TestClient<Client>,
    value: Arc<Mutex<usize>>,
    response: Arc<Mutex<String>>,
}

impl TestApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            client: create_connection(),
            value: Arc::new(Mutex::new(0)),
            response: Arc::new(Mutex::new(String::from("-"))),
        }
    }

    pub fn spawn_test_connection(&mut self) {
        let value_ref = self.value.clone();
        let response_ref = self.response.clone();
        let client = self.client.clone();

        spawn_local(async move {
            log::info!("Calling test_connection");
            Self::test_connection(value_ref, response_ref, client.clone()).await;
        });
    }

    async fn test_connection(value: Arc<Mutex<usize>>, response: Arc<Mutex<String>>, mut client: TestClient<Client>) {
        log::info!("Calling test_connection async wrapper");

        *value.lock().unwrap() += 1;
        let request = DummyRequest { dummy_data: format!("Server pinged {} times", *value.lock().unwrap()) };

        match client.test_connection(request).await {
            Ok(result) => {
                log::info!("Response!");
                *response.lock().unwrap() = result.into_inner().dummy_data
            },
            Err(e) => log::error!("Error. Reason: {}", e.to_string())
        }
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("gRPC pinger");

                ui.spacing();

                ui.label("Response from server:");
                ui.label(format!("{}", *self.response.lock().unwrap()));

                if ui.button("Ping server").clicked() {
                    self.spawn_test_connection();
                }
            })
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting application");
    }
}