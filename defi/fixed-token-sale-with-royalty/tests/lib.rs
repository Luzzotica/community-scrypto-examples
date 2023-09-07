#[cfg(test)]
mod tests {
		use scrypto::prelude::*;
		use scrypto_unit::*;
		use transaction::builder::ManifestBuilder;
		use fixed_price_sale_with_royalty::RoyaltyShare;

		#[test]
		fn test_fixed_token_sale_with_royalty() {
				// Set up environment.
				let mut test_runner = TestRunner::builder().build();

				// Create an account
				let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
				let (public_key2, _private_key2, account_component2) = test_runner.new_allocated_account();
				// let (public_key3, _private_key3, account_component3) = test_runner.new_allocated_account();

				// Publish package
				let package_address = test_runner.compile_and_publish(this_package!());

				// Create our NFTs
				let nfts1 = test_runner.create_non_fungible_resource(account_component);
				let nfts2 = test_runner.create_non_fungible_resource(account_component);

				// Instantiate the contract
        let transaction1 = ManifestBuilder::new()
          .withdraw_from_account(account_component, nfts1, dec!("1"))
          .withdraw_from_account(account_component, nfts2, dec!("2"))
          .take_all_from_worktop(nfts1, "nfts1")
          .take_all_from_worktop(nfts2, "nfts2")
          .call_function_with_name_lookup(
            package_address,
            "FixedPriceSaleWithRoyalty",
            "instantiate_fixed_price_sale_with_royalty",
            |lookup| (
              [
                lookup.bucket("nfts1"),
                lookup.bucket("nfts2"),
              ],
              XRD,
              dec!("1"),
              [
                // (account_component, dec!("0.1")),
                // (account_component2, dec!("0.1")),
                RoyaltyShare { account_component, percentage: dec!("0.1") },
                RoyaltyShare { account_component: account_component2, percentage: dec!("0.1") },
              ]
            )
          )
          .build();
				let receipt1 = test_runner.execute_manifest_ignoring_fee(
						transaction1,
						vec![NonFungibleGlobalId::from_public_key(&public_key)],
				);
				println!("{:?}\n", receipt1);
				receipt1.expect_commit_success();

				// let mut access_rules = BTreeMap::new();
				// access_rules.insert(ResourceMethodAuthKey::Withdraw, (rule!(allow_all), LOCKED));
				// access_rules.insert(ResourceMethodAuthKey::Deposit, (rule!(allow_all), LOCKED));
				
				// let nft_creation = ManifestBuilder::new()
				//     .create_non_fungible_resource(
				//         NonFungibleIdType::Integer, 
				//         Default::default(), 
				//         access_rules, 
				//         Some(
				//             [
				//                 (NonFungibleLocalId::integer(1), TestData { name: "NFT One".to_owned() }),
				//                 (NonFungibleLocalId::integer(2), TestData { name: "NFT Two".to_owned() })
				//             ]
				//         )
				//     )
				//     .call_method(
				//         account_component, 
				//         "deposit_batch", 
				//         manifest_args!(ManifestExpression::EntireWorktop)
				//     )
				//     .build();
				
				// let receipt = test_runner.execute_manifest_ignoring_fee(
				//     nft_creation,
				//     vec![NonFungibleGlobalId::from_public_key(&public_key)],
				// );
				// let receipt1 = test_runner.execute_manifest_ignoring_fee(
				//     transaction1,
				//     vec![NonFungibleGlobalId::from_public_key(&public_key)],
				// );
				// println!("{:?}\n", receipt1);
				// receipt1.expect_commit_success();

				// // Test the `buy_special_card` method.
				// let component = receipt1
				//     .expect_commit(true).new_component_addresses()[0];

				// let transaction2 = ManifestBuilder::new()
				//     .withdraw_from_account(account_component, RADIX_TOKEN,  dec!("666"))
				//     .take_from_worktop(RADIX_TOKEN, |builder, bucket| {
				//         builder.call_method(
				//             component,
				//             "buy_special_card",
				//             manifest_args!(NonFungibleLocalId::integer(2u64), bucket),
				//         )
				//     })
				//     .call_method(
				//         account_component,
				//         "deposit_batch",
				//         manifest_args!(ManifestExpression::EntireWorktop),
				//     )
				//     .build();
				// let receipt2 = test_runner.execute_manifest_ignoring_fee(
				//     transaction2,
				//     vec![NonFungibleGlobalId::from_public_key(&public_key)],
				// );
				// println!("{:?}\n", receipt2);
				// receipt2.expect_commit_success();

				// // Test the `buy_special_card` method.
				// let component = receipt1
				//     .expect_commit(true).new_component_addresses()[0];

				// let transaction3 = ManifestBuilder::new()
				//     .withdraw_from_account(account_component, RADIX_TOKEN, dec!("500"))
				//     .take_from_worktop(RADIX_TOKEN, |builder, bucket| {
				//         builder.call_method(component, "buy_random_card", manifest_args!(bucket))
				//     })
				//     .call_method(
				//         account_component,
				//         "deposit_batch",
				//         manifest_args!(ManifestExpression::EntireWorktop),
				//     )
				//     .build();
				// let receipt3 = test_runner.execute_manifest_ignoring_fee(
				//     transaction3,
				//     vec![NonFungibleGlobalId::from_public_key(&public_key)],
				// );
				// println!("{:?}\n", receipt3);
				// receipt3.expect_commit_success();
		}
}


