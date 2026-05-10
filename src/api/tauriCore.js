import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export { invoke }

export const listenCompat = (eventName, callback) => {
  let unlisten = null
  let disposed = false

  listen(eventName, (event) => {
    callback(event.payload)
  }).then((fn) => {
    if (disposed) {
      fn()
      return
    }
    unlisten = fn
  }).catch((error) => {
    console.warn(`Failed to listen ${eventName}`, error)
  })

  return () => {
    disposed = true
    if (typeof unlisten === 'function') {
      unlisten()
    }
  }
}
