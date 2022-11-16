use diesel::backend::Backend;
use diesel::expression::is_aggregate;
use diesel::expression::MixedAggregates;
use diesel::expression::ValidGrouping;
use diesel::sql_types::Bool;
use diesel::sql_types::Text;
use diesel::AppearsOnTable;
use diesel::BoxableExpression;
use diesel::Expression;
use diesel::TextExpressionMethods;

mod schema;

enum StringFilter {
    Equals(String),
    StartsWith(String),
}

fn string_filter<T, C, DB>(
    column: C,
    filter: StringFilter,
) -> Box<dyn BoxableExpression<T, DB, SqlType = Bool>>
where
    DB: Backend,
    C: TextExpressionMethods + Expression<SqlType = Text> + AppearsOnTable<T> + ValidGrouping<()>,
    is_aggregate::No: MixedAggregates<C::IsAggregate, Output = is_aggregate::No>,
{
    match filter {
        StringFilter::Equals(other) => Box::new(column.eq(other)),
        StringFilter::StartsWith(other) => Box::new(column.like(format!("{}%", other))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut statement = crate::schema::users::table.into_boxed();
        let filter = StringFilter::Equals("John Doe".to_string());
        statement = statement.filter(string_filter(crate::schema::users::name, filter));
    }
}
