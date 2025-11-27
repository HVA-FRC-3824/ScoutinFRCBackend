pub mod lib
{
enum DBRequestStatus
{
    SUCCESS,
    ERROR,
}

impl DBRequestStatus {}

trait DatabaseInterface
{
    fn create_db()            -> DBRequestStatus;
    fn sanitzed_sql_request() -> DBRequestStatus;
}

struct DebugDatabase
{
}

impl DatabaseInterface for DebugDatabase 
{
    fn create_db() -> DBRequestStatus
    {
        DBRequestStatus::SUCCESS
    }

    fn sanitzed_sql_request() -> DBRequestStatus
    {
        DBRequestStatus::SUCCESS
    }
}

struct Api
{
    db_interface: &impl DatabaseInterface,
}

impl Api 
{
    fn new<T: &impl DatabaseInterface>(instance: T) -> Self
    {
        Api{db_instance: interface}
    }

    fn debug_new() -> Self
    {
        Api(DebugDatabase())
    }
}
}