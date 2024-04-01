import React, { useEffect, useState, useCallback, useRef } from "react";
import { observer, inject } from "mobx-react";
import { withRouter, useParams, useHistory } from "react-router-dom";
import { fromJS } from "immutable";

import Modal from "@/components/Modal/index";
import Message from "@/components/Message/index";
import PageEmpty from "@/components/empty";
import { Spin, ButtonLoading } from "@/components/Loading";
import { useGenerateRefAndState, useScrollEndLoadMore } from "@/utils/useHooks";
import SvgsSubscription from "@/components/svgs/subscription";
import ImgUser from "@/static/images/user.png";
import { passedStatus } from '@/pages/external-subscription/utils'
import Detail from './detail'
import { unSubText } from './utils'
import { SubscribedRoot } from "./css";

const Chunk = ({ store }) => {
  const history = useHistory();
  const { key } = useParams();
  const { isMobile } = store.mobile;

  const [apps, setApps, appRef] = useGenerateRefAndState([])
  const totalRef = useRef(0)
  const pageRef = useRef(0)
  const onSelect = (nft) => () => {
    // avoid tabs switch bug
    setApps([])
    setReQuery(0)
    totalRef.current = 0
    pageRef.current = 0
    setFirstLoading(true)
    history.push(`/hubs/subscribed/${nft}`)
  }

  const toUnSubscribe = ({ id }) => async (ev) => {
    ev.stopPropagation()

    Modal({
      isMobile,
      width: '405px',
      type: "warn",
      title: "Unsubscribe",
      content: unSubText,
      okText: "Continue",
      async onOk() {
        if (true) {
          pageRef.current = 0
          setReQuery(reQuery+1) 
        }
        return true
      },
    });
  }

  return (
    <>
      {key ? <Detail parentPath="subscribed" /> : (
        <SubscribedRoot ref={wrapperRef} className={`subscribed-root ${isMobile ? 'mobile' : ''}`}>
          <Spin loading={firstLoading} style={ { height: '100%' } }>
            {!apps.length ? <PageEmpty /> : (
              <ul>
                {apps.map((app, index) => (
                  <li key={index} onClick={onSelect(app.nft)}>
                    <div className={`ava ${app.logo ? '' : 'default-ava'}`}>
                      <img src={app.logo || ImgUser} alt="" />
                    </div>
                    <div className="project">
                      <div className="flex">
                        <div className="name">
                          <h1>{app.name || '--'}{app.name && app.certifications == passedStatus ? <SvgsSubscription type="verified" width="16" /> : null}</h1>
                          <p>
                            {app.typetext || '--'}
                          </p>
                        </div>
                      </div>
                      <div className="actions">
                        <a className="gray-btn" onClick={toUnSubscribe(app, true)}>
                          {loading == app.id ? <ButtonLoading size={14} color={'#666'} /> : null}
                          <strong>Subscribed</strong><strong style={{ display: 'none' }}>Unsub</strong>
                        </a>
                      </div>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </Spin>
          
        </SubscribedRoot>
      )}
    </>
  );
};

export default withRouter(inject("store")(observer(Chunk)));
