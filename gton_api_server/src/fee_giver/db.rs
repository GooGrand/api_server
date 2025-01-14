use crate::schema::voters;
use diesel::prelude::*;
use actix_web_dev::error::{
    Result,
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Queryable,Deserialize,Serialize,Clone)]
pub struct Voter {
    pub id: i32,
    pub round_id: i32,
    pub user_address: String,
    pub vote_times: i32,
}

#[derive(Insertable,Serialize,Deserialize)]
#[table_name = "voters"]
pub struct VoterInstance {
    pub round_id: i32,
    pub user_address: String,
}

#[derive(QueryableByName,Serialize,Deserialize)]
pub struct Allowance {
    #[sql_type="diesel::sql_types::Bool"]
    res: bool,
}

impl VoterInstance {
    pub async fn check(
        &self,
        conn: &PgConnection, 
    ) -> Result<bool> {
        diesel::sql_query("call GiveFee($1,$2)")
            .bind::<diesel::sql_types::Varchar,_>(self.user_address.clone())
            .bind::<diesel::sql_types::Integer,_>(self.round_id)
            .get_result::<Allowance>(conn)
            .map_err(|e|e.into())
            .map(|r|r.res)
    }

    pub async fn get_times(
        &self,
        conn: &PgConnection, 
    ) -> Result<Voter> {
        voters::table
            .filter(voters::round_id.eq(self.round_id))
            .filter(voters::user_address.eq(self.user_address.clone()))
            .get_result(conn)
            .map_err(|e|e.into())
    }
}


