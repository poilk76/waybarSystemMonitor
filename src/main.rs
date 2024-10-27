use sysinfo::{ System, Components , CpuRefreshKind , RefreshKind };
use std::fs::File;
use std::fs;
use std::path::Path;
mod reader;
use crate::reader::{ QueFunctions, DataFunctions };
mod graph;


pub struct SystemData {
    pub ram_usage: Vec<u8>,
    pub ram_usage_mb: u64,
    pub ram_all_mb: u64,
    pub cpu_usage: Vec<u8>,
    pub cpu_temp: u8,
}

fn get_cpu_usage() -> u8{
    let mut s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_cpu_all();
    let mut usage:f32 = 0.0;
    let mut count:f32 = 0.0;
    for cpu in s.cpus() {
        usage += cpu.cpu_usage();
        count += 1.0;
    };
    usage = usage/count;
    
    return usage.floor() as u8
}

pub fn construct() -> SystemData {
    let mut sys = System::new_all();
    let components = Components::new_with_refreshed_list();
    let path = Path::new("cache");
    sys.refresh_all();
    
    if !path.exists() {
      let _ = File::create("cache");
      let _ = fs::write("cache", "0\n0");
    };
    let file = match File::open(path) {
      Ok(file) => file,
      Err(e) => panic!("couldn't open {}",e)
    };
    let mut data = reader::Data{
      ram_usage:reader::Que{values:vec![]},
      cpu_usage:reader::Que{values:vec![]},
      file:file
    };
    data.read();
    data.cpu_usage.add(get_cpu_usage());
    data.ram_usage.add((sys.used_memory()*100/sys.total_memory()) as u8);
    let result = SystemData {
        cpu_usage: data.cpu_usage.values.clone(),
        cpu_temp: components[0].temperature().floor() as u8,
        ram_usage: data.ram_usage.values.clone(),
        ram_usage_mb: (sys.used_memory()/1024/1024),
        ram_all_mb: (sys.total_memory()/1024/1024)
    };
    data.write();

    return result
}

fn main() {
  let data = construct();
  println!("{{\"text\": \"󰍹\", \"tooltip\": \"CPU temp: {}°C\\nCPU usage:             {}%\\n{}\\nRAM usage:       {}MB/{}MB\\n{}\"}}",
  data.cpu_temp,
  data.cpu_usage[29],
  graph::create_graph(&data.cpu_usage),
  data.ram_all_mb,
  data.ram_usage_mb,
  graph::create_graph(&data.ram_usage)
  );
}