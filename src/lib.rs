use ergo_headless_dapp_framework::encoding::build_token;
pub use ergo_headless_dapp_framework::*;
use ergo_lib::ergotree_ir::chain::ergo_box::{ErgoBox, ErgoBoxCandidate};
use ergo_lib::ergotree_ir::chain::token::{Token, TokenAmount, TokenId};

/*
Metadata of an NFT
*/
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub asset_type: String,
    pub url: String,
    pub bytes: Vec<u8>,
}

impl NFTMetadata {
    pub fn hash(&self) -> String {
        return "some_hash".to_string();
    }
}

pub struct NFTBox {}

impl NFTBox {
    /*
    returns a box that can be used in a transaction to create a new NFT.
    A ergo NFT is a new Token whose amount is 1 with some metadata in its registers.
    this is based on the spec done by anon-real's auction house definition:
    https://github.com/anon-real/ErgoAuctionHouse/blob/master/src/auction/issueArtworkAssm.js#L29
    */
    pub fn candidate_box(
        fee: u64,
        metadata: NFTMetadata,
        address: &P2PKAddressString,
        current_height: BlockHeight,
        ergs_box_for_fee: &ErgsBox,
    ) -> Result<ErgoBoxCandidate> {
        let amount = 1;
        let decimals = 0;

        let tokens = vec![build_token(&ergs_box_for_fee.box_id(), amount).unwrap()];

        let registers = vec![
            // R4
            Constant::from(metadata.name.as_bytes().to_vec()),
            // R5
            Constant::from(metadata.description.as_bytes().to_vec()),
            // R6
            Constant::from(decimals),
            // R7
            Constant::from(metadata.asset_type.as_bytes().to_vec()),
            // R8
            Constant::from(metadata.hash().as_bytes().to_vec()),
            // R9
            Constant::from(metadata.url.as_bytes().to_vec()),
        ];

        println!("about to create candidate..");
        return create_candidate(fee, address, &tokens, &registers, current_height);
    }
}

pub struct GachaponProtocol {}

impl GachaponProtocol { 
    // returns an unsigned transaction to mint an nft
    pub fn mint(
        metadata: NFTMetadata,
        current_height: u64,
        artist_address: String,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
    ) -> UnsignedTransaction {
        println!("building nft box");
        let erg_in_box = 100000;
        let nft_box_candidate = NFTBox::candidate_box(
            erg_in_box,
            metadata,
            &artist_address,
            current_height,
            &ergs_box_for_fee,
        )
        .unwrap();

        let tx_inputs = vec![ergs_box_for_fee.as_unsigned_input()];

        let total_nano_ergs = ergs_box_for_fee.nano_ergs();
        let total_change = total_nano_ergs - erg_in_box - transaction_fee;

        let transaction_fee_candidate =
            TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();

        let change_box_candidate =
            ChangeBox::output_candidate(&vec![], total_change, &artist_address, current_height)
                .unwrap();

        let output_candidates = vec![
            nft_box_candidate,
            transaction_fee_candidate,
            change_box_candidate,
        ];

        println!("returning transaction");

        UnsignedTransaction::new(
            tx_inputs.try_into().unwrap(),
            None,
            output_candidates.try_into().unwrap(),
        )
        .unwrap()
    }
}
