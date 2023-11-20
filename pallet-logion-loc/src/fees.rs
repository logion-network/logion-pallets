use logion_shared::Beneficiary;
use sp_runtime::Percent;

use crate::{mock::*, Config};

/// Balances of accounts involved in fees payment
pub struct BalancesSnapshot {
    pub payer_account: AccountId,
    pub payer: Balance,
    pub payer_reserved: Balance,
    pub logion_treasury: Balance,
    pub community_treasury: Balance,
    pub legal_officers: Vec<(AccountId, Balance)>,
}
impl BalancesSnapshot {
    pub fn take(
        payer_account: AccountId,
        legal_officer_accounts: Vec<AccountId>,
    ) -> BalancesSnapshot {
        BalancesSnapshot {
            payer_account,
            payer: Self::get_free_balance(payer_account),
            payer_reserved: Self::get_reserved_balance(payer_account),
            logion_treasury: Self::get_free_balance(LOGION_TREASURY_ACCOUNT_ID),
            community_treasury: Self::get_free_balance(COMMUNITY_TREASURY_ACCOUNT),
            legal_officers: legal_officer_accounts
                .iter()
                .map(|legal_officer_account| {
                    (
                        legal_officer_account.clone(),
                        Self::get_free_balance(legal_officer_account.clone()),
                    )
                })
                .collect(),
        }
    }

    fn get_free_balance(account_id: AccountId) -> Balance {
        <Test as Config>::Currency::free_balance(account_id)
    }

    fn get_reserved_balance(account_id: AccountId) -> Balance {
        <Test as Config>::Currency::reserved_balance(account_id)
    }

    pub fn delta_since(&self, previous: &BalancesSnapshot) -> BalancesDelta {
        BalancesDelta {
            payer: previous.payer.saturating_sub(Self::get_free_balance(previous.payer_account)),
            payer_reserved: previous.payer_reserved.saturating_sub(Self::get_reserved_balance(previous.payer_account)),
            logion_treasury: Self::get_free_balance(LOGION_TREASURY_ACCOUNT_ID).saturating_sub(previous.logion_treasury),
            community_treasury: Self::get_free_balance(COMMUNITY_TREASURY_ACCOUNT).saturating_sub(previous.community_treasury),
            legal_officers: previous.legal_officers.iter()
                .map(|legal_officer| Self::get_free_balance(legal_officer.0.clone()).saturating_sub(legal_officer.1.clone()))
                .collect(),
        }
    }
}

/// Amounts that were credited or debited between 2 snapshots
pub struct BalancesDelta {
    /// Debited amount or 0 if credited
    payer: u128,
    /// Debited amount from community_treasury or 0 if credited
    payer_reserved: u128,
    /// Credited amount or 0 if debited
    logion_treasury: u128,
    /// Credited amount or 0 if debited
    community_treasury: u128,
    /// Credited amount or 0 if debited
    legal_officers: Vec<u128>,
}
impl BalancesDelta {
    pub fn total_credited(&self) -> u128 {
        self.logion_treasury + self.community_treasury + self.legal_officers.iter().sum::<u128>()
    }

    pub fn total_debited(&self) -> u128 {
        self.payer
    }

    pub fn total_debited_reserve(&self) -> u128 {
        self.payer_reserved
    }
}

/// Other fees than inclusion
pub struct Fees {
    pub storage_fees: Balance,
    pub legal_fees: Balance,
    /// When legal_fees is > 0, legal_fee_beneficiary must be some; should be none otherwise
    pub fee_beneficiary: Option<Beneficiary<AccountId>>,
    pub certificate_fees: Balance,
    pub value_fee: Balance,
    pub collection_item_fee: Balance,
    pub tokens_record_fee: Balance,
}
impl Fees {

    pub fn total(&self) -> Balance {
        self.storage_fees
            + self.legal_fees
            + self.certificate_fees
            + self.value_fee
            + self.collection_item_fee
            + self.tokens_record_fee
    }

