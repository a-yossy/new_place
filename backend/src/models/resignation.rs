use std::collections::HashMap;

use anyhow::{Context, Result as AnyhowResult, anyhow};
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, Weekday};
use sqlx::{MySql, Pool, Result};

use crate::utils::time::now;

pub struct Resignation {
    pub id: i32,
    pub retirement_date: NaiveDate,
    pub remaining_paid_leave_days: u32,
    pub created_at: NaiveDateTime,
}

pub struct ResignationInput {
    pub retirement_date: NaiveDate,
    pub remaining_paid_leave_days: u32,
}

impl Resignation {
    pub async fn fetch_latest(pool: &Pool<MySql>) -> Result<Self> {
        let latest_resignation = sqlx::query_as!(
            Self,
            r#"
                SELECT
                    id, retirement_date, remaining_paid_leave_days, created_at
                FROM
                    resignation
                ORDER BY
                    created_at DESC
                LIMIT 1
            "#
        )
        .fetch_one(pool)
        .await?;

        Ok(latest_resignation)
    }

    pub async fn insert(pool: &Pool<MySql>, input: &ResignationInput) -> Result<Resignation> {
        let now = now();
        let id = sqlx::query!(
            r#"
            INSERT INTO
                resignation (retirement_date, remaining_paid_leave_days, created_at)
            VALUES
                (?, ?, ?)
            "#,
            input.retirement_date.to_string(),
            input.remaining_paid_leave_days,
            now.format("%Y-%m-%d %H:%M:%S").to_string()
        )
        .execute(pool)
        .await?
        .last_insert_id();

        let resignation = sqlx::query_as!(
            Self,
            r#"
            SELECT
                id, retirement_date, remaining_paid_leave_days, created_at
            FROM
                resignation
            WHERE
                ID = ?
        "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(resignation)
    }

    pub async fn vacation_start_date(
        &self,
        holidays: &HashMap<String, String>,
    ) -> AnyhowResult<NaiveDate> {
        let mut vacation_start_date = self.retirement_date;
        let mut remaining_paid_leave_days = self.remaining_paid_leave_days;

        if remaining_paid_leave_days == 0 {
            return Err(anyhow!("有給がありません"));
        }

        remaining_paid_leave_days -= 1;

        while remaining_paid_leave_days > 0 {
            let new_date = vacation_start_date
                .checked_sub_days(Days::new(1))
                .context("invalid date")?;
            vacation_start_date = new_date;
            if holidays.contains_key(&new_date.to_string()) {
                continue;
            }
            if new_date.weekday() == Weekday::Sat {
                continue;
            }
            if new_date.weekday() == Weekday::Sun {
                continue;
            }

            remaining_paid_leave_days -= 1;
        }

        Ok(vacation_start_date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};
    use sqlx::MySqlPool;

    use crate::models::resignation::Resignation;

    #[sqlx::test(fixtures("../fixtures/resignation/resignations.sql"))]
    async fn fetch_latest(pool: MySqlPool) {
        let resignation = Resignation::fetch_latest(&pool).await.unwrap();

        assert_eq!(resignation.id, 2222);
        assert_eq!(
            resignation.retirement_date,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(resignation.remaining_paid_leave_days, 5);
        assert_eq!(
            resignation.created_at,
            NaiveDateTime::parse_from_str("2025-02-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[sqlx::test]
    async fn insert(pool: MySqlPool) {
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(resignations.len(), 0);
        let input = ResignationInput {
            retirement_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            remaining_paid_leave_days: 10,
        };

        let result = Resignation::insert(&pool, &input).await;

        assert!(result.is_ok());
        let resignation = result.unwrap();
        assert_eq!(
            resignation.retirement_date,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(resignation.remaining_paid_leave_days, 10);
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(resignations.len(), 1);
        assert_eq!(
            resignations.first().unwrap().retirement_date,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(resignations.first().unwrap().remaining_paid_leave_days, 10);
    }
}
