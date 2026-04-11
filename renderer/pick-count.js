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
    const BGM_GAIN = 0.3;
    const CLICK_SOUND_GAIN = 1;
    const audioContext = new (window.AudioContext || window.webkitAudioContext)();
    const bgmBufferPromise = fetch('../sound/bgm.mp3')
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => audioContext.decodeAudioData(arrayBuffer.slice(0)));
    const clickBufferPromise = fetch('../sound/button_click.wav')
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => audioContext.decodeAudioData(arrayBuffer.slice(0)));
    const EXIT_ANIMATION_MS = 400;
    let removeOnOpenListener = null;
    let bgmSource = null;
    let bgmGainNode = null;

    const canDecrease = computed(() => count.value > 1);
    const canIncrease = computed(() => count.value < 10);

    const overlayStyle = computed(() => {
      const darkness = Math.max(0, Math.min(100, backgroundDarknessPercent.value));
      const alpha = darkness / 100;

      return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`
      };
    });

    async function initConfig() {
      if (!window.pickCountApi) return;
      const cfg = await window.pickCountApi.getConfig();

      count.value = clampInt(cfg.defaultCount, 1, 10, 1);
      playMusic.value = Boolean(cfg.defaultPlayMusic);
      backgroundDarknessPercent.value = clampInt(cfg.backgroundDarknessPercent, 0, 100, 50);

      if (bgmGainNode) {
        bgmGainNode.gain.value = BGM_GAIN;
      }
    }

    function increaseCount() {
      if (count.value < 10) {
        playClickSound();
        count.value += 1;
      }
    }

    function decreaseCount() {
      if (count.value > 1) {
        playClickSound();
        count.value -= 1;
      }
    }

    function stopAudio() {
      if (bgmSource) {
        try {
          bgmSource.stop(0);
        } catch (_error) {
        }
        bgmSource.disconnect();
        bgmSource = null;
      }

      if (bgmGainNode) {
        bgmGainNode.disconnect();
        bgmGainNode = null;
      }
    }

    async function playBgm() {
      stopAudio();

      const buffer = await bgmBufferPromise;
      if (audioContext.state === 'suspended') {
        await audioContext.resume();
      }

      const source = audioContext.createBufferSource();
      source.buffer = buffer;
      source.loop = true;

      const gainNode = audioContext.createGain();
      gainNode.gain.value = BGM_GAIN;

      source.connect(gainNode);
      gainNode.connect(audioContext.destination);
      source.start(0);

      bgmSource = source;
      bgmGainNode = gainNode;
    }

    async function resetDialogStateFromConfig() {
      isLeaving.value = false;
      stopAudio();
      await initConfig();

      if (playMusic.value) {
        try {
          await playBgm();
        } catch (error) {
          console.warn('Failed to play bgm on open:', error);
        }
      }
    }

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

    function beginExit(action) {
      if (isLeaving.value) {
        return;
      }

      isLeaving.value = true;
      playClickSound();
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
          await playBgm();
        } catch (error) {
          console.warn('Failed to play bgm:', error);
        }
      } else {
        stopAudio();
      }
    });

    onMounted(async () => {
      await resetDialogStateFromConfig();

      if (window.pickCountApi && typeof window.pickCountApi.onOpen === 'function') {
        removeOnOpenListener = window.pickCountApi.onOpen(async () => {
          await resetDialogStateFromConfig();
        });
      }
    });

    onBeforeUnmount(() => {
      stopAudio();
      if (typeof removeOnOpenListener === 'function') {
        removeOnOpenListener();
      }
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
