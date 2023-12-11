# dfx deploy --network ic  cap
# dfx deploy --network ic   dmail_nft --argument "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\", \"tkn\", \"token\", principal \"45elo-3aaaa-aaaak-aafwa-cai\")"

#  dfx canister  --network ic  call aaaaa-aa update_settings "(
#   record {
#     canister_id=principal \"$(dfx canister --network ic  id cap)\";
#     settings=record {
#       controllers=opt vec{
#         principal\"$(dfx identity --network ic get-principal)\";
#         principal \"$(dfx canister --network ic id cap)\";
#         principal \"$(dfx canister --network ic id dmail_nft)\";
#         principal \"$(dfx identity --network ic get-wallet)\";
#         principal \"xfed4-4yaaa-aaaak-aapya-cai\";
#         principal \"wyj5p-yqaaa-aaaak-acj2q-cai\";
#         principal \"ea7tw-yaaaa-aaaak-aam6q-cai\";
#       }
#     }
#   }
# )"

# dfx canister --network ic call dmail_nft bind "(\"24580\",principal \"dogas-pkjqh-smbhg-kgsa3-wwu7r-rm6yh-mzbtn-qsw3f-54lnt-i37xe-hae\")"
# dfx canister --network ic call dmail_nft getMetadataForUserDip721 "(principal \"dogas-pkjqh-smbhg-kgsa3-wwu7r-rm6yh-mzbtn-qsw3f-54lnt-i37xe-hae\")"
# dfx canister --network ic call dmail_nft check_mint_bind "(principal \"dogas-pkjqh-smbhg-kgsa3-wwu7r-rm6yh-mzbtn-qsw3f-54lnt-i37xe-hae\",\"21621\")"
dfx canister --network ic call dmail_nft getMetadataForUserDip721 "(principal \"w7bg7-j236m-2av3e-glqyw-endkg-jsjj6-rjhmo-oowfm-55hql-avp6r-dae\")"
# dfx canister --network ic call dmail_nft single_mint_nft_to_market "(
#     principal \"6cjya-hnrph-ohcs7-pt7m4-dxzwj-zax3s-b5f2w-n472d-mulap-7jaeg-2ae\",
#     23998,
#     \"9877808225\",
#     false
# )"
# dfx canister --network ic call dmail_nft batch_mint_nft_to_market "()"

# dfx canister --network ic call dmail_nft display_all_token "()"
dfx canister --network ic call dmail_nft query_pid_by_alias "(\"1111233\")"
# dfx canister --network ic call dmail_nft getTokenIdsForUserDip721 "(principal \"xti4z-a33ih-dbmiz-buj73-uwjlv-dlpyw-iiojj-p6cte-qjcm2-ira2l-2qe\")"
# dfx canister   --network ic  call dmail_nft  bind "( \"9505\", principal \"2e2zr-ylqej-iruuu-ugkd3-bmv6e-cvceo-davpx-6lbbm-vyfh4-ply3e-qae\")"
# echo poapdmail_nftci/d
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
