use config::{Config, File};

#[derive(Debug, Clone)]
#[warn(unused_must_use)]
pub struct Configs {
    database: String,
}

impl Configs {
    pub fn new(path_name: String) -> Box<Self> {
        let conf = Config::builder()
            .add_source(File::with_name(path_name.as_str()))
            .build()
            .expect("read configs not found");
        Box::new(Configs {
            database: conf.get::<String>("databse.mysql").unwrap(),
        })
    }
    pub fn get_database(self) -> String {
        self.database
    }
}
