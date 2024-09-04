#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(account_id))]
#[allow(non_camel_case_types)]
#[cfg(target_os = "linux")]
pub struct CarryOverBalance {
    pub account_id: i64,
    pub debit: BigDecimal,
    pub description: Option<String>,
    pub description2: Option<Vec<String>>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(id1, id2))]
#[allow(non_camel_case_types)]
#[cfg(target_os = "linux")]
pub struct Order {
    pub id1: i64,
    pub id2: i64,
    pub time: NaiveDateTime,
    pub time2: DateTime<Utc>,
    pub json: String,
}
