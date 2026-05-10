export const DEFAULT_ADMIN_TASK_NAME = 'KVRandom (Admin)'

export const MIN_PICK_COUNT = 1
export const MAX_PICK_COUNT = 10

export const DEFAULT_FLOATING_SIZE_PERCENT = 100
export const DEFAULT_FLOATING_TRANSPARENCY_PERCENT = 20
export const DEFAULT_BACKGROUND_DARKNESS_PERCENT = 50
export const DEFAULT_PICK_COUNT = 1
export const DEFAULT_PLAY_MUSIC = false
export const DEFAULT_PLAY_GACHA_SOUND = true
export const DEFAULT_GACHA_SOUND_VOLUME = 0.6
export const DEFAULT_WEB_CONFIG_PORT = 21219

export const createDefaultConfig = () => ({
  studentList: [],
  allowRepeatDraw: true,
  floatingButton: {
    sizePercent: DEFAULT_FLOATING_SIZE_PERCENT,
    transparencyPercent: DEFAULT_FLOATING_TRANSPARENCY_PERCENT,
    alwaysOnTop: true,
    position: {
      x: null,
      y: null
    }
  },
  pickCountDialog: {
    defaultPlayMusic: DEFAULT_PLAY_MUSIC,
    backgroundDarknessPercent: DEFAULT_BACKGROUND_DARKNESS_PERCENT,
    defaultCount: DEFAULT_PICK_COUNT
  },
  pickResultDialog: {
    defaultPlayGachaSound: DEFAULT_PLAY_GACHA_SOUND,
    gachaSoundVolume: DEFAULT_GACHA_SOUND_VOLUME
  },
  webConfig: {
    port: DEFAULT_WEB_CONFIG_PORT,
    adminTopmostEnabled: false,
    adminAutoStartEnabled: false,
    adminAutoStartPath: '',
    adminAutoStartTaskName: DEFAULT_ADMIN_TASK_NAME
  }
})
