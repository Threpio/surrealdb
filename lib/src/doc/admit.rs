use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::dbs::Statement;
use crate::dbs::Transaction;
use crate::doc::Document;
use crate::err::Error;

impl<'a> Document<'a> {
	pub async fn admit(
		&self,
		_ctx: &Runtime,
		_opt: &Options,
		_txn: &Transaction,
		stm: &Statement<'_>,
	) -> Result<(), Error> {
		match self.id {
			Some(_) => Ok(()),
			None => match stm {
				Statement::Create(_) => Err(Error::CreateStatementError {
					value: (*self.initial).clone(),
				}),
				Statement::Update(_) => Err(Error::UpdateStatementError {
					value: (*self.initial).clone(),
				}),
				Statement::Relate(_) => Err(Error::RelateStatementError {
					value: (*self.initial).clone(),
				}),
				Statement::Delete(_) => Err(Error::DeleteStatementError {
					value: (*self.initial).clone(),
				}),
				Statement::Insert(_) => Err(Error::InsertStatementError {
					value: (*self.initial).clone(),
				}),
				_ => unreachable!(),
			},
		}
	}
}