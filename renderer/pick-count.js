const { createApp, ref, computed, onMounted, onBeforeUnmount, watch } = Vue;

function clampInt(value, min, max, fallback) {
  const n = Number(value);
  if (!Number.isFinite(n)) return fallback;
  const rounded = Math.round(n);
  return Math.max(min, Math.min(max, rounded));
}

createApp({
  setup() {
    const count = ref(1);
    const playMusic = ref(false);
    const isLeaving = ref(false);
    const backgroundDarknessPercent = ref(50);
    const backgroundBlurPercent = ref(10);
    const audio = new Audio('../sound/bgm.mp3');
    const EXIT_ANIMATION_MS = 400;

    audio.loop = true;
    audio.volume = 0.3;

    const canDecrease = computed(() => count.value > 1);
    const canIncrease = computed(() => count.value < 10);

    const overlayStyle = computed(() => {
      const darkness = Math.max(0, Math.min(100, backgroundDarknessPercent.value));
      const blur = Math.max(0, Math.min(100, backgroundBlurPercent.value));
      const alpha = darkness / 100;
      const blurPx = (blur / 100) * 24;

      return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        backdropFilter: `blur(${blurPx}px)`
      };
    });

    async function initConfig() {
      if (!window.pickCountApi) return;
      const cfg = await window.pickCountApi.getConfig();

      count.value = clampInt(cfg.defaultCount, 1, 10, 1);
      playMusic.value = Boolean(cfg.defaultPlayMusic);
      backgroundDarknessPercent.value = clampInt(cfg.backgroundDarknessPercent, 0, 100, 50);
      backgroundBlurPercent.value = clampInt(cfg.backgroundBlurPercent, 0, 100, 10);
    }

    function increaseCount() {
      if (count.value < 10) {
        count.value += 1;
      }
    }

    function decreaseCount() {
      if (count.value > 1) {
        count.value -= 1;
      }
    }

    function stopAudio() {
      audio.pause();
      audio.currentTime = 0;
    }

    function beginExit(action) {
      if (isLeaving.value) {
        return;
      }

      isLeaving.value = true;
      window.setTimeout(() => {
        stopAudio();
        if (!window.pickCountApi) {
          return;
        }

        if (action === 'confirm') {
          window.pickCountApi.confirm(count.value, playMusic.value);
        } else {
          window.pickCountApi.cancel();
        }
      }, EXIT_ANIMATION_MS);
    }

    function handleCancel() {
      beginExit('cancel');
    }

    function handleConfirm() {
      beginExit('confirm');
    }

    watch(playMusic, async (enabled) => {
      if (enabled) {
        try {
          audio.currentTime = 0;
          await audio.play();
        } catch (error) {
          console.warn('Failed to play bgm:', error);
        }
      } else {
        stopAudio();
      }
    });

    onMounted(async () => {
      await initConfig();
      if (playMusic.value) {
        try {
          audio.currentTime = 0;
          await audio.play();
        } catch (error) {
          console.warn('Failed to play bgm on mount:', error);
        }
      }
    });

    onBeforeUnmount(() => {
      stopAudio();
    });

    return {
      count,
      playMusic,
      canDecrease,
      canIncrease,
      isLeaving,
      overlayStyle,
      increaseCount,
      decreaseCount,
      handleCancel,
      handleConfirm
    };
  },
  template: `
    <div class="pick-overlay" :class="{ 'is-leaving': isLeaving }" :style="overlayStyle">
      <div class="pick-panel">
        <h1 class="pick-title">选择人数</h1>

        <div class="pick-counter-row">
          <button class="pick-circle-btn" :disabled="isLeaving || !canDecrease" @click="decreaseCount" aria-label="减少人数">-</button>
          <div class="pick-count-box">{{ count }}</div>
          <button class="pick-circle-btn" :disabled="isLeaving || !canIncrease" @click="increaseCount" aria-label="增加人数">+</button>
        </div>

        <div class="pick-actions">
          <button class="pick-action-btn pick-action-cancel" :disabled="isLeaving" @click="handleCancel">取消</button>
          <button class="pick-action-btn pick-action-confirm" :disabled="isLeaving" @click="handleConfirm">确定</button>
        </div>

        <label class="pick-music-row">
          <input class="pick-checkbox" type="checkbox" v-model="playMusic" :disabled="isLeaving" />
          <span>播放 喜 庆 的 点名音乐</span>
        </label>
      </div>
    </div>
  `
}).mount('#app');
