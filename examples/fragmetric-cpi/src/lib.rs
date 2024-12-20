extern crate core;

anchor_gen::generate_cpi_crate!("idl-v0.3.2.json");

// devnet program: frag9zfFME5u1SNhUYGa4cXLzMKgZXF3xwZ2Y1KCYTQ, mainnet program: fragnAis7Bp6FTsMoa6YcH8UffhEw43Ph79qAiK3iF3
anchor_lang::declare_id!("frag9zfFME5u1SNhUYGa4cXLzMKgZXF3xwZ2Y1KCYTQ");

#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use anchor_lang::{pubkey, AnchorDeserialize, Discriminator};

    #[test]
    fn parse_account() {
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
        let account_data = rpc.get_account_data(&pubkey!("36znkkBhTNJY6PzidFN7vwuZysbn8P8hz4LBk3AZn33Z")).unwrap();
        let account_data_slice = &mut account_data.as_slice();
        assert_eq!(crate::account::NormalizedTokenPoolAccount::discriminator(), account_data_slice[0..8]);

        let account_data_slice_without_discriminator = &mut &account_data_slice[8..];
        let ntp = crate::account::NormalizedTokenPoolAccount::deserialize(account_data_slice_without_discriminator).unwrap();
        println!("ntp: {:?}", ntp);
    }
}