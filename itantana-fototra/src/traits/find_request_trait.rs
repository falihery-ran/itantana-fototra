use std::fmt::Debug;

use crate::dtos::error::find_request_error::FindRequestError;

pub trait FindRequestTrait: Sync + Send + 'static {
    type Filter: Clone + Debug;
    fn validate_per_page(per_page: &u16) -> Result<&u16, FindRequestError> {
        if per_page < &1 {
            return Err(FindRequestError::PerPageValueTooLow {
                per_page: *per_page,
            });
        } else if per_page > &1000 {
            return Err(FindRequestError::PerPageValueTooHigh {
                per_page: *per_page,
            });
        }
        Ok(per_page)
    }

    fn validate_offset(offset: &u64) -> Result<&u64, FindRequestError> {
        if offset < &1 {
            return Err(FindRequestError::PerPageOffsetTooLow { offset: *offset });
        }
        Ok(offset)
    }

    fn set_filters(&mut self, filters: &Self::Filter);

    fn set_per_page(&mut self, per_page: &u16);

    fn set_offset(&mut self, offset: &u64);
}
