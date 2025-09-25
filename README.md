# MQTT Desktop Notification

## Sample

```
cargo run -- -v
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/mqttdn -v`
1758766494.707 :: INFO  :: mqttdn::config:57                             config file: /home/joe/mqttdn/config/mqttdn.toml
1758766494.707 :: INFO  :: mqttdn:163                                    pid=/home/joe/mqttdn/target/debug/mqttdn.pid exists=true
MQTT Desktop Notification
    Server:          10.0.0.2
    Topics:          4
    Verbose:         true
1758766494.707 :: INFO  :: mqttdn:94                                     connecting to 10.0.0.2 as mqttdn_6029e4e7-d3b4-4c89-afee-d994b01d3eef
1758766494.707 :: INFO  :: mqttdn:102                                    sub /motion/office
1758766494.707 :: INFO  :: mqttdn:102                                    sub /motion/porch
1758766494.708 :: INFO  :: mqttdn:102                                    sub /motion/driveway
1758766494.708 :: INFO  :: mqttdn:102                                    sub /motion/stairs
1758766556.483 :: INFO  :: mqttdn:126                                    event topic=/motion/driveway payload=enter
```

## Install

```
cargo install --git https://github.com/uintptr/mqttdn
```

## Requirements

- `aosd_cat` for the On-Screen-Display (OSD) notifications
