use crate::ownershipvoucher::OwnershipVoucher;

pub struct accept {
    guid: Vec<OwnershipVoucher>,
}

pub struct failure {
    code: u8,
    details: error_details,
}

pub struct error_details(&failure);