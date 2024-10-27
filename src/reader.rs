use std::fs::File;
use std::fs;
use std::io::Read;

pub trait QueFunctions {
    fn add(&mut self,to_add:u8);
    fn str_to_vec(&mut self,str_line:&str);
}

pub trait DataFunctions {
    fn read(&mut self);
    fn write(self);
}

pub struct Que {
    pub values: Vec<u8>
}

pub struct Data {
    pub ram_usage: Que,
    pub cpu_usage: Que,
    pub file: File
}

impl QueFunctions for Que {
    fn add (&mut self,to_add:u8) {
        self.values.remove(0);
        self.values.push(to_add);
    }
    fn str_to_vec (&mut self,str_line:&str) {
        let splited_str = str_line.split(" ");
        for val in splited_str {
            match val.parse::<u8>() {
                Ok(num) => self.values.push(num),
                Err(_) => (),
            }
        };
        for _ in 0..(30-self.values.len()){
            self.values.push(0);
        }
    }
}

impl DataFunctions for Data {
    fn read(&mut self) {
        self.ram_usage = Que{values:Vec::new()};
        self.cpu_usage = Que{values:Vec::new()};

        let mut from_file:String = String::new();
        match self.file.read_to_string(&mut from_file) {
            Ok(o) => o,
            Err(e) => panic!("couldn't read {}",e)
        };
        let splited_file:Vec<&str> = from_file.split('\n').collect();
        self.cpu_usage.str_to_vec(splited_file[0]);
        self.ram_usage.str_to_vec(splited_file[1]);
    }
    fn write(self) {
        let mut to_write = String::from("");
        for val in self.cpu_usage.values {
            to_write += &(val.to_string()+" ");
        }
        to_write.push('\n');
        for val in self.ram_usage.values {
            to_write += &(val.to_string()+" ");
        }

        let _ = fs::write("cache",to_write.as_bytes());
    }
}
