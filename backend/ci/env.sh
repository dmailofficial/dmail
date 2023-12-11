# if [ $1 == "ic" ]; then
#   # choose question and make sure that $1
#   cp ./ci/ic.json ./ci/config.json
# elif [ $1 == "pre" ]; then
#   # choose question and make sure that $1
#   cp ./ci/pre.json ./ci/config.json
# else
#   cp ./ci/local.json ./ci/config.json
# fi

# principal\"2vxsx-fae\";

# MODE=$3
# if [[ "$MODE" == "reinstall" ]]; then
#   MODE="--mode reinstall"
# fi

export PATH="/Volumes/LaCie/code/com/dmail/dmail-icp-core:$PATH"
echo $(dfx identity get-principal)
# dfx deploy --network $1 $2 --argument "(
#      opt record {
#          name=opt \"$2\";
#          custodians=opt vec {
#             principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#             principal\"lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae\";
#           }
#       }
#  )" $MODE

# echo $(dfx canister --network $1 call $2 custodians "()")

dfx canister --network $1 call $2 queryEmailById "(1)"
# ./dfx canister --network $1 call $2 dip721_burn "(28866,\"dmyze22\")"
# ./dfx canister --network $1 call $2 dip721_token_metadata "(28866)"

# dfx canister --network $1 call $2 query_id_by_alias "(\"45mailbox\")"
# ./dfx canister --network $1 call $2 getAllTheIdMap "()"

# dfx canister --network $1 call $2 setCustodians "(vec {
#             principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#             principal\"lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae\";
#             principal\"doitg-kqaaa-aaaak-ae4qa-cai\";
# })"
# dfx canister --network $1 call $2 query_all_cid_by_split "(180000,180002)"
# dfx canister --network $1 call $2 install_bucket_code "(principal\"pgkm4-oiaaa-aaaak-ad2ua-cai\")"
# dfx canister --network $1 call $2 get_user_root_buckets "(record { user=principal\"pgkm4-oiaaa-aaaak-ad2ua-cai\"; witness= false })"

# dfx canister --network $1 call $2 update_create_jwt_by_pid "(principal \"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\")"
# dfx canister --network $1 call $2 query_some_jwt_by_pid "(principal \"lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae\")"

# dfx canister --network $1 call $2 update_take_all_pid_by_jwt "()"
# dfx canister --network $1 call $2 getPidByAddr "(\"0x27dB6Fc8739318f750712C306D07fA2247384aaD\")"
# dfx canister --network $1 call $2  linkAddressWithWalletName "(\"0x27dB6Fc8739318f750712C306D07fA2247384aaD\",\"EVM\")"
# dfx canister --network $1 call $2 getPidByAddr "(\"0x27dB6Fc8739318f750712C306D07fA2247384aaD\")"

# dfx canister --network $1 call $2 getAddrByPid "(principal \"ql7aw-nkat3-tiysq-bj2t3-kgfer-m6blc-5froq-6v7aj-4y7ei-qdfn6-gqe\")"
# dfx canister --network $1 call $2 dip721_mint "(
#     principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#     0:nat,
#     vec {}
#   )"

# dfx canister --network $1 call $2 git_commit_hash "(
#   principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#   0:nat,
#   vec {}
# )"

# dfx canister --network $1 call $2 mint "(
#     principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#     0:nat,
#     vec {
#       record {
#         \"location\";
#         variant {
#           TextContent = \"my  url\"
#         }
#       };
#       record {
#         \"alias\";
#         variant {
#           TextContent = \"bin6\"
#         }
#       };
#       record {
#         \"binding\";
#         variant {
#           BoolContent = false
#         }
#       };
#       record {
#         \"points\";
#         variant {
#           Nat64Content = 90000
#         }
#       };
#     }
#   )"

