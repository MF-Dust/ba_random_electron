const FloatingButton = {
  name: 'FloatingButton',
  props: {
    sizePx: {
      type: Number,
      required: true
    },
    transparencyPercent: {
      type: Number,
      required: true
    }
  },
  emits: ['click'],
  setup(props, { emit }) {
    const { computed, ref } = Vue;
    const CLICK_SOUND_GAIN = 1;
    const audioContext = new (window.AudioContext || window.webkitAudioContext)();
    const clickBufferPromise = fetch('../sound/button_click.wav')
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => audioContext.decodeAudioData(arrayBuffer.slice(0)));

    function playClickSound() {
      clickBufferPromise
        .then(async (buffer) => {
          if (audioContext.state === 'suspended') {
            await audioContext.resume();
          }

          const source = audioContext.createBufferSource();
          source.buffer = buffer;

          const gainNode = audioContext.createGain();
          gainNode.gain.value = CLICK_SOUND_GAIN;

          source.connect(gainNode);
          gainNode.connect(audioContext.destination);
          source.start(0);
        })
        .catch(() => {});
    }

    const styleOpacity = computed(() => {
      return Math.max(0, Math.min(1, 1 - props.transparencyPercent / 100));
    });

    const buttonStyle = computed(() => {
      return {
        width: `${props.sizePx}px`,
        height: `${props.sizePx}px`,
        opacity: String(styleOpacity.value)
      };
    });

    const textStyle = computed(() => {
      return {
        opacity: String(styleOpacity.value)
      };
    });

    const pointerDown = ref(false);
    const activePointerId = ref(null);
    const isDragging = ref(false);
    const startGlobalX = ref(0);
    const startGlobalY = ref(0);
    const pendingDx = ref(0);
    const pendingDy = ref(0);
    const rafId = ref(0);
    const DRAG_THRESHOLD_PX = 3;

    function getGlobalPoint(event) {
      const fallbackX = window.screenX + event.clientX;
      const fallbackY = window.screenY + event.clientY;

      if (event.pointerType === 'touch') {
        return { x: fallbackX, y: fallbackY };
      }

      const screenX = Number(event.screenX);
      const screenY = Number(event.screenY);
      return {
        x: Number.isFinite(screenX) ? screenX : fallbackX,
        y: Number.isFinite(screenY) ? screenY : fallbackY
      };
    }

    function flushMove() {
      if (!isDragging.value || !window.floatingButtonApi) {
        rafId.value = 0;
        return;
      }
      window.floatingButtonApi.moveDrag(pendingDx.value, pendingDy.value);
      rafId.value = 0;
    }

    function scheduleMove() {
      if (rafId.value !== 0) return;
      rafId.value = window.requestAnimationFrame(flushMove);
    }

    function cancelScheduledMove() {
      if (rafId.value !== 0) {
        window.cancelAnimationFrame(rafId.value);
        rafId.value = 0;
      }
    }

    function handlePointerDown(event) {
      if (event.pointerType === 'mouse' && event.button !== 0) return;
      pointerDown.value = true;
      activePointerId.value = event.pointerId;
      isDragging.value = false;
      const point = getGlobalPoint(event);
      startGlobalX.value = point.x;
      startGlobalY.value = point.y;
      pendingDx.value = 0;
      pendingDy.value = 0;
      cancelScheduledMove();
      if (event.currentTarget && event.currentTarget.setPointerCapture) {
        event.currentTarget.setPointerCapture(event.pointerId);
      }
    }

    function handlePointerMove(event) {
      if (activePointerId.value !== event.pointerId) return;
      if (!pointerDown.value || !window.floatingButtonApi) return;

      const point = getGlobalPoint(event);
      const dx = point.x - startGlobalX.value;
      const dy = point.y - startGlobalY.value;
      const movedEnough = Math.abs(dx) >= DRAG_THRESHOLD_PX || Math.abs(dy) >= DRAG_THRESHOLD_PX;

      if (!isDragging.value && movedEnough) {
        isDragging.value = true;
        window.floatingButtonApi.startDrag();
      }

      if (isDragging.value) {
        pendingDx.value = dx;
        pendingDy.value = dy;
        scheduleMove();
      }
    }

    function handlePointerUp(event) {
      if (activePointerId.value !== event.pointerId) return;
      if (!pointerDown.value) return;

      if (isDragging.value) {
        if (window.floatingButtonApi) {
          cancelScheduledMove();
          window.floatingButtonApi.moveDrag(pendingDx.value, pendingDy.value);
          window.floatingButtonApi.endDrag();
        }
      } else {
        playClickSound();
        emit('click');
      }

      pointerDown.value = false;
      activePointerId.value = null;
      isDragging.value = false;
      if (event.currentTarget && event.currentTarget.releasePointerCapture) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
    }

    function handlePointerCancel(event) {
      if (activePointerId.value !== null && activePointerId.value !== event.pointerId) return;
      if (isDragging.value && window.floatingButtonApi) {
        cancelScheduledMove();
        window.floatingButtonApi.endDrag();
      }
      pointerDown.value = false;
      activePointerId.value = null;
      isDragging.value = false;
    }

    return {
      buttonStyle,
      textStyle,
      handlePointerDown,
      handlePointerMove,
      handlePointerUp,
      handlePointerCancel,
      isDragging
    };
  },
  template: `
    <div class="floating-root">
      <button
        class="floating-button"
        :class="{ 'is-dragging': isDragging }"
        :style="buttonStyle"
        @contextmenu.prevent
        @pointerdown="handlePointerDown"
        @pointermove="handlePointerMove"
        @pointerup="handlePointerUp"
        @pointercancel="handlePointerCancel"
        title="抽取"
      >
        <img src="../image/random.svg" alt="随机抽取" draggable="false" />
        <span class="floating-button-label" :style="textStyle">抽取</span>
      </button>
    </div>
  `
};
