#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod commands {
    tonic::include_proto!("cc.arduino.cli.commands");
}

mod debug {
    tonic::include_proto!("cc.arduino.cli.debug");
}

mod monitor {
    tonic::include_proto!("cc.arduino.cli.monitor");
}

mod settings {
    tonic::include_proto!("cc.arduino.cli.settings");
}