# dfx canister --network $1 call $2 mint "(
#     principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#     0:nat,
#     vec {
#       record {
#         \"location\";
#         variant {
#           TextContent = \"my_url\"
#         }
#       };
#       record {
#         \"alias\";
#         variant {
#           TextContent = \"bin2\"
#         }
#       };
#       record {
#         \"binding\";
#         variant {
#           BoolContent = false
#         }
#       };
#       record {
#         \"points\";
#         variant {
#           Nat64Content = 400
#         }
#       };
#     }
#   )"
# dfx canister --network $1 call $2  bind  "(principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\",opt 9501,9500)"
# dfx canister --network $1 call $2 dip721_owner_of "(47903)"
# dfx canister --network $1 call $2 getPidByAddr "(\"0x01bfd403a54abd4848cace111c2438f885a3edf8\")"
# dfx canister --network $1 call $2 queryCidByPid "(principal \"7i2bi-x3zbw-efhnd-nzpl2-w3t65-eqj5e-ixlce-sbodm-fevzm-wql4k-lae\")"
# dfx canister --network $1 call $2 queryPidByCid "(principal\"khues-fiaaa-aaaak-ado5a-cai\")"
# ./dfx canister --network $1 call $2 dip721_owner_token_metadata "(principal \"4blvb-bvli4-lgksb-o7som-kssil-j3ioz-gzl3l-cfkty-zzq5b-5ekod-zae\")"
# dfx canister --network $1 call $2 contract_id "()"
# dfx canister --network $1 call $2 get_token_contract_root_bucket "(record {
#   witness=false;
#   canister=principal\"qohn4-fiaaa-aaaak-ady6a-cai\";
# })"

# dfx canister --network $1 call $2 insert "(
#   record {
#     operation=\"sss\";
#     details=vec{ record { \"to\"; variant { True }; };};
#     caller=principal\"pgkm4-oiaaa-aaaak-ad2ua-cai\";
#   }
# )"
# dfx canister --network $1 call $2 get_transactions "(record{ id=0; witness= false })"

# dfx canister --network $1 call $2 bucket_status "(principal\"pgkm4-oiaaa-aaaak-ad2ua-cai\")"
# dfx canister --network $1 call $2 insert_new_users "(principal\"pgkm4-oiaaa-aaaak-ad2ua-cai\")"
# dfx canister --network $1 call $2 get_index_canisters "(record { witness=true })"
# dfx canister --network $1 call $2 trigger_upgrade "(\"we know what we are doing\")"
# dfx canister --network $1 call $2 get_token_contract_root_bucket "(record { canister=principal\"qohn4-fiaaa-aaaak-ady6a-cai\"; witness=false })"
# dfx canister --candid    --network $1 call $2 get_user_transactions "( record {
#   page=0;
#   user=principal\"azatl-afb3n-celcz-e5jkt-hclfn-otkr5-6uo3t-2wfkr-e53u7-wc3np-jae\";
#   witness =false;
# })"

# dfx canister --network $1 call $2 dip721_owner_token_metadata "(principal\"elygp-kboaw-fgmm2-zjj6y-lykdd-rfiqt-ecwjm-5545i-755ha-45b6i-xqe\")"
# dfx canister --network $1 call $2 query_pid_by_alias "(\"tino\")"
# dfx canister --network $1 call $2 transfer "(principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",100001)"
# dfx canister --network $1 call $2 query_pid_by_alias "(\"lijiaxu100001\")"
# dfx canister --network $1 call $2 transferFrom "(principal\"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",principal\"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\",348919)"
# dfx canister --network $1 call $2 query_pid_by_alias "(\"lijiaxu9501\")"

# dfx canister --network $1 call $2 dip721_transfer_from "(
#   principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\",
#   principal\"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#   9504
# )"
# dfx canister --network $1 call $2 dip721_transfer_from "(
#   principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\",
#   principal\"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\",
#   9505
# )"

# common
#  dfx canister --network $1 call $2 setCustodians "(
#    vec {
#             principal\"lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae\";
#             principal\"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#             principal\"6cjya-hnrph-ohcs7-pt7m4-dxzwj-zax3s-b5f2w-n472d-mulap-7jaeg-2ae\";
#             principal\"doitg-kqaaa-aaaak-ae4qa-cai\";
#           }
#  )"

# call
# pid: Principal, old_index: u64, new_index: u64
# dfx canister --network $1 call $2 custodians "()"
# dfx canister --network $1 call $2 queryBindIndexAndAliasByPid "(principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\")"
# dfx canister --network $1 call $2 query_bind_alias_by_pid "(principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\")"
# dfx canister --network $1 call $2 query "(principal \"d4jfd-luys3-hmaz6-2ux6f-wizkv-cpaxs-5s33e-6wd5i-ai4kk-ok4qq-hqe\")"

