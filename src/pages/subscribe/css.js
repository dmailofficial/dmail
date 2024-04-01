import styled, { keyframes } from "styled-components";
import {
  flexAlign,
  Title,
  FlexBetweenWrapper,
} from "../../components/css.common";
import { RootStyle } from '@/pages/external-subscription/css'

export {
  Title,
  FlexBetweenWrapper,
}

export const Root = styled.div`
  margin-top: 20px;

  &.no-top {
    margin-top: 0;
  }
`;

export const SubscribedRoot = styled.div`
  flex: 1;
  box-sizing: border-box;

  &.subscribed-root {
    padding: 0 15px;
    overflow-y: auto;
  }

  &.subcenter-root {
    display: flex;
    flex-direction: column;
    width: 100%;
    overflow: hidden;
  }

  &.mobile {
    padding: 20px 0;

    &.subscribed-root {
      padding: 20px 15px;
    }

    .search {
      width: auto;
      height: 35px;
      margin: 0 15px 20px;
    }

    .types {
      padding-bottom: 15px;
      margin-bottom: 10px;
    }

    li {
      width: 100%;
      padding: 0;
      margin-bottom: 15px;
      margin-right: 0;

      &:hover {
        transform: none;
      }

      &:hover, &.on {
        background-color: #fff;
      }
    }

    .desc {
      margin-top: 15px;
      padding-top: 12px;
      border-top: 1px solid #eee;
    }

    .project {
      .ava {
        width: 50px;
        height: 50px;
        margin-right: 10px;
      }

      .name {
        h1 {
          margin-bottom: 5px;
        }

        p {
          font-size: 12px;
        }
      }

      .actions {
        a {
          height: 28px;
          padding: 0 12px;
          font-size: 13px;

          strong, &:hover strong {
            &:first-of-type {
              display: none!important;;
            }

            &:last-of-type {
              display: inline!important;
            }
          }

          /* & > svg {
            display: none;
          } */
        }
        
        .red-btn {
          & > span {
            margin-right: 5px;
          }
        }
      }
    }
  }

  .search {
    width: 220px;
    height: 40px;
    border-radius: 10px;
    background-color: #F5F5F5;
    border: 1px solid #eee;
    margin-left: 25px;
  }

  .types {
    display: flex;
    align-items: center;
    overflow: hidden;
    padding: 0 15px 10px;

    p {
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      width: 24px;
      height: 24px;
      background: #F6F6F6;
      border-radius: 5px;
      color: #999;
      cursor: pointer;

      &:hover {
        background: #EDEDED;
      }

      &.tprev {
        margin-right: 20px;

        svg {
          transform: rotate(180deg);
        }
      }

      &.tnext {
        margin-left: 20px;
      }
    }

    .types-list {
      flex: 1;
      overflow: hidden;

      div {
        display: flex;
        flex-wrap: nowrap;
        transition: transform .3s ease-out 0s;
      }
    }

    span {
      margin-right: 10px;
      padding: 0 20px;
      line-height: 30px;
      border-radius: 14px;
      color: #999;
      font-size: 14px;
      white-space: nowrap;
      cursor: pointer;

      &:last-child {
        margin-right: 0;
      }

      &.on, &:hover {
        color: #333;
        background-color: #EDEDED;
      }
    }
  }

  .app-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 15px;
  }

  ul {
    display: flex;
    flex-wrap: wrap;
    padding-bottom: 10px;
  }

  li {
    display: flex;
    width: calc((100% - 10px) / 2);
    margin-bottom: 5px;
    margin-right: 10px;
    padding: 15px;
    box-sizing: border-box;
    border-radius: 10px;
    cursor: pointer;
    transition: transform 0.4s ease 0s;

    &:nth-child(2n+2) {
      margin-right: 0;
    }

    @media (min-width: 1780px) {
      width: calc((100% - 20px) / 3);

      &:nth-child(2n+2) {
        margin-right: 10px;
      }

      &:nth-child(3n+3) {
        margin-right: 0;
      }
    }

    &:hover {
      transform: scale(1.02);
    }

    &:hover, &.on {
      background-color: #EDEDED;

      .project {
        border-bottom-color: #F8F9FB;

        .actions {
          .gray-btn {
            background-color: #fff;
          }
        }
      }
    }
  }

  .ava {
    width: 50px;
    height: 50px;
    padding: 0;
    border-radius: 10px;
    border: 1px solid #EEEEEE;
    margin-right: 15px;
    position: relative;

    img {
      width: 100%;
      height: 100%;
      border-radius: 10px;
    }

    &.default-ava {
      img {
        border-radius: 0;
      }
    }
  }

  .project {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex: 1;
    height: 52px;

    .flex {
      display: flex;
      align-items: center;
    }

    .actions {
      a {
        ${flexAlign};
        justify-content: center;
        width: 98px;
        height: 32px;
        font-size: 14px;
        border-radius: 5px;

        strong {
          font-weight: normal;
        }

        &:hover {
          strong {
            &:first-of-type {
              display: none;
            }

            &:last-of-type {
              display: inline!important;
            }
          }
        }

        & > span {
          margin-right: 6px;
        }

        & > svg {
          width: 14px;
          margin-right: 5px;
        }
      }

      .gray-btn {
        background-color: #fff;
        border: 1px solid #999;
        color: #999;
        box-shadow: none;

        &:hover {
          box-shadow: none;
        }
      }

      .num {
        margin-top: 5px;
        font-size: 12px;
        color: #999;
        text-align: center;

        span {
          color: #FF563F;
          margin-right: 6px;
        }
      }
    }

    .name {
      h1 {
        line-height: 1;
        font-size: 16px;
        margin-bottom: 10px;

        svg {
          vertical-align: middle;
          margin-left: 6px;
        }
      }

      p {
        display: inline-block;
        /* line-height: 20px; */
        /* padding: 0 10px; */
        /* background-color: #F4F4F4; */
        /* border-radius: 10px; */
        font-size: 14px;
        color: #999;
      }
    }
  }

  .desc {
    margin-top: 20px;

    h2 {
      font-size: 14px;
    }

    div {
      color: #999;
      margin-top: 5px;
      min-height: 28px;
      line-height: 16px;
      font-size: 12px;
    }
  }
`

