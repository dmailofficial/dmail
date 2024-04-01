import React, { Suspense, lazy } from "react";
import { Switch, Router, Route, Redirect, BrowserRouter, useLocation } from "react-router-dom";

import { Spin } from "@/components/Loading";

// can not use 'React.lazy' because of guide
import Compose from "@/pages/compose/index";
import Inbox from "@/pages/inbox/index";
import Setting from "@/pages/setting/index";
import Events from "@/pages/events/index";
import Starred from "@/pages/starred/index";
import Sent from "@/pages/sent/index";
import Drafts from "@/pages/drafts/index";
import Spam from "@/pages/spam/index";
import Trash from "@/pages/trash/index";
import Assets from "@/pages/assets/index";
import Market from "@/pages/market/index";
import Orders from "@/pages/presale/orders/index";
import Contact from "@/pages/contact/index";
import Subscribe from "@/pages/subscribe/index";

// import Test from "@/pages/test/index";
// import Upgrades from "@/pages/upgrades/index";

export const NoNavAndTopPaths = ['/login', '/dlogin', '/alogin', '/upgrades']
export const NoTopPaths = ['/orders', '/referrals', '/compose']

// const Compose = React.lazy(() => import("@/pages/compose/index"));
const Presale = React.lazy(() => import("@/pages/presale/index"));
const Referrals = React.lazy(() => import("@/pages/presale/referrals/index"));
const Login = React.lazy(() => import("@/pages/login/index"));
const DLogin = React.lazy(() => import("@/pages/dlogin/index"));
const AminoLogin = React.lazy(() => import("@/pages/amino/login"));

// same with container.jsx
const Loading = () => {
  return <div className="main-chunk">
    <Spin loading={ true } text="Loading..." className="main-loading" maskStyle={ { background: 'rgba(255, 255, 255, 0.85)' } } />
  </div>
}

const Routes = () => (
  <Suspense fallback={<Loading />}>
    <Switch>
      <Route exact path="/login" component={ Login } />
      <Route exact path="/dlogin" component={ DLogin } />
      <Route exact path="/alogin/:key" component={AminoLogin} />
      
      <Route exact path="/inbox" component={ Inbox } />
      <Route exact path="/starred" component={ Starred } />
      <Route exact path="/sent" component={ Sent } />
      <Route exact path="/drafts" component={ Drafts } />
      <Route exact path="/spam" component={ Spam } />
      <Route exact path="/trash" component={ Trash } />
      <Route exact path="/setting/:tabname?/:cid?" component={ Setting } />
      <Route exact path="/events/:tabname?" component={ Events } />
      <Route exact path="/market/:tabname?" component={ Market } />
      <Route exact path="/compose/:emailId?" component={ Compose } />
      <Route exact path="/presale/:channelId?" component={ Presale } />
      <Route exact path="/referrals" component={ Referrals } />
      <Route exact path="/orders" component={ Orders } />
      <Route exact path="/assets" component={ Assets } />
      <Route exact path="/contacts/:tabname?" component={ Contact } />
      <Route exact path="/hubs/:tabname/:key?" component={Subscribe} />
      
      {/* <Route exact path="/test" component={Test} /> */}
      {/* <Route exact path="/upgrades" component={Upgrades} /> */}
      <Redirect from="*" to="/inbox" /> 
    </Switch>
  </Suspense>
);

export default Routes;