# dfx canister --network $1 call $2 query_len "()"
# dfx canister --network $1 call $2 queryBindAliasByPid  "( principal \"567to-2ufhs-rzv5c-2wnbb-6y34z-kzi7q-nvwcv-ulekn-2esk4-kyggc-iae\")"
# dfx canister --network $1 call $2 queryBindAliasByPid  "( principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\")"
# dfx canister --network $1 call $2 register "( \"amino_nft\", principal \"4phuj-zqaaa-aaaak-acp7a-cai\" )"
# dfx canister --network $1 call $2 register "( \"Dmail\", principal \"rno2w-sqaaa-aaaaa-aaacq-cai\" )"
# dfx canister --network $1 call $2 queryDmailAll "()"
#  vcofx-kaaaa-aaaak-acooa-cai
# dfx canister --network $1 call $2 getPidByAddr "(\"0xD55696dC476410F12E71F7D3a944B5384a4A2033\")"

# dfx canister --network $1 call $2 query_pid_by_alias "(\"dmyxx009\")"
# dfx canister --network $1 call $2 changeBindCid "()"

# dfx canister --network $1 call $2 query "(principal \"q4gry-n3xtc-p4di4-tyjnt-vsyj7-qbjo4-wyxka-qsgkp-ese22-vcd6u-xqe\")"
# dfx canister --network $1 call $2 bind "(principal \"s2mm4-gcijs-h3hmj-tk4le-brlzr-avnsb-weayc-ffjog-vjpj6-6ww3z-aae\",null,1924018:nat64)"
# dfx canister --network $1 call $2 query "(principal \"s2mm4-gcijs-h3hmj-tk4le-brlzr-avnsb-weayc-ffjog-vjpj6-6ww3z-aae\")"
# dfx canister --network $1 call $2 query "(principal \"g7ygv-uwk4k-6q3ng-4d5pt-cgv53-26fpn-lnktx-mhrxw-53ynk-gp435-xqe\" )"
# dfx canister --network $1 call $2 queryWeb2MailCache "(7)"

# dfx canister --network $1 call $2 bindCid "(principal \"a3z3d-piolh-fwnzd-cicjy-5t7zd-kjqdr-btniu-aql23-totrn-4lzia-2ae\",principal \"5ekwd-fyaaa-aaaan-qaxlq-cai\")"
# dfx canister --network $1 call $2 info
# dfx canister --network $1 call $2 query_all_pid_len "()"
# dfx canister --network $1 call $2 singleUpdateSettings "(principal \"4vsif-3aaaa-aaaak-abldq-cai\" )"
# dfx canister --network $1 call $2 singleCanisterStatus "(principal \"4vsif-3aaaa-aaaak-abldq-cai\" )"
# dfx canister --network $1 call $2 updateCount "()"
# dfx canister --network $1 call $2 queryCount "()"

# dfx canister --network $1 call $2 upgradeCode "()"
# dfx canister --network $1 call $2 queryWeb2MailCache  "(1)"
# dfx canister --network $1 call $2 queryCidByPid  "(principal \"7oe7r-oy5an-2qht5-5rfat-i6mgw-g6hlm-csoe7-epxsn-qjfcd-bhqi6-6ae\" )"
# dfx canister --network $1 call $2 ownerTokenMetadata  "(principal \"xsb7i-osvzh-finyu-rlai3-tmb5h-n3eel-cmvnj-mweyn-rsinx-mhbyv-tae\" )"
# dfx canister --network $1 call $2 dip721_token_metadata "(37790:nat)" # nft序号大于9500

# dfx canister --network $1 call $2 isa1dkiip9 "(\"0x5badc91651ebe3ae189cb09f773c828f62f0581386bdaae1fcb4bd40392ce7\",\"0x0fF213846D42528A4CA24f8079f85b8044AcfA7B\",\"0xbaef51e96cc70131a51cb2303a0e85e50d3d6dd93d7ba9a600c6548f09f684406eb4293161e6ec750403dc41992ff6ccb0e52ff0788d69725bf65c12d3d1d21e1c\",\"OTJjZWZkZDZlMjAyNmE5ZDE5NzZiOTU5NzE3YWNiMTE=\")" # nft序号大于9500
# dfx canister --network $1 call $2 isa1dkiip9 "(\"lhlw2-d53lt-rsq5n-5iyhb-nthfo-kjvvk-lkgqx-ibgou-am7sx-g4wlt-7ae\",\"0x0fF213846D42528A4CA24f8079f85b8044AcfA7B\",\"0xebeda77060240ad18ab19f810d9624090a67e7e3b0e2da46220aacbbaa63e7ad58d41604de5499be9f68265625004981bb7d20abdac6994f135b4904c6bb9d5a1c\",\"ZGI4ZGIwZGQ0NmM4NDIyOTI4YjU4YTI5YWZiMzVlMzY=\")"  # nft序号大于9500
# dfx canister --network $1 call $2 qsa1oldazxq1 "(\"22sssaqqq\")" # nft序号大于9500
# dfx canister --network $1 call $2 qsa1oldazxql "(\"22sssaqqq\")" # nft序号
# dfx canister --network $1 call $2 usa1olpifo2 "(\"22sssaqqq\")" # nft序号大于9500
# dfx canister --network $1 call $2 qsa1oldazxq1 "(\"22sssaqqq\")" # nft序号大于9500
# dfx canister --network $1 call $2 qsa1oldazxqi "(2)" # nft序号大于9500

