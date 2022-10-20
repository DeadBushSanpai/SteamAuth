/// Represent a confirmation order
pub struct Confirmation {

	/// The ID of this confirmation.
	pub id: u64,

	    /// The unique key used to act upon this confirmation.
	    pub key: u64,

	    /// The value of the data-type HTML attribute returned for this contribution.
	    pub int_type: i32,

	    /// Represents either the Trade Offer ID or market transaction ID that caused this confirmation to be created.
	    pub creator: u64,

	    /// The type of this confirmation.
	    pub conf_type: ConfirmationType,
}

/// Public function
impl Confirmation {
/// Creates a new `Confirmation` containing the given value.
	pub fn new(id: u64, key: u64, int_type: i32, creator: u64) -> Confirmation {
		let conf_type: ConfirmationType = match int_type {
			1 => ConfirmationType::GenericConfirmation,
				2 => ConfirmationType::Trade,
				3 => ConfirmationType::MarketSellTransaction,
				_ => ConfirmationType::Unknown,
		};

		Confirmation {
			id,
				key,
				int_type,
				creator,
				conf_type,
		}
	}
}

/// Represent type of confirmation.
pub enum ConfirmationType {
	GenericConfirmation,
		Trade,
		MarketSellTransaction,
		Unknown
}
