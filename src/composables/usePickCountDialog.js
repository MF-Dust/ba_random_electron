import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { audioApi } from '../api/audioApi'
import { pickCountApi } from '../api/pickCountApi'
import {
  DEFAULT_BACKGROUND_DARKNESS_PERCENT,
  DEFAULT_PICK_COUNT,
  MAX_PICK_COUNT,
  MIN_PICK_COUNT
} from '../configDefaults'

const MIN_COUNT = MIN_PICK_COUNT
const MAX_COUNT = MAX_PICK_COUNT
const EXIT_ANIMATION_MS = 400

function clampInt(value, min, max, fallback) {
  const n = Number(value)
  if (!Number.isFinite(n)) return fallback
  const rounded = Math.round(n)
  return Math.max(min, Math.min(max, rounded))
}

export function usePickCountDialog() {
  const count = ref(DEFAULT_PICK_COUNT)
  const playMusic = ref(false)
  const isLeaving = ref(false)
  const backgroundDarknessPercent = ref(DEFAULT_BACKGROUND_DARKNESS_PERCENT)
  const isDialogOpen = ref(false)
  const isInitializing = ref(false)

  let removeOnOpenListener = null
  let removeStopListener = null

  const canDecrease = computed(() => count.value > MIN_COUNT)
  const canIncrease = computed(() => count.value < MAX_COUNT)

  const overlayStyle = computed(() => {
    const darkness = Math.max(0, Math.min(100, backgroundDarknessPercent.value))
    const alpha = darkness / 100

    return {
      backgroundColor: `rgba(0, 0, 0, ${alpha})`
    }
  })

  const applyConfig = (cfg) => {
    if (!cfg) return
    count.value = clampInt(cfg.defaultCount, MIN_COUNT, MAX_COUNT, DEFAULT_PICK_COUNT)
    playMusic.value = Boolean(cfg.defaultPlayMusic)
    backgroundDarknessPercent.value = clampInt(
      cfg.backgroundDarknessPercent,
      0,
      100,
      DEFAULT_BACKGROUND_DARKNESS_PERCENT
    )
  }

  const initConfig = async (configOverride) => {
    isInitializing.value = true
    try {
      applyConfig(configOverride || await pickCountApi.getConfig())
    } finally {
      isInitializing.value = false
    }
  }

  const playClickSound = () => {
    audioApi.playClickSound().catch(() => {})
  }

  const increaseCount = () => {
    if (count.value < MAX_COUNT) {
      playClickSound()
      count.value += 1
    }
  }

  const decreaseCount = () => {
    if (count.value > MIN_COUNT) {
      playClickSound()
      count.value -= 1
    }
  }

  const setMinCount = () => {
    if (count.value !== MIN_COUNT) {
      playClickSound()
      count.value = MIN_COUNT
    }
  }

  const setMaxCount = () => {
    if (count.value !== MAX_COUNT) {
      playClickSound()
      count.value = MAX_COUNT
    }
  }

  const stopAudio = () => {
    audioApi.stopBgm().catch(() => {})
  }

  const playBgm = async () => {
    await audioApi.playBgm()
  }

  const isCurrentWindowVisible = async () => {
    if (!window.__TAURI_INTERNALS__) {
      return false
    }
    try {
      return await getCurrentWindow().isVisible()
    } catch (_error) {
      return false
    }
  }

  const resetDialogStateFromConfig = async (shouldPlayBgm, configOverride) => {
    isLeaving.value = false
    stopAudio()
    await initConfig(configOverride)

    if (shouldPlayBgm && playMusic.value) {
      try {
        await playBgm()
      } catch (error) {
        console.warn('Failed to play bgm on open:', error)
      }
    }
  }

  const beginExit = (action) => {
    if (isLeaving.value) {
      return
    }

    isLeaving.value = true
    isDialogOpen.value = false
    playClickSound()
    window.setTimeout(() => {
      if (action !== 'confirm') {
        stopAudio()
      }

      if (action === 'confirm') {
        pickCountApi.confirm(count.value, playMusic.value)
      } else {
        pickCountApi.cancel()
      }
    }, EXIT_ANIMATION_MS)
  }

  const handleCancel = () => {
    beginExit('cancel')
  }

  const handleConfirm = () => {
    beginExit('confirm')
  }

  watch(playMusic, async (enabled) => {
    if (!isDialogOpen.value || isInitializing.value) {
      return
    }
    if (enabled) {
      try {
        await playBgm()
      } catch (error) {
        console.warn('Failed to play bgm:', error)
      }
    } else {
      stopAudio()
    }
  })

  onMounted(async () => {
    isLeaving.value = false
    stopAudio()
    let openedByEvent = false

    removeOnOpenListener = pickCountApi.onOpen(async (payload) => {
      openedByEvent = true
      isDialogOpen.value = true
      await resetDialogStateFromConfig(true, payload?.config)
    })

    removeStopListener = pickCountApi.onStopBgm(() => {
      stopAudio()
    })

    await initConfig()
    if (!openedByEvent && await isCurrentWindowVisible()) {
      isDialogOpen.value = true
      await resetDialogStateFromConfig(true)
    }
  })

  onBeforeUnmount(() => {
    stopAudio()
    if (typeof removeOnOpenListener === 'function') {
      removeOnOpenListener()
    }
    if (typeof removeStopListener === 'function') {
      removeStopListener()
    }
  })

  return {
    MIN_COUNT,
    MAX_COUNT,
    count,
    playMusic,
    isLeaving,
    canDecrease,
    canIncrease,
    overlayStyle,
    increaseCount,
    decreaseCount,
    setMinCount,
    setMaxCount,
    handleCancel,
    handleConfirm
  }
}
