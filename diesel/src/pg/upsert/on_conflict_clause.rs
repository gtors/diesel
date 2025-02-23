use super::on_conflict_actions::*;
use super::on_conflict_target::*;
use insertable::*;
use pg::Pg;
use query_builder::*;
use result::QueryResult;

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct OnConflictValues<Values, Target, Action> {
    values: Values,
    target: Target,
    action: Action,
}

impl<Values> OnConflictValues<Values, NoConflictTarget, DoNothing> {
    pub fn do_nothing(values: Values) -> Self {
        Self::new(values, NoConflictTarget, DoNothing)
    }
}

impl<Values, Target, Action> OnConflictValues<Values, Target, Action> {
    pub fn new(values: Values, target: Target, action: Action) -> Self {
        OnConflictValues {
            values,
            target,
            action,
        }
    }
}

impl<Values, Target, Action> CanInsertInSingleQuery<Pg> for OnConflictValues<Values, Target, Action>
where
    Values: CanInsertInSingleQuery<Pg>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        self.values.rows_to_insert()
    }
}

impl<Values, Target, Action> QueryFragment<Pg> for OnConflictValues<Values, Target, Action>
where
    Values: QueryFragment<Pg>,
    Target: QueryFragment<Pg>,
    Action: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        self.values.walk_ast(out.reborrow())?;
        out.push_sql(" ON CONFLICT");
        self.target.walk_ast(out.reborrow())?;
        self.action.walk_ast(out.reborrow())?;
        Ok(())
    }
}
