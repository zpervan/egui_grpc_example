use std::default::Default;
use std::sync::{Arc, Mutex};

use eframe::Frame;
use egui::Context;
use tokio::runtime;
use tonic::transport::Channel;

use crate::proto::rpc::{DummyRequest, test_client::*};

pub async fn create_connection() -> TestClient<Channel> {
    TestClient::connect("http://localhost:3500").await.expect("Couldn't connect to gRPC server")
}

pub struct TestApp {
    client: TestClient<Channel>,
    runtime: runtime::Runtime,
    value: Arc<Mutex<usize>>,
    response: Arc<Mutex<String>>,
}

impl TestApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // The connection to the gRPC server MUST by async that's why we create a Tokio runtime in order to connect to the server
        let runtime = runtime::Builder::new_multi_thread().enable_all().build().expect("Couldn't create multi-thread runtime");
        let client = runtime.block_on(create_connection());

        Self {
            client,
            runtime,
            value: Arc::new(Mutex::new(0)),
            response: Arc::new(Mutex::new(String::from("-"))),
        }
    }

    pub fn spawn_test_connection(&mut self) {
        let value_ref = self.value.clone();
        let response_ref = self.response.clone();
        let client = self.client.clone();

        self.runtime.spawn(async move {
            Self::test_connection(value_ref, response_ref, client).await;
        });
    }

    async fn test_connection(value: Arc<Mutex<usize>>, response: Arc<Mutex<String>>, mut client: TestClient<Channel>) {
        *value.lock().unwrap() += 1;
        let request = DummyRequest { dummy_data: format!("Server pinged {} times", *value.lock().unwrap()) };

        match client.test_connection(request).await {
            Ok(result) => *response.lock().unwrap() = result.into_inner().dummy_data,
            Err(e) => println!("Error. Reason: {}", e.to_string())
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