    pub fn only_storage(num_of_files: u32, tot_size: u32) -> Fees {
        Fees {
            certificate_fees: 0,
            legal_fees: 0,
            storage_fees: Self::storage_fees(num_of_files, tot_size),
            fee_beneficiary: None,
            value_fee: 0,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }
    }

    pub fn only_storage_and_tokens_record(num_of_files: u32, tot_size: u32, fee: Balance, beneficiary: Beneficiary<AccountId>) -> Fees {
        Fees {
            certificate_fees: 0,
            legal_fees: 0,
            storage_fees: Self::storage_fees(num_of_files, tot_size),
            fee_beneficiary: Some(beneficiary),
            value_fee: 0,
            collection_item_fee: 0,
            tokens_record_fee: fee,
        }
    }

    pub fn only_collection_item(fee: Balance, beneficiary: Beneficiary<AccountId>) -> Fees {
        Fees {
            certificate_fees: 0,
            legal_fees: 0,
            storage_fees: 0,
            fee_beneficiary: Some(beneficiary),
            value_fee: 0,
            collection_item_fee: fee,
            tokens_record_fee: 0,
        }
    }

    pub fn storage_fees(num_of_files: u32, tot_size: u32) -> Balance {
        let entry_fee: Balance = Into::<Balance>::into(num_of_files) * Into::<Balance>::into(FileStorageEntryFee::get());
        let storage_fee: Balance = Into::<Balance>::into(tot_size) * Into::<Balance>::into(FileStorageByteFee::get());
        entry_fee + storage_fee
    }

    pub fn only_legal(fee: Balance, beneficiary: Beneficiary<AccountId>) -> Fees {
        Fees {
            certificate_fees: 0,
            legal_fees: fee,
            storage_fees: 0,
            fee_beneficiary: Some(beneficiary),
            value_fee: 0,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }
    }

    pub fn assert_balances_events(&self, previous_balances: BalancesSnapshot) {
        let expected_fees_total = self.total();

        let current_balances = BalancesSnapshot::take(
            previous_balances.payer_account,
            previous_balances.legal_officers.iter()
                .map(|legal_officer| legal_officer.0.clone())
                .collect(),
        );
        let balances_delta = current_balances.delta_since(&previous_balances);
        let credited_fees: Balance = balances_delta.total_credited();
        assert_eq!(credited_fees, expected_fees_total);

        let debited_fees = balances_delta.total_debited() + balances_delta.total_debited_reserve();
        assert_eq!(debited_fees, expected_fees_total);

        if self.storage_fees > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::StorageFeeWithdrawn {
                0: previous_balances.payer_account,
                1: self.storage_fees,
            }));
        }

        if self.legal_fees > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::LegalFeeWithdrawn {
                0: previous_balances.payer_account,
                1: self.fee_beneficiary.unwrap(),
                2: self.legal_fees,
            }));
        }

        if self.certificate_fees > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::CertificateFeeWithdrawn {
                    0: previous_balances.payer_account,
                    1: self.certificate_fees,
            }));
        }

        if self.value_fee > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::ValueFeeWithdrawn {
                0: previous_balances.payer_account,
                1: self.value_fee,
            }));
        }

        let percent_to_loc_owner: Percent = RecurentFeeDistributionKey::get().loc_owner_percent;
        if self.collection_item_fee > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::CollectionItemFeeWithdrawn {
                    0: previous_balances.payer_account,
                    1: self.collection_item_fee,
                    2: self.fee_beneficiary.unwrap(),
                    3: percent_to_loc_owner * self.collection_item_fee,
            }));
        }

        if self.tokens_record_fee > 0 {
            System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::TokensRecordFeeWithdrawn {
                    0: previous_balances.payer_account,
                    1: self.tokens_record_fee,
                    2: self.fee_beneficiary.unwrap(),
                    3: percent_to_loc_owner * self.tokens_record_fee,
            }));
        }
    }
    
}
