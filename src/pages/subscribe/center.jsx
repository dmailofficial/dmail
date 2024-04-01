import React, { useEffect, useState, useCallback, useRef } from "react";
import { observer, inject } from "mobx-react";
import { withRouter, useParams, useHistory } from "react-router-dom";
import { fromJS } from "immutable";

import Modal from "@/components/Modal/index";
import PageEmpty from "@/components/empty";
import { Spin, ButtonLoading } from "@/components/Loading";
import { useGenerateRefAndState, useScrollEndLoadMore } from "@/utils/useHooks";
import SvgsSubscription from "@/components/svgs/subscription";
import SvgsCommon from "@/components/svgs/common";
import ImgUser from "@/static/images/user.png";
import { passedStatus } from '@/pages/external-subscription/utils'
import Search from './search'
import Detail from './detail'
import { unSubText, useSlidePrevAndNext } from './utils'
import { SubscribedRoot } from "./css";

const Chunk = ({ store }) => {
  const history = useHistory();
  const { key } = useParams();
  const { isMobile } = store.mobile;
  const { userInfo } = store.common;

  const [apps, setApps, appRef] = useGenerateRefAndState([])
  const totalRef = useRef(0)
  const pageRef = useRef(0)

  const onSelect = (nft) => () => {
    // ignore tabs switch bug
    setApps([])
    totalRef.current = 0
    pageRef.current = 0
    history.push(`/hubs/subcenter/${nft}`)
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
          const list = fromJS(appRef.current).toJS();
          for (var item of list) {
            if (item.id === id) {
              item.is_follow = false
              break
            }
          }
          setApps(list)
        }
        return true
      },
    });
  }

  const toSubscribe = ({ id }) => async (ev) => {
    ev.stopPropagation()

    if (true) {
      const list = fromJS(appRef.current).toJS();
      for (var item of list) {
        if (item.id === id) {
          item.is_follow = true
          break
        }
      }
      setApps(list)
    }
  }

  return (
    <>
      {key ? <Detail parentPath="subcenter" /> : (
        <SubscribedRoot className={`subcenter-root ${isMobile ? 'mobile' : ''}`}>
          {!isMobile ? null : <Search onSearch={onSearch} />}
          <div className="types">
            <p className="tprev" onClick={onTypesPrev}><SvgsCommon type="arrow" /></p>
            <div className="types-list">
              <div ref={typesDomRef} style={{ transform: `translateX(${typesLeft}px)` }}>
                {types.map(({ value, name }) => <span onClick={onType(value)} className={currentType === value ? 'on' : ''} key={value}>{ name }</span>)}
              </div>
            </div>
            <p className="tnext" onClick={onTypesNext}><SvgsCommon type="arrow" /></p>
            {isMobile ? null : <Search onSearch={onSearch} />}
          </div>
          <div className="app-list" ref={wrapperRef}>
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
                          {app.is_follow ? (
                            <a className="gray-btn" onClick={toUnSubscribe(app, true)}>
                              {loading == app.id ? <ButtonLoading size={14} color={'#666'} /> : null}
                              <strong>Subscribed</strong><strong style={{ display: 'none' }}>Unsub</strong>
                            </a>
                          ) : (
                            <a className="red-btn" onClick={toSubscribe(app, true)}>
                              {loading == app.id ? <ButtonLoading size={isMobile ? 14 : 16} color={'#fff'} /> : null}
                              Subscribe
                            </a>
                          )}
                        </div>
                      </div>
                    </li>
                  ))}
                </ul>
              )}
            </Spin>
          </div>
        </SubscribedRoot>
      )}
    </>
  );
};

export default withRouter(inject("store")(observer(Chunk)));