# did_cache
# dfx canister --network $1 call $2 createAnDidMail "(
#   record {
#     cid=principal \"ppo3r-raaaa-aaaak-aav2q-cai\";
#     email_body=record {
#       content=\"neiong\";
#        assets=\"\";
#         attachment_list=vec {
#           record {
#             name=\"166.png\";
#              size=1582;
#               mime_type=\"{type:'image/png',mixedId:'K2dggg8gMpsadL-VytRXf'}\";
#              id=0;
#           }
#         }};
#          email_header=record {
#           id=0;
#           subject=\"dier22222ci\";
#           created_at=0;
#            s_pid=opt principal \"6hjv3-5yksc-c7gxr-tsh23-ywjic-qzld6-tivgu-r234t-rx7ax-4b2e2-vae\";
#            parent_id=null;
#            r_alias=\"lijiaxu.did\";
#            s_alias=\"dongge\";
#            r_pid=null
#            }
#         }
# )"
# dfx canister --network $1 call $2 createAnDidMail "(
#   record {
#     cid=principal \"ppo3r-raaaa-aaaak-aav2q-cai\";
#     email_body=record {
#       content=\"neiong\";
#        assets=\"\";
#         attachment_list=vec {
#           record {
#             name=\"166.png\";
#              size=1582;
#                 mime_type=\"{type:'image/png',mixedId:'K2dggg8gMpsadL-VytRXf'}\";
#              id=0;
#           }
#         }};
#          email_header=record {
#           id=0;
#           subject=\"dier333333i\";
#           created_at=0;
#            s_pid=opt principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#            parent_id=null;
#            r_alias=\"lijiaxu1.did\";
#            s_alias=\"dongge\";
#            r_pid=null
#            }
#         }
# )"
# dfx canister --network $1 call $2 createAnDidMail "(
#   record {
#     cid=principal \"ppo3r-raaaa-aaaak-aav2q-cai\";
#     email_body=record {
#       content=\"neiong\";
#        assets=\"\";
#         attachment_list=vec {
#           record {
#             name=\"166.png\";
#              size=1582;
#                 mime_type=\"{type:'image/png',mixedId:'K2dggg8gMpsadL-VytRXf'}\";
#              id=0;
#           }
#         }};
#          email_header=record {
#           id=0;
#           subject=\"dier333333i\";
#           created_at=0;
#            s_pid=opt principal \"3g3l5-7forh-pp6xk-fd4us-bowx7-tz3sn-scucg-2nhdn-dc6iy-ga6sp-2qe\";
#            parent_id=null;
#            r_alias=\"lijiaxu3.did\";
#            s_alias=\"dongge\";
#            r_pid=null
#            }
#         }
# )"
# dfx canister --network $1 call $2 createAnDidMail "(
#   record {
#     cid=principal \"ppo3r-raaaa-aaaak-aav2q-cai\";
#     email_body=record {
#       content=\"neiong\";
#        assets=\"\";
#         attachment_list=vec {
#           record {
#             name=\"166.png\";
#              size=1582;
#                 mime_type=\"{type:'image/png',mixedId:'K2dggg8gMpsadL-VytRXf'}\";
#              id=0;
#           }
#         }};
#          email_header=record {
#           id=0;
#           subject=\"dier333333i\";
#           created_at=0;
#            s_pid=opt principal \"6hjv3-5yksc-c7gxr-tsh23-ywjic-qzld6-tivgu-r234t-rx7ax-4b2e2-vae\";
#            parent_id=null;
#            r_alias=\"lijiaxu2.did\";
#            s_alias=\"dongge\";
#            r_pid=null
#            }
#         }
# )"
# dfx canister --network $1 call $2 queryMailByDid  "(\"69001.bab\")"
