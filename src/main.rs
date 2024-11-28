use reqwest::Client;
use serde::Deserialize;
use prometheus::{Encoder, TextEncoder, GaugeVec, opts, register_gauge_vec};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::env;

#[derive(Debug, Deserialize)]
struct Appointment {
    Subject: String,
    Organizer: String,
    Start: i64,
    End: i64,
    Private: bool,
}

#[derive(Debug, Deserialize)]
struct Room {
    Roomlist: String,
    Name: String,
    RoomAlias: String,
    Email: String,
    Appointments: Vec<Appointment>,
    Busy: bool,
}

lazy_static::lazy_static! {
    static ref ROOM_OCCUPIED: GaugeVec = register_gauge_vec!(
        opts!(
            "meeting_room_occupied",
            "Indicates if the room is currently occupied"
        ),
        &["room_alias", "room_name"]
    )
    .unwrap();

    static ref ROOM_APPOINTMENT_DETAILS: GaugeVec = register_gauge_vec!(
        opts!(
            "meeting_room_appointment_details",
            "Details of individual appointments in meeting rooms"
        ),
        &["room_alias", "subject", "organizer", "private", "start", "end"]
    )
    .unwrap();
}

async fn fetch_room_data(api_url: &str) -> Result<Vec<Room>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/api/rooms", api_url);
    let response = client.get(&url).send().await?.json::<Vec<Room>>().await?;
    Ok(response)
}

async fn update_metrics(api_url: &str) -> Result<(), String> {
    match fetch_room_data(api_url).await {
        Ok(rooms) => {
            for room in rooms {
                let is_occupied = if room.Busy { 1.0 } else { 0.0 };
                ROOM_OCCUPIED
                    .with_label_values(&[&room.RoomAlias, &room.Name])
                    .set(is_occupied);

                for appointment in room.Appointments {
                    ROOM_APPOINTMENT_DETAILS
                        .with_label_values(&[
                            &room.RoomAlias,
                            &appointment.Subject,
                            &appointment.Organizer,
                            &appointment.Private.to_string(),
                            &appointment.Start.to_string(),
                            &appointment.End.to_string(),
                        ])
                        .set(1.0);
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Error fetching room data: {}", e)),
    }
}

// Serve Prometheus metrics dynamically
async fn serve_metrics(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let api_url = env::var("API_URL").unwrap();

    if let Err(err) = update_metrics(&api_url).await {
        eprintln!("{}", err);
        return Ok(Response::builder()
            .status(500)
            .body(Body::from("Failed to fetch data from API"))
            .unwrap());
    }

    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();

    Ok(Response::new(Body::from(buffer)))
}

// Main function to run the exporter
#[tokio::main]
async fn main() {
    let _api_url = env::var("API_URL").expect("API_URL environment variable not set");

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(serve_metrics))
    });

    let addr = ([0, 0, 0, 0], 8000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Exporter running at http://localhost:8000/metrics");

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
