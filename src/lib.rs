#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod commands {
    tonic::include_proto!("cc.arduino.cli.commands");
}

pub mod debug {
    tonic::include_proto!("cc.arduino.cli.debug");
}

pub mod monitor {
    tonic::include_proto!("cc.arduino.cli.monitor");
}

pub mod settings {
    tonic::include_proto!("cc.arduino.cli.settings");
}
