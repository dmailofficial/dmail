import React, { useEffect, useState, useCallback, useRef } from "react";
import { observer, inject } from "mobx-react";
import { withRouter, useParams } from "react-router-dom";
import { fromJS } from "immutable";

import Tabs from "@/components/Tabs";
import Container from "@/components/layout/container";
import Subscribed from './subscribed'
import Center from './center'
import { Root, Title } from "./css";

const tabList = [
  {
    name: 'Subscription Center',
    value: 1,
    key: 'subcenter',
    onClick: (history) => {
      history.push('/hubs/subcenter')
    }
  },
  {
    name: 'Subscribed',
    value: 2,
    key: 'subscribed',
    onClick: (history) => {
      history.push('/hubs/subscribed')
    }
  },
]

const Index = ({ location: { pathname }, store }) => {
  const { isMobile } = store.mobile;
  return (
    <Container noSearch="false">
      <Root className={`flexColumn1`}>
        <Tabs tabList={tabList} matchKey="key">
          <Center />
          <Subscribed />
        </Tabs>
      </Root>
    </Container>
  );
};

export default withRouter(inject("store")(observer(Index)));
