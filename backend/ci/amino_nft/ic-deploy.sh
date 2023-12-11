# dfx deploy --network ic  cap
# dfx deploy --network ic amino_nft --argument "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\", \"tkn\", \"token\", principal \"45elo-3aaaa-aaaak-aafwa-cai\")"

# dfx canister --network ic call aaaaa-aa update_settings "(
#   record {
#     canister_id=principal \"4phuj-zqaaa-aaaak-acp7a-cai\";
#     settings=record {
#       controllers=opt vec{
#         principal\"$(dfx identity --network ic get-principal)\";
#         principal \"$(dfx canister --network ic id amino_nft)\";
#         principal \"$(dfx identity --network ic get-wallet)\";
#         principal\"6cjya-hnrph-ohcs7-pt7m4-dxzwj-zax3s-b5f2w-n472d-mulap-7jaeg-2ae\";
#       }
#     }
#   }
# )"

# dfx canister --network ic call amino_nft getMetadataForUserDip721 "(principal \"vn5x4-rxwxv-ktg2m-b2atg-7onbh-ih6sl-7j6vq-li6su-x4poh-icfei-jae\")"
# dfx canister --network ic call amino_nft single_mint_nft_to_market "(
#     principal \"vn5x4-rxwxv-ktg2m-b2atg-7onbh-ih6sl-7j6vq-li6su-x4poh-icfei-jae\",
#     23992,
#     \"13888\",
#     false
# )"
# dfx canister  --network ic   call amino_nft  batch_mint_nft_to_market "()"
dfx canister --network ic call amino_nft getMetadataForUserDip721 "( principal \"m3ba5-z2qpf-ccobt-cky6r-ekgbk-mu4tv-yfs2r-aqevv-2dzaq-7lf5p-iqe\")"
# dfx canister --network ic call amino_nft getTokenIdsForUserDip721 "(principal \"sh63v-kxuhp-47ese-qfbkd-aypkm-7qgi7-k77t6-5lzk6-5s44p-hf2hi-7qe\")"
# dfx canister --network ic call amino_nft getTokenIdsForUserDip721 "(principal \"xti4z-a33ih-dbmiz-buj73-uwjlv-dlpyw-iiojj-p6cte-qjcm2-ira2l-2qe\")"
# dfx canister   --network ic  call amino_nft  bind "( \"9505\", principal \"2e2zr-ylqej-iruuu-ugkd3-bmv6e-cvceo-davpx-6lbbm-vyfh4-ply3e-qae\")"
# echo poapamino_nft
# dfx canister    --network ic    call 42fn2-wyaaa-aaaak-aafwq-cai mintDip721 "(
#   principal \"6cjya-hnrph-ohcs7-pt7m4-dxzwj-zax3s-b5f2w-n472d-mulap-7jaeg-2ae\",
#   vec {record {
#     data=vec {};
#     key_val_data=vec {
#       record {
#         key=\"alias\";
#         val=variant {TextContent=\"stest00000001\"}};
#         record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#         purpose=variant { Rendered }
#       }
#     }
#   )"
