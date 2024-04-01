import { useState, useEffect } from "react"

export const unSubText = 'After unsubscribing, you will not be able to receive project information, are you sure to unsubscribe?'

export const useSlidePrevAndNext = (domRef) => {
  const [left, setLeft] = useState(0)
  const onSlide = (_index) => {
    if (!domRef.current) {
      return
    }

    const dom = domRef.current
    const parentWidth = dom.parentElement.clientWidth
    const domScrollWidth = dom.scrollWidth
    // console.log('parentWidth', parentWidth, domScrollWidth)
    if (_index > 0) {
      if (domScrollWidth + left > parentWidth) {
        setLeft(left - Math.min(domScrollWidth + left - parentWidth, parentWidth))
      }
    } else if (left < 0) {
      setLeft(left + Math.min(Math.abs(left), parentWidth))
    }
  }

  const currentScrollToView = () => {
    if (!domRef.current || !domRef.current.querySelector('.on')) {
      return
    }

    const dom = domRef.current
    const parentWidth = dom.parentElement.clientWidth
    const current = dom.querySelector('.on')
    const currentOffsetLeft = current.offsetLeft
    const currentWidth = current.clientWidth
    // console.log('left', left, parentWidth, currentOffsetLeft, currentWidth)
    if (Math.abs(left) + parentWidth < currentOffsetLeft + currentWidth) {
      setLeft(-(currentOffsetLeft + currentWidth - parentWidth))
    } else if (Math.abs(left) > currentOffsetLeft) {
      setLeft(-currentOffsetLeft)
    }
  }

  const onPrev = () => onSlide(-1)
  const onNext = () => onSlide(1)

  useEffect(() => {

  }, [])

  return [left, onPrev, onNext, currentScrollToView]
}