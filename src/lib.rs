pub mod lib 
{
    #![allow(dead_code)]
    use serde::{Serialize, Deserialize};
    use serde_json;
    use sqlx::mysql::MySqlPool;
    use std::{env};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AutoData
    {
        moved: bool,
        low:   u8,
        outer: u8,
        inner: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TeleopData
    {
        low:              u8,
        outer:            u8,
        inner:            u8,
        rotation_control: bool,
        position_control: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum HangState
    {
        None,
        Park,
        Hang,
        Level,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EndgameData
    {
        hang:  HangState,
        level: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MatchData
    {
        match_number: u8,
        team_number:  u16,
        scouter_name: String,
        auto:         AutoData,
        teleop:       TeleopData,
        endgame:      EndgameData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum DrivetrainTypes
    {
        Tank,
        Swerve,
        Mecanum,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum MotorType
    {
        Falcon500,
        KrakenX60,
        KrakenX44,
        NeoV1_1,
        Neo550,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RobotSpec
    {
        drivetrain: DrivetrainTypes,
        weight_lb:  f32,
        motor:      MotorType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RobotCapabilities
    {
        auto_move: bool,
        auto_low:  bool,
        auto_high: bool,
        climb:     bool,
        wheel_of_fortune: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PitData
    {
        team_name:    String,
        team_number:  u16,
        spec:         RobotSpec,
        capabilities: RobotCapabilities,
        comment:      String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum DBRequestError 
    {
        NotImplemented,
        ConnectionFaliure
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ServerInfo
    {
        description: String,
        version: String,
    }

    pub trait DatabaseInterface 
    {
        fn get_server_info(&self) -> ServerInfo;

        async fn set_match_data(&self, data: MatchData) -> impl Future<Output = Result<(), DBRequestError>>;
        async fn set_pit_data  (&self, data: PitData)   -> impl Future<Output = Result<(), DBRequestError>>;

        async fn get_all_match_data(&self)                  -> impl Future<Output = Result<Vec<MatchData>, DBRequestError>>;
        async fn get_all_pit_data  (&self)                  -> impl Future<Output = Result<Vec<PitData>,   DBRequestError>>;
        async fn get_match_data    (&self, identifier: u32) -> impl Future<Output = Result<MatchData,      DBRequestError>>;
        async fn get_pit_data      (&self, identifier: u32) -> impl Future<Output = Result<PitData,        DBRequestError>>;
    }

    pub struct DebugDatabase {}

    impl DatabaseInterface for DebugDatabase 
    {
        fn get_server_info(&self) -> ServerInfo
        {
            ServerInfo{
                description: stringify!(This is the debug server).to_string(),
                version: stringify!(0.1.0).to_string()}
        }

        async fn set_match_data(&self, data: MatchData) -> impl Future<Output = Result<(), DBRequestError>>
        {
            println!("DEBUG: Set match data"); // add identifier
            println!("{}", serde_json::to_string(&data).unwrap());
            async{ Err(DBRequestError::NotImplemented)}
        }

        async fn set_pit_data(&self, data: PitData) -> impl Future<Output = Result<(), DBRequestError>>
        {
            println!("DEBUG: Set pit data"); // add identifier
            println!("{}", serde_json::to_string(&data).unwrap());
            async{ Err(DBRequestError::NotImplemented) }
        }

        async fn get_all_match_data(&self) -> impl Future<Output = Result<Vec<MatchData>, DBRequestError>>
        {
            println!("DEBUG: Get all match data");
            async{ Err(DBRequestError::NotImplemented) }
        }

        async fn get_all_pit_data(&self) -> impl Future<Output = Result<Vec<PitData>, DBRequestError>>
        {
            println!("DEBUG: Get all pit data");
            async{ Err(DBRequestError::NotImplemented) }
        }

        async fn get_match_data(&self, identifier: u32) -> impl Future<Output = Result<MatchData, DBRequestError>>
        {
            println!("DEBUG: Get match data with identifier: {}", identifier);
            async{ Err(DBRequestError::NotImplemented) }
        }

        async fn get_pit_data(&self, identifier: u32) -> impl Future<Output = Result<PitData, DBRequestError>>
        {
            println!("DEBUG: Get pit data with identifier: {}", identifier);
            async{ Err(DBRequestError::NotImplemented) }
        }
    } // impl DatabaseInterface for DebugDatabase

    struct DatabaseReal
    {
        db_connection: MySqlPool,
    }

    impl DatabaseReal
    {
        async fn new() -> Self
        {
            // If these don't work, error out and let the user try again. The compile time error should trigger immedietly after running.
            // mysql://user:pass@host/database is the template for the database url
            Self{db_connection: MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap()}
        }
    }

    impl DatabaseInterface for DatabaseReal 
    {
        fn get_server_info(&self) -> ServerInfo
        {
            ServerInfo{
                description: stringify!(The backend using rust through actix and sqlx/mysql.).to_string(),
                version:     stringify!(0.1.0).to_string(),
            }
        }

        async fn set_match_data(&self, _data: MatchData) -> impl Future<Output = Result<(), DBRequestError>>
        {
            let result = sqlx::query("").execute(&self.db_connection).await;
            async{ Err(DBRequestError::NotImplemented) }
        }
        async fn set_pit_data(&self, data: PitData) -> impl Future<Output = Result<(), DBRequestError>>
        {
            println!("DEBUG: Set pit data"); // add identifier
            println!("{}", serde_json::to_string(&data).unwrap());
            async { Err(DBRequestError::NotImplemented) }
        }

        async fn get_all_match_data(&self) -> impl Future<Output = Result<Vec<MatchData>, DBRequestError>>
        {
            println!("DEBUG: Get all match data");
            async { Err(DBRequestError::NotImplemented) }
        }

        async fn get_all_pit_data(&self) -> impl Future<Output = Result<Vec<PitData>, DBRequestError>>
        {
            println!("DEBUG: Get all pit data");
            async { Err(DBRequestError::NotImplemented) }
        }

        async fn get_match_data(&self, identifier: u32) -> impl Future<Output = Result<MatchData, DBRequestError>>
        {
            println!("DEBUG: Get match data with identifier: {}", identifier);
            async { Err(DBRequestError::NotImplemented) }
        }

        async fn get_pit_data(&self, identifier: u32) -> impl Future<Output = Result<PitData, DBRequestError>>
        {
            println!("DEBUG: Get pit data with identifier: {}", identifier);
            async { Err(DBRequestError::NotImplemented) }
        }

    }

} // mod lib