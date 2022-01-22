pub use ergo_headless_dapp_framework::*;

#[derive(Debug, Clone, WrapBox, SpecBox)]
pub struct NFTBox {
    ergo_box: ErgoBox,
}

impl SpecifiedBox for NFTBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}

pub fn upload_img(){
    println!("uploading image")
}

pub struct GachaponProtocol {}

impl GachaponProtocol {

    fn nft_issue_candidate() {

    }

    pub fn claim(){
        // given an NFTBox it claims it
    }

    pub fn pull_lever(){
        // claims a random NFTbox
    }

    pub fn mint_many() {
        // mint many nfts in a single transaction
    }

    pub fn mint(name: String, description: String,
                royalty: f32, start_gacha_height: u64,
                artist_fee: u64, current_height: u64,
                artist_address: String, transaction_fee: u64,
                ergs_box_for_fee: ErgsBox) -> UnsignedTransaction {

         let tx_inputs = vec![
            ergs_box_for_fee.as_unsigned_input(),
        ];

        let nft_candidate = create_candidate(
            1,
            &"94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string(),
            &vec![],
            &vec![],
            current_height,
        ) .unwrap();

        let total_nano_ergs = ergs_box_for_fee.nano_ergs();
        let total_change = total_nano_ergs  - transaction_fee;

        let change_box_candidate =
            ChangeBox::output_candidate(&vec![],
                                        total_change,
                                        &artist_address,
                                        current_height)
                .unwrap();

        let output_candidates = vec![
            nft_candidate,
            change_box_candidate,
        ];

        UnsignedTransaction::new(
            tx_inputs.try_into().unwrap(),
            None,
            output_candidates.try_into().unwrap(),
        )
        .unwrap()

    }
}
