use std::env;
// use dotenv::;
// use std::env;
use ::breez_sdk_spark::*;
use dotenv::dotenv;

#[tokio::main]
pub async fn walletconnect() -> Result<(), breez_sdk_spark::SdkError> {
    dotenv().ok();
    // Construct the seed using mnemonic words or entropy bytes
    let mnemonic = env::var("SEED_PHRASE".to_string()).unwrap();
    let seed = Seed::Mnemonic {
        mnemonic,
        passphrase: None,
    };

    // Create the default config
    let mut config = default_config(Network::Mainnet);
    config.api_key = Some(dotenv::var("API_KEY".to_string()).unwrap());

    // Connect to the SDK using the simplified connect method
    let sdk = connect(ConnectRequest {
        config,
        seed,
        storage_dir: "./.data".to_string(),
    })
    .await?;
    let info = sdk
        .get_info(GetInfoRequest {
            // ensure_synced: true will ensure the SDK is synced with the Spark network
            // before returning the balance
            ensure_synced: Some(true),
        })
        .await?;
    let balance = info.balance_sats;
    // let description = "<invoice description>".to_string();
    // // Optionally set the invoice amount you wish the payer to send
    // let optional_amount_sats = Some(10);

    // let response = sdk
    //     .receive_payment(ReceivePaymentRequest {
    //         payment_method: ReceivePaymentMethod::Bolt11Invoice {
    //             description,
    //             amount_sats: optional_amount_sats,
    //         },
    //     })
    //     .await?;

    // // let payment_request = response.payment_request;
    // // println!("Payment request: {payment_request}");
    // // let receive_fee_sats = response.fee;
    // // println!("Fees: {receive_fee_sats} sats");
    // let request = ListUnclaimedDepositsRequest {};
    // let response = sdk.list_unclaimed_deposits(request).await?;

    // for deposit in response.deposits {
    //     println!("Unclaimed deposit: {}:{}", deposit.txid, deposit.vout);
    //     println!("Amount: {} sats", deposit.amount_sats);

    //     if let Some(claim_error) = &deposit.claim_error {
    //         match claim_error {
    //             DepositClaimError::MaxDepositClaimFeeExceeded {
    //                 max_fee,
    //                 required_fee_sats,
    //                 required_fee_rate_sat_per_vbyte,
    //                 ..
    //             } => {
    //                 println!(
    //                     "Max claim fee exceeded. Max: {:?}, Required: {} sats or {} sats/vByte",
    //                     max_fee, required_fee_sats, required_fee_rate_sat_per_vbyte
    //                 );
    //             }
    //             DepositClaimError::MissingUtxo { .. } => {
    //                 println!("UTXO not found when claiming deposit");
    //             }
    //             DepositClaimError::Generic { message } => {
    //                 println!("Claim failed: {}", message);
    //             }
    //         }
    //     }
    // }
    dbg
    !("My balance in sats {}", balance);

    Ok(())
}