export const DetailRoot = styled.div`
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  .back {
    ${flexAlign};
    margin: 5px 20px 0;
    font-size: 16px;
    font-weight: bold;
    color: #666;
    cursor: pointer;

    p {
      ${flexAlign};
      justify-content: center;
      width: 45px;
      height: 45px;
      margin-right: 10px;
      background: #F6F6F6;
      border-radius: 10px;

      svg {
        transform: rotate(180deg);
      }
    }
  }

  ${RootStyle};

  &.mobile {
    .top-back {
      padding: 20px 15px 15px!important;
      position: -webkit-sticky;
      position: sticky;
      background-color: #fff;
      top: 0;
      left: 0;
      right: 0;
      z-index: 10;

      .back {
        margin: 0;
      }
    }

    .detail-wrapper {
      margin: 5px 0 0;
      padding: 15px;
    }

    .detail {
      width: auto;
      margin: 0;
    }

    .project {
      .actions {
        a {
          border-radius: 5px;

          strong, &:hover strong {
            &:first-of-type {
              display: none!important;;
            }

            &:last-of-type {
              display: inline!important;
            }
          }
        }

        .red-btn  {
          border: none;
        }
      }
    }

    .desc {
      margin-top: 15px;
    }
  }

  .detail-wrapper {
    flex: 1;
    overflow-y: auto;
    margin-top: 30px;
  }

  .detail {
    width: 78%;
    margin: 0 auto;
  }

  .project {
    padding: 0;
    position: static;

    .name {
      h1 {
        font-size: 20px;
        margin-bottom: 6px;

        svg {
          margin-left: 6px;
        }
      }
    }

    .actions {
      a {
        width: 110px;
        height: 32px;
        font-size: 14px;
        border-radius: 5px;
        
        & > svg {
          flex-shrink: 0;
          width: auto;
          margin-right: 6px;
        }

        strong {
          font-weight: normal;
        }

        &:hover {
          strong {
            &:first-of-type {
              display: none;
            }

            &:last-of-type {
              display: inline!important;
            }
          }
        }
      }

      .red-btn  {
        border: none;
      }
    }

    .ava {
      width: 60px;
      height: 60px;
      margin-right: 15px;
    }
  }

  .desc {
    margin-top: 15px;
    padding-bottom: 24px;
    line-height: 18px;
    font-size: 14px;
  }

  .infos-title {
    margin: 15px 0;
  }

  .links {
    .link {
      height: 45px;
      border: none;
      margin-bottom: 10px;
    }
  }
`