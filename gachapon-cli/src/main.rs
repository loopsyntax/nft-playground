use gachapon::*;
use ergo_node_interface::*;
use reqwest::blocking::get;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("ARGS: {}", args[1]);

  let node = acquire_node_interface_from_local_config();
  let block_height = node.current_block_height().unwrap();
  let artist_address = node.wallet_addresses().unwrap()[0].clone();

  // Acquire the ergs_box_for_fee
  let tx_fee = 10000000;
  let ergs_box_for_fee =
        get_ergs_box_for_fee(artist_address.clone(), tx_fee);
  println!("calling mint");
    let nft = NFTMetadata{name: "dummy_nft_1".to_string(),
                          description: "dummy description".to_string(),
                          asset_type: "text".to_string(),
                          url: "http://dummy_url".to_string(),
                          bytes: "dummy".as_bytes().to_vec(),
                          };
    println!("Preparing NFT minting..");
    println!("ergs box for fee :{}", ergs_box_for_fee.box_id());
   let unsigned_tx = GachaponProtocol::mint(nft,
                                            block_height, //current_height
                                            artist_address, // artist address

                                            tx_fee,
                           ergs_box_for_fee
   );
    println!("minting nft");
    let tx_id = node.sign_and_submit_transaction(&unsigned_tx).unwrap();
    println!("transaction submitted");
    println!("TX: {}", tx_id);
}

pub fn get_ergs_box_for_fee(user_address: String, tx_fee: u64) -> ErgsBox{
    let ergs_box_spec = ErgsBox::box_spec()
        .modified_address(Some(user_address))
        .modified_value_range(Some(tx_fee..u64::MAX));
    // Acquire the Ergo Explorer API endpoint in order to find
    // the our `ergs_box_for_bounty`.
    let ergs_box_url = ergs_box_spec
        //.explorer_endpoint("https://api-testnet.ergoplatform.com/api")
        .explorer_endpoint("https://api.ergoplatform.com/api")
        .unwrap();
    // Make a get request to the Ergo Explorer API endpoint
    let get_response = get(&ergs_box_url).unwrap().text().unwrap();
    // Process the `get_response` into `ErgsBox`es which match our
    // `ergs_box_for_bounty_spec`
    let list_of_ergs_boxes =
        ErgsBox::process_explorer_response_custom(&get_response, ergs_box_spec).unwrap();

    // Return the first `ErgsBox` from the list
    //println!("erg boxes: {}", list_of_ergs_boxes[0].ergo_box);
    list_of_ergs_boxes[0].clone()
    
}
