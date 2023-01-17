use crate::schema::TaskEncoder;
use crate::schema::TaskVersioning;
use sqlx::postgres::PgConnection;

//impl TaskEncoder for PgConnection {
//    fn encode_task(&mut self, task:TaskVersioning){
//        match task {
//            TaskVersioning::V1(task)=>{
//                query!("SELECT id FROM pogo_tasks WHERE id=''")
//            }
//        }
//    }
//}
