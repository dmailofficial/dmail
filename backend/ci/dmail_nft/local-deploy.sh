# dfx deploy dmail_nft --argument "(principal \"$(dfx identity get-principal)\", \"tkn\", \"token\", principal \"rno2w-sqaaa-aaaaa-aaacq-cai\")"

# dfx canister call aaaaa-aa update_settings "(
#     record {
#       canister_id=principal \"$(dfx canister id dmail_nft)\";
#       settings=record {
#         controllers=opt vec {
#           principal\"$(dfx canister id dmail_nft)\";
#           principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#         }
#       }
#     }
#   )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"bin1222333\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"cheng1\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

    dfx canister call dmail_nft mintDip721 "(
    principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",
    vec {record {
      data=vec {};
      key_val_data=vec {
        record {
          key=\"alias\";val=variant {TextContent=\"90000001\"}};
          record {key=\"binding\"; val=variant {TextContent=\"true\"}}};
          purpose=variant { Rendered }
        }
      }
    )"

    dfx canister call dmail_nft mintDip721 "(
    principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",
    vec {record {
      data=vec {};
      key_val_data=vec {
        record {
          key=\"alias\";val=variant {TextContent=\"90000002\"}};
          record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
          purpose=variant { Rendered }
        }
      }
    )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"dong\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"dong1\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"dong2\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"true\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

# dfx canister call dmail_nft mintDip721 "(
#     principal\"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\",
#     vec {record {
#       data=vec {};
#       key_val_data=vec {
#         record {
#           key=\"alias\";val=variant {TextContent=\"dong3\"}};
#           record {key=\"binding\"; val=variant {TextContent=\"false\"}}};
#           purpose=variant { Rendered }
#         }
#       }
#     )"

# dfx canister call dmail_nft getTokenIdsForUserDip721 "(principal \"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\")"
# dfx canister call dmail_nft getTokenIdsForUserDip721 "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft getMetadataForUserDip721 "( principal \"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\"   )"
#  dfx canister  call dmail_nft  batch_mint_nft_to_market "()"

# dfx canister call dmail_nft transferFromDip721 "( principal \"j6ypl-bscrm-cyclq-ltrzf-eypfa-ydnw2-yo5e2-ebyep-xaxsz-vwtdd-sqe\"  , principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\"  ,9505)"
# dfx canister call dmail_nft getMetadataDip721 "(9505)"

# dfx canister call dmail_nft query_pid_by_alias "asd1111"
# dfx canister call dmail_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft query_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"

# dfx canister call dmail_nft bind "(\"9500\",principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft unbind "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft query_bind_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister call dmail_nft query_alias_by_pid "(principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
