import React, { useEffect, useState, useCallback, useRef } from "react";
import { observer, inject } from "mobx-react";
import { withRouter, useHistory, useParams } from "react-router-dom";
import { fromJS } from "immutable";

import Modal from "@/components/Modal/index";
import { getChainName } from '@/pages/login/metamask/bsc'
import { useGenerateRefAndState } from "@/utils/useHooks";
import { userInfoKeys } from "@/utils/storage";
import { unsubscribe } from "@/api/web2/subscription";
import { ButtonLoading } from "@/components/Loading";
import SvgsSubscription from "@/components/svgs/subscription";
import SvgsCommon from "@/components/svgs/common";
import { passedStatus } from '@/pages/external-subscription/utils'
import ImgUser from "@/static/images/user.png";
import { unSubText } from './utils'
import { DetailRoot, FlexBetweenWrapper } from "./css";

const Detail = ({ store, parentPath }) => {
  const history = useHistory();
  const { isMobile } = store.mobile;

  const [subed, setSubed] = useState(0)
  const [loading, setLoading] = useState(false)

  const goBack = () => {
    history.push(`/hubs/${parentPath}`)
  }

  const toUnSubscribe = ({ id }) => async (ev) => {
    Modal({
      isMobile,
      width: '405px',
      type: "warn",
      title: "Unsubscribe",
      content: unSubText,
      okText: "Continue",
      async onOk() {
        setLoading(true)
        const success = await unsubscribe(id, loginAddress)
        setLoading(false)
        if (success) {
          detectIsSubscription()
        }
        return true
      },
    });
  }

  const toSubscribe = async () => {
    if (true) {
      detectIsSubscription()
    }
  }

  const detectIsSubscription = async () => {
    const appid = appRef.current.id
    if (!loginAddress || !appid) {
      return
    }

    setSubed(1)
  }

  const btnRight = <SvgsCommon type="down" width={14} height={7} color={isMobile ? '#8F8D9B' : '#999'} />

  return (
    <DetailRoot className={`${isMobile ? 'mobile' : ''}`}>
      <FlexBetweenWrapper className="top-back bottom-shadow">
        <div className="back" onClick={goBack}>
          {isMobile ? (
            <>
              <SvgsCommon type="back" />
              <span></span>
            </>
          ) : (
            <>
              <p><SvgsCommon type="arrow" /></p>
              <span>Back</span>
            </>
          )}
        </div>
      </FlexBetweenWrapper>
      <div className="detail-wrapper">
        <div className="detail">
          <div className="project">
            <div className="flex">
              <div className={`ava ${app.logo ? '' : 'default-ava'}`}>
                <img src={app.logo || ImgUser} alt="" />
              </div>
              <div className="name">
                <h1>{app.name || '--'}{app.name && app.certifications == passedStatus ? <SvgsSubscription type="verified" width={isMobile ? "16" : "18"} /> : null}</h1>
                <p>
                  {app.typetext || '--'}
                </p>
              </div>
            </div>
            <div className="flex actions">
              {app.name && subed === 1 ? (
                <a className="gray-btn" onClick={toUnSubscribe(app)}>
                  {loading ? <ButtonLoading size={14} color={'#666'} /> : null}
                  <strong>Subscribed</strong><strong style={{ display: 'none' }}>Unsub</strong>
                </a>
              ) : null}
              {app.name && subed === -1 ? (
                <a className="red-btn" onClick={toSubscribe}>{loading ? <ButtonLoading size={isMobile ? 14: 16} color={'#fff'} /> : null}{isMobile ? null : <SvgsSubscription type="sub" width="14" />}Subscribe</a>
              ) : null}
            </div>
          </div>
          <div className="desc">{app.description || '--'}</div>
          <div className="infos-title">Official Links</div>
          <div className="links">
            {app.website ? (
              <a className="link" href={app.website} target="_blank">
                {/* <span><SvgsIcons type="website" /></span> */}
                <strong>Website</strong>
                <span>{isMobile ? null : <span>{app.website}</span>} {btnRight}</span>
              </a>
            ) : null}
            {app.twitter ? (
              <a className="link"  href={app.twitter} target="_blank">
                {/* <span><SvgsIcons type="twitter" /></span> */}
                <strong>Twitter</strong>
                <span>{isMobile ? null : <span>{app.twitter}</span>} {btnRight}</span>
              </a>
            ) : null}
            {app.discord ? (
              <a className="link"  href={app.discord} target="_blank">
                {/* <span><SvgsIcons type="discord" /></span> */}
                <strong>Discord</strong>
                <span>{isMobile ? null : <span>{app.discord}</span>} {btnRight}</span>
              </a>
            ) : null}
            {app.telegram ? (
              <a className="link"  href={app.telegram} target="_blank">
                {/* <span><SvgsIcons type="telegram" /></span> */}
                <strong>Telegram</strong>
                <span>{isMobile ? null : <span>{app.telegram}</span>} {btnRight}</span>
              </a>
            ) : null}
            {Array.isArray(app.custom_properties) ? (
              app.custom_properties.map(({ name, link }, key) => (
                <a className="link" href={link} target="_blank" key={key}>
                  {/* <span><SvgsIcons type="common" /></span> */}
                  <strong>{name}</strong>
                  <span>{isMobile ? null : <span>{link}</span>} {btnRight}</span>
                </a>
              ))
            ) : null}
          </div>
        </div>
      </div>
    </DetailRoot>
  );
};

export default withRouter(inject("store")(observer(Detail)));
