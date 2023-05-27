use tokio::{net::UdpSocket, sync::mpsc, task};
// use std::net::UdpSocket;
// use async_std::net::UdpSocket;
use std::{io, sync::Arc};
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};

extern crate nalgebra as na;
use na::{Vector3};

use serde_yaml::{self};

use r2r::{QosProfile, builtin_interfaces};

#[derive(Serialize, Deserialize, Debug)]
struct IMUData {
    pub lin_accel: Vector3<f32>,
    pub ang_vel: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SentiROSConfig {
    pub port: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("config.yaml").expect("Could not open file.");
    let cfg: SentiROSConfig =
        serde_yaml::from_reader(f).expect("Could not parse file.");


    let port_str = cfg.port.to_string();
    let mut addr = "0.0.0.0:".to_owned();
    addr.push_str(&port_str);
    
    println!("Creating UDP socket and binding it to: {}", addr);
    let sock = UdpSocket::bind(addr).await?;
    // sock.connect("172.16.1.170:6004");
    // use `sock`
    // let mut buf = vec![0; 10];
    // let n = sock.recv(&mut buf).await?;
    // println!("received {} bytes {:?}", n, &buf[..n]);

    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "senti_ros", "")?;

    let imu_pub = node.create_publisher::<r2r::sensor_msgs::msg::Imu>("/rov30k/imu/data_raw", QosProfile::sensor_data())?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_micros(1));
    });


    loop {
        sock.readable().await?;

        let mut buf = Vec::with_capacity(1024);

        // Try to recv data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match sock.try_recv_buf(&mut buf) {
            Ok(n) => {
                // println!("GOT {:?}", &buf[..n]);
                let deserialized_imu_data: IMUData = rmp_serde::from_slice(&buf).unwrap();
                let ros_imu_msg = create_imu_ros_msg(&deserialized_imu_data);
                imu_pub.publish(&ros_imu_msg).unwrap();
                // println!("{:?}", deserialized);
                // break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                // return Err(e);
            }
        }
    }

    Ok(())
}

fn create_imu_ros_msg(imu_data: &IMUData) -> r2r::sensor_msgs::msg::Imu {
    let lin_accel = imu_data.lin_accel;
    let ang_vel = imu_data.ang_vel;
    let t_now = std::time::SystemTime::now();
    let since_epoch = t_now.duration_since(std::time::UNIX_EPOCH).unwrap();


    r2r::sensor_msgs::msg::Imu {
        header: r2r::std_msgs::msg::Header {
            stamp: builtin_interfaces::msg::Time {
                sec: since_epoch.as_secs() as i32,
                nanosec: since_epoch.subsec_nanos(),
            },
            frame_id: "".to_string(),
        },
      angular_velocity: r2r::geometry_msgs::msg::Vector3 {
        x: ang_vel[0] as f64,
        y: ang_vel[1] as f64,
        z: ang_vel[2] as f64,      
      },
      linear_acceleration: r2r::geometry_msgs::msg::Vector3 {
        x: lin_accel[0] as f64,
        y: lin_accel[1] as f64,
        z: lin_accel[2] as f64,      
      },
      ..Default::default()
    }
  }
  
