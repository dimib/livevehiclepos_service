# Live Vehicle Position Service

This is a simulation for live vehicle positions as they are provided usually by public transport operators.

The idea behind this simulation is to record real bus or other vehicle movements and replay them later for test and development purposes.

# Let's go

## You need Rust

This service is written in Rust, so you need the install the Rust development environment first. I assume that you are familiar with at least installing and using software using a Unix shell.

See https://www.rust-lang.org/tools/install for details.

## Clone and build the project

In a unix shell just type

```
$ git clone https://github.com/dimib/livevehiclepos_service
$ cd livevehiclepos_service
$ cargo build
$ cargo run
```

This will load the simulation data and start the service.

## Configuration

You need position files to replay and define vehicle IDs for requests. There is a configuration file in the project's root directory: `sim_vehicle.json`.

```
{
    "vehicle_simulations": [
        { "vehicle_id": "23-1", "pos_file": "retro/2024-05-17_19-41-40-23-olivaer-barmbek.pos", "step": 4 },
        { "vehicle_id": "23-2", "pos_file": "retro/2024-05-18_17-37-18-23-barmbek-laemmersieth.pos", "step": 4 },
        { "vehicle_id": "7-1", "pos_file": "retro/2024-05-17_19-55-04-7-barmbek-langenfort.pos", "step": 4 },
        { "vehicle_id": "7-2", "pos_file": "retro/2024-05-18_17-25-42-7-langenfort-barmbek.pos", "step": 4 }
    ],
    "vehicle_id_mapping": [
        { "sim_vehicle_id": "23-1", "real_vehicle_ids": "4256 2456" },
        { "sim_vehicle_id": "23-2", "real_vehicle_ids": "9812 3972" },
        { "sim_vehicle_id": "7-1", "real_vehicle_ids": "9086 1134" },
        { "sim_vehicle_id": "7-2", "real_vehicle_ids": "8652 3411" }
    ]
}
```
`vehicle_simulations` contains the simulation files and assigns vehicleIds.

`vehicle_id` is the vehicleId.

`pos_file` file with the recorded positions.

`step` Number of positions to skip when you need to speed up the track.

`vehicle_id_mapping` can be used to map real vehicleIds from getRoute to simulated vehicleIds.

`sim_vehicle_id` our known vehicleIds

`real_vehicle_ids` a string with real vehicleIds separated by spaces.

## Requests

You can use `curl` or `wget` to test the service:

Request positions for one vehicle

```
$ curl -H 'Accept: application/json' 'http://127.0.0.1:3003/positions/?vehicleIds=23-1'         
[{"lastReceived":"2024-05-19T10:26:25+02:00","position":{"latitude":53.58308745553119,"longitude":10.056276172408886},"vehicleId":"23-1"}]
```

Request positions for many vehicles

```
% curl -H 'Accept: application/json' 'http://127.0.0.1:3003/positions/?vehicleIds=23-1,23-2,7-1'
[{"lastReceived":"2024-05-19T10:27:11+02:00","position":{"latitude":53.58299173419697,"longitude":10.05624415353877},"vehicleId":"23-1"},{"lastReceived":"2024-05-19T10:27:11+02:00","position":{"latitude":53.58754004822447,"longitude":10.045582121247469},"vehicleId":"23-2"},{"lastReceived":"2024-05-19T10:27:11+02:00","position":{"latitude":53.58773522083982,"longitude":10.045331418523606},"vehicleId":"7-1"}]

```

## Admin

The is also a URL for admin purposes:

### Reload the simulation

```
curl -H 'Accept: application/json' 'http://127.0.0.1:3003/admin/reload'
```