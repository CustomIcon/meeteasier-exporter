## MeetEasier Exporter for Prometheus

### Features:
- Get rooms
- Find Organizers
- Busy rooms

### Example Data from Exporter:
```
# HELP meeting_room_appointment_details Details of individual appointments in meeting rooms
# TYPE meeting_room_appointment_details gauge
meeting_room_appointment_details{end="1732775400000",organizer="John Doe",private="false",room_alias="main-meeting-1",start="1732770000000",subject="INTERVIEW"} 1
meeting_room_appointment_details{end="1732788000000",organizer="Jerry",private="false",room_alias="main-meeting-1",start="1732784400000",subject="Meeting with Western Union"} 1
meeting_room_appointment_details{end="1732795200000",organizer="Shanon Breads",private="false",room_alias="main-meeting-1",start="1732788000000",subject="Comitee"} 1
meeting_room_appointment_details{end="1733209200000",organizer="Eleanore Cummerata",private="false",room_alias="main-meeting-1",start="1733205600000",subject="DevOps Meeting"} 1
meeting_room_appointment_details{end="1733299200000",organizer="Josue Keeling",private="false",room_alias="main-meeting-1",start="1733290200000",subject="Travel Agency Handover"} 1
# HELP meeting_room_occupied Indicates if the room is currently occupied
# TYPE meeting_room_occupied gauge
meeting_room_occupied{room_alias="main-meeting-1",room_name="MAIN MR 1"} 1
```
### Usage:
#### Docker compose:
`docker compose up -d`

#### Baremetal:
- build: `cargo build --release`
- run: `./app/target/release/meeting-room-exporter`

#### Prometheus
```
scrape_configs:
  - job_name: 'meeteasier'
    static_configs:
      - targets: ['<meeteasier-exporter>:8000']
```