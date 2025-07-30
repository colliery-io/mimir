//! Custom types for JSON fields in the database

use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::io::Write;

/// Macro to implement Diesel traits for JSON types
macro_rules! impl_json_type {
    ($type:ty) => {
        impl ToSql<Text, Sqlite> for $type {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
                let json = serde_json::to_string(self)?;
                out.write_all(json.as_bytes())?;
                Ok(IsNull::No)
            }
        }

        impl FromSql<Text, Sqlite> for $type {
            fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
                let text = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
                serde_json::from_str(&text).map_err(|e| e.into())
            }
        }

        impl AsExpression<Text> for $type {
            type Expression = diesel::expression::bound::Bound<Text, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a> AsExpression<Text> for &'a $type {
            type Expression = diesel::expression::bound::Bound<Text, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }
    };
}

// Export the macro for use in submodules
pub(crate) use impl_json_type;

// Common JSON types will be added as needed