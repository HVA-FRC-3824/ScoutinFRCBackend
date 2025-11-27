pub mod lib 
{
    #![allow(dead_code)]
    use serde::{Serialize, Deserialize};

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
        version: String,
    }

    pub trait DatabaseInterface 
    {
        fn get_server_info(&self) -> ServerInfo;

        fn set_match_data(&self, data: MatchData) -> Result<(), DBRequestError>;
        fn set_pit_data  (&self, data: PitData)   -> Result<(), DBRequestError>;

        fn get_all_match_data(&self)                  -> Result<Vec<MatchData>, DBRequestError>;
        fn get_all_pit_data  (&self)                  -> Result<Vec<PitData>,   DBRequestError>;
        fn get_match_data    (&self, identifier: u32) -> Result<MatchData,      DBRequestError>;
        fn get_pit_data      (&self, identifier: u32) -> Result<PitData,        DBRequestError>;
    }

    pub struct DebugDatabase {}

    impl DatabaseInterface for DebugDatabase 
    {
        fn get_server_info(&self) -> ServerInfo
        {
            ServerInfo{version: stringify!(0.1.0).to_string()}
        }

        fn set_match_data(&self, _data: MatchData) -> Result<(), DBRequestError>
        {
            println!("DEBUG: Set match data"); // add identifier
            Err(DBRequestError::NotImplemented)
        }
        fn set_pit_data(&self, _data: PitData)-> Result<(), DBRequestError>
        {
            println!("DEBUG: Set pit data"); // add identifier
            Err(DBRequestError::NotImplemented)
        }

        fn get_all_match_data(&self) -> Result<Vec<MatchData>, DBRequestError>
        {
            println!("DEBUG: Get all match data");
            Err(DBRequestError::NotImplemented)
        }

        fn get_all_pit_data(&self) -> Result<Vec<PitData>, DBRequestError>
        {
            println!("DEBUG: Get all pit data");
            Err(DBRequestError::NotImplemented)
        }

        fn get_match_data(&self, identifier: u32) -> Result<MatchData, DBRequestError>
        {
            println!("DEBUG: Get match data with identifier: {}", identifier);
            Err(DBRequestError::NotImplemented)
        }

        fn get_pit_data(&self, identifier: u32) -> Result<PitData, DBRequestError>
        {
            println!("DEBUG: Get pit data with identifier: {}", identifier);
            Err(DBRequestError::NotImplemented)
        }
    } // impl DatabaseInterface for DebugDatabase

    struct DatabaseReal
    {

    }

} // mod lib