use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::collections::LookupMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearGigs {
    gigs: LookupMap<u64, Gig>, // Map for storing each gig by ID
    gig_count: u64,            // Counter to assign gig IDs
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum GigStatus {
    Offered,
    Deposited,
    InProgress,
    Completed,
    Confirmed,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Gig {
    client: AccountId,
    freelancer: AccountId,
    amount: Balance,
    status: GigStatus,
    stage: u8,
}

#[near_bindgen]
impl NearGigs {
    #[init]
    pub fn new() -> Self {
        Self {
            gigs: LookupMap::new(b"g".to_vec()),
            gig_count: 0,
        }
    }

    // Create a new gig offer, setting the initial status
    pub fn create_gig(&mut self, freelancer: AccountId, amount: Balance) -> u64 {
        let client = env::predecessor_account_id();
        let gig_id = self.gig_count;
        self.gigs.insert(&gig_id, &Gig {
            client,
            freelancer,
            amount,
            status: GigStatus::Offered,
            stage: 1,
        });
        self.gig_count += 1;
        gig_id
    }

    // Trigger offer stage (1): Only the client can call
    pub fn offer(&mut self, gig_id: u64) {
        let mut gig = self.get_gig_or_panic(gig_id);
        self.assert_client(&gig);
        assert_eq!(gig.stage, 1, "Offer already made or incorrect stage.");
        gig.status = GigStatus::Offered;
        gig.stage = 2;
        self.gigs.insert(&gig_id, &gig);
    }

    // Trigger deposit stage (2): Only the client can call
    #[payable]
    pub fn deposit(&mut self, gig_id: u64) {
        let mut gig = self.get_gig_or_panic(gig_id);
        self.assert_client(&gig);
        assert_eq!(gig.stage, 2, "Gig not in deposit stage.");
        assert_eq!(env::attached_deposit(), gig.amount, "Deposit must match gig amount.");

        gig.status = GigStatus::Deposited;
        gig.stage = 3;
        self.gigs.insert(&gig_id, &gig);
    }

    // Trigger in-progress stage (3): Only the freelancer can call
    pub fn start_progress(&mut self, gig_id: u64) {
        let mut gig = self.get_gig_or_panic(gig_id);
        self.assert_freelancer(&gig);
        assert_eq!(gig.stage, 3, "Gig not in progress stage.");

        gig.status = GigStatus::InProgress;
        gig.stage = 4;
        self.gigs.insert(&gig_id, &gig);
    }

    // Trigger completed stage (4): Only the freelancer can call
    pub fn complete(&mut self, gig_id: u64) {
        let mut gig = self.get_gig_or_panic(gig_id);
        self.assert_freelancer(&gig);
        assert_eq!(gig.stage, 4, "Gig not in completed stage.");

        gig.status = GigStatus::Completed;
        gig.stage = 5;
        self.gigs.insert(&gig_id, &gig);
    }

    // Trigger confirm stage (5): Only the client can call; releases payment to freelancer
    pub fn confirm(&mut self, gig_id: u64) {
        let mut gig = self.get_gig_or_panic(gig_id);
        self.assert_client(&gig);
        assert_eq!(gig.stage, 5, "Gig not in confirm stage.");

        gig.status = GigStatus::Confirmed;
        gig.stage = 6;
        self.gigs.insert(&gig_id, &gig);
        Promise::new(gig.freelancer.clone()).transfer(gig.amount);
    }

    // Retrieve gig details by ID
    pub fn get_gig(&self, gig_id: u64) -> Option<Gig> {
        self.gigs.get(&gig_id)
    }

    // Private helper methods

    // Get a gig by ID or panic if it does not exist
    fn get_gig_or_panic(&self, gig_id: u64) -> Gig {
        self.gigs.get(&gig_id).expect("Gig does not exist.")
    }

    // Assert only the client can call certain stages
    fn assert_client(&self, gig: &Gig) {
        assert_eq!(env::predecessor_account_id(), gig.client, "Only the client can call this.");
    }

    // Assert only the freelancer can call certain stages
    fn assert_freelancer(&self, gig: &Gig) {
        assert_eq!(env::predecessor_account_id(), gig.freelancer, "Only the freelancer can call this.");
    }
}
