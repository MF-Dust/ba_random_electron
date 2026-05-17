import { ref } from 'vue'
import { audioApi } from '../api/audioApi'
import { floatingButtonApi } from '../api/floatingButtonApi'

const DRAG_THRESHOLD_PX = 3

export function useFloatingDrag(emit) {
  const pointerDown = ref(false)
  const activePointerId = ref(null)
  const isDragging = ref(false)
  const startGlobalX = ref(0)
  const startGlobalY = ref(0)
  const pendingDx = ref(0)
  const pendingDy = ref(0)
  const rafId = ref(0)
  let dragCommandQueue = Promise.resolve()

  const enqueueDragCommand = (task) => {
    dragCommandQueue = dragCommandQueue
      .catch(() => {})
      .then(task)
      .catch((error) => {
        console.warn('Floating drag command failed:', error)
      })
    return dragCommandQueue
  }

  const playClickSound = () => {
    audioApi.playClickSound().catch(() => {})
  }

  const getGlobalPoint = (event) => {
    const fallbackX = window.screenX + event.clientX
    const fallbackY = window.screenY + event.clientY

    if (event.pointerType === 'touch') {
      return { x: fallbackX, y: fallbackY }
    }

    const screenX = Number(event.screenX)
    const screenY = Number(event.screenY)
    return {
      x: Number.isFinite(screenX) ? screenX : fallbackX,
      y: Number.isFinite(screenY) ? screenY : fallbackY
    }
  }

  const flushMove = () => {
    if (!isDragging.value) {
      rafId.value = 0
      return
    }
    const dx = pendingDx.value
    const dy = pendingDy.value
    enqueueDragCommand(() => floatingButtonApi.moveDrag(dx, dy))
    rafId.value = 0
  }

  const scheduleMove = () => {
    if (rafId.value !== 0) return
    rafId.value = window.requestAnimationFrame(flushMove)
  }

  const cancelScheduledMove = () => {
    if (rafId.value !== 0) {
      window.cancelAnimationFrame(rafId.value)
      rafId.value = 0
    }
  }

  const handlePointerDown = (event) => {
    if (event.pointerType === 'mouse' && event.button !== 0) return
    pointerDown.value = true
    activePointerId.value = event.pointerId
    isDragging.value = false
    const point = getGlobalPoint(event)
    startGlobalX.value = point.x
    startGlobalY.value = point.y
    pendingDx.value = 0
    pendingDy.value = 0
    dragCommandQueue = Promise.resolve()
    cancelScheduledMove()
    if (event.currentTarget && event.currentTarget.setPointerCapture) {
      event.currentTarget.setPointerCapture(event.pointerId)
    }
  }

  const handlePointerMove = (event) => {
    if (activePointerId.value !== event.pointerId) return
    if (!pointerDown.value) return

    const point = getGlobalPoint(event)
    const dx = point.x - startGlobalX.value
    const dy = point.y - startGlobalY.value
    const movedEnough = Math.abs(dx) >= DRAG_THRESHOLD_PX || Math.abs(dy) >= DRAG_THRESHOLD_PX

    if (!isDragging.value && movedEnough) {
      isDragging.value = true
      enqueueDragCommand(() => floatingButtonApi.startDrag())
    }

    if (isDragging.value) {
      pendingDx.value = dx
      pendingDy.value = dy
      scheduleMove()
    }
  }

  const handlePointerUp = async (event) => {
    if (activePointerId.value !== event.pointerId) return
    if (!pointerDown.value) return

    let finishDrag = null
    if (isDragging.value) {
      cancelScheduledMove()
      const finalDx = pendingDx.value
      const finalDy = pendingDy.value
      finishDrag = enqueueDragCommand(async () => {
        await floatingButtonApi.moveDrag(finalDx, finalDy)
        await floatingButtonApi.endDrag()
      })
    } else {
      playClickSound()
      emit('click')
    }

    pointerDown.value = false
    activePointerId.value = null
    isDragging.value = false
    if (event.currentTarget && event.currentTarget.releasePointerCapture) {
      event.currentTarget.releasePointerCapture(event.pointerId)
    }
    if (finishDrag) {
      await finishDrag
    }
  }

  const handlePointerCancel = (event) => {
    if (activePointerId.value !== null && activePointerId.value !== event.pointerId) return
    if (isDragging.value) {
      cancelScheduledMove()
      enqueueDragCommand(() => floatingButtonApi.endDrag())
    }
    pointerDown.value = false
    activePointerId.value = null
    isDragging.value = false
  }

  return {
    isDragging,
    handlePointerDown,
    handlePointerMove,
    handlePointerUp,
    handlePointerCancel
  }
}
