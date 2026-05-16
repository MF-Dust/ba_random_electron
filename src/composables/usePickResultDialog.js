import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import { audioApi } from '../api/audioApi'
import { pickResultApi } from '../api/pickResultApi'

const pityCounter = ref(0)

export function usePickResultDialog() {
  const results = ref([])
  const animationKey = ref(0)
  const instructionText = ref('点击任意位置关闭')
  const revealStarted = ref(false)
  const canClose = ref(false)
  const isClosing = ref(false)
  const lastToken = ref(0)
  const playGachaSound = ref(true)
  const gachaSoundVolume = ref(0.6)

  let revealTimer = null
  let closeTimer = null
  let closeFadeTimer = null
  let removeOpenListener = null
  let removeResetListener = null

  const topRow = computed(() => results.value.slice(0, 5))
  const bottomRow = computed(() => results.value.slice(5))
  const isTwoRows = computed(() => results.value.length > 5)

  const normalizeResults = (payload) => {
    const list = Array.isArray(payload?.results) ? payload.results : payload
    if (!Array.isArray(list)) return []
    return list
      .map((item) => {
        if (!item) return null
        const name = typeof item === 'string' ? item.trim() : String(item.name || '').trim()
        if (!name) return null

        pityCounter.value++
        const isPityDraw = pityCounter.value % 10 === 0

        const rand = Math.random()
        let rarity = 'blue'
        if (rand > 0.97) {
          rarity = 'pink'
        } else if (rand > 0.785) {
          rarity = 'gold'
        }

        // 10th draw guarantee (pity): must be gold or pink
        if (isPityDraw && rarity === 'blue') {
          // 5% chance to "upgrade" pity to pink, otherwise gold
          rarity = Math.random() > 0.95 ? 'pink' : 'gold'
        }

        return { name, rarity }
      })
      .filter((item) => item)
  }

  const stopGachaLoadingSound = () => {
    audioApi.stopGachaSound().catch(() => {})
  }

  const playGachaLoadingSound = async () => {
    await audioApi.playGachaSound(Math.max(0, Math.min(1, Number(gachaSoundVolume.value) || 0)))
  }

  const resetResultState = ({ stopSound = true } = {}) => {
    results.value = []
    animationKey.value += 1
    revealStarted.value = false
    canClose.value = false
    isClosing.value = false
    if (revealTimer) {
      clearTimeout(revealTimer)
      revealTimer = null
    }
    if (closeTimer) {
      clearTimeout(closeTimer)
      closeTimer = null
    }
    if (closeFadeTimer) {
      clearTimeout(closeFadeTimer)
      closeFadeTimer = null
    }
    if (stopSound) {
      stopGachaLoadingSound()
    }
  }

  const applySoundConfig = (cfg) => {
    if (!cfg) return
    playGachaSound.value = Boolean(cfg.defaultPlayGachaSound)
    gachaSoundVolume.value = Number(cfg.gachaSoundVolume)
  }

  const applyResults = (payload) => {
    resetResultState({ stopSound: false })
    applySoundConfig(payload?.config)
    results.value = normalizeResults(payload)
    const token = Number(payload?.token)
    if (Number.isFinite(token)) {
      lastToken.value = token
    }

    if (results.value.length === 0) {
      canClose.value = true
      return
    }

    const totalDelayMs = (Math.max(results.value.length - 1, 0) * 120) + 600
    revealTimer = setTimeout(() => {
      revealStarted.value = true
    }, totalDelayMs)

    const totalDurationMs = totalDelayMs + 450
    closeTimer = setTimeout(() => {
      canClose.value = true
    }, totalDurationMs)

    if (playGachaSound.value) {
      playGachaLoadingSound()
    }
  }

  const handleReset = (payload) => {
    const token = Number(payload?.token)
    const reason = payload?.reason
    if (Number.isFinite(token)) {
      if (reason === 'before-open' && token === lastToken.value) {
        return
      }
      if (token < lastToken.value) {
        return
      }
      lastToken.value = token
    }
    resetResultState()
  }

  const closeResult = async () => {
    if (isClosing.value) return
    isClosing.value = true
    closeFadeTimer = setTimeout(async () => {
      resetResultState()
      await nextTick()
      await new Promise((resolve) => window.requestAnimationFrame(resolve))
      pickResultApi.close()
    }, 220)
  }

  const handleStageClick = () => {
    if (!canClose.value || isClosing.value) return
    closeResult()
  }

  const handleKeydown = (event) => {
    if (event.key === 'Escape' && canClose.value && !isClosing.value) {
      closeResult()
    }
  }

  const loadSoundConfig = async (configOverride) => {
    applySoundConfig(configOverride || await pickResultApi.getConfig())
  }

  onMounted(async () => {
    await loadSoundConfig()

    const initial = await pickResultApi.getResults()
    applyResults({ results: initial })

    removeOpenListener = pickResultApi.onOpen(async (payload) => {
      await loadSoundConfig(payload?.config)
      applyResults(payload)
    })

    removeResetListener = pickResultApi.onReset((payload) => {
      handleReset(payload)
    })
  })

  onBeforeUnmount(() => {
    resetResultState()
    if (typeof removeOpenListener === 'function') {
      removeOpenListener()
    }
    if (typeof removeResetListener === 'function') {
      removeResetListener()
    }
  })

  return {
    results,
    animationKey,
    instructionText,
    revealStarted,
    isClosing,
    topRow,
    bottomRow,
    isTwoRows,
    handleStageClick,
    handleKeydown
  }
}
