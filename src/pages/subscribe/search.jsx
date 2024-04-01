import React, { useEffect, useState, useCallback, useRef } from "react";
import { observer, inject } from "mobx-react";

import { Search } from "@/components/layout/css.js";
import SvgsCommon from "@/components/svgs/common";

const Chunk = ({ store, onSearch }) => {
  const { isMobile } = store.mobile;

  return (
    <Search className={`search ${isMobile ? 'mobile' : ''}`}>
      <SvgsCommon type="search" />
      <input type="text" placeholder='Search DApp' onInput={onSearch} />
    </Search>
  )
}

export default inject("store")(observer(Chunk));