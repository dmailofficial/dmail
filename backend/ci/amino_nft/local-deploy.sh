# dfx deploy amino_nft --mode=reinstall --argument "(principal \"$(dfx identity get-principal)\", \"tkn\", \"token\", principal \"rno2w-sqaaa-aaaaa-aaacq-cai\")"

# dfx canister call aaaaa-aa update_settings "(
#     record {
#       canister_id=principal \"$(dfx canister id amino_nft)\";
#       settings=record {
#         controllers=opt vec {
#           principal\"$(dfx canister id amino_nft)\";
#           principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#         }
#       }
#     }
#   )"



# dfx canister call amino_nft getTokenIdsForUserDip721 "(principal \"p4z6k-mmtgs-7y5cm-bgv35-efsgp-7ktr4-ue4oz-igkdo-taprh-oje5v-zae\")"
# dfx canister call amino_nft getTokenIdsForUserDip721 "(principal \"p4z6k-mmtgs-7y5cm-bgv35-efsgp-7ktr4-ue4oz-igkdo-taprh-oje5v-zae\")"
dfx canister call amino_nft transferFromDip721 "( principal \"p4z6k-mmtgs-7y5cm-bgv35-efsgp-7ktr4-ue4oz-igkdo-taprh-oje5v-zae\"  , principal \"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\"  ,1)"
dfx canister call amino_nft getMetadataForUserDip721 "( principal \"p4z6k-mmtgs-7y5cm-bgv35-efsgp-7ktr4-ue4oz-igkdo-taprh-oje5v-zae\")"
# dfx canister call amino_nft getMetadataForUserDip721 "( principal \"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\")"

# dfx canister call amino_nft getMetadataDip721 "(9505)"

# dfx canister call amino_nft query_pid_by_alias "asd1111"
# dfx canister call amino_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call amino_nft query_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"

# dfx canister call amino_nft bind "(\"9500\",principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call amino_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call amino_nft unbind "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call amino_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call amino_nft query_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